//! Linux'ta uygulamalar: sistem paketi (.deb / .rpm) ya da AppImage.
//!
//! Paketler: kurulu sürüm paket veritabanından okunur; dpkg'nin durum dosyası doğrudan, rpm ise `rpm -q` ile
//! (veritabanı değişmedikçe yeniden sorulmaz; 5 sn'lik denetim ucuz kalır). Kurma ve kaldırma sistemin paket
//! yöneticisine (apt, dnf, zypper ya da rpm) yaptırılır; yönetici yetkisini pkexec ister: masaüstünün parola
//! penceresi açılır, Robz Hub parolayı görmez.
//!
//! AppImage: her dağıtımda (Arch, Manjaro…) çalışır. Kullanıcının kendi klasörüne kurulur, parola istemez:
//! ~/.local/share/robz-hub/apps/<kimlik>/<kimlik>.AppImage, yanında sürümü yazan "version" dosyası ve uygulama
//! menüsü için ~/.local/share/applications/robz-hub-<kimlik>.desktop.
//!
//! Kod her sistemde derlenir (testler Windows'ta da çalışır); yalnız Linux'ta kullanılır.
#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

use crate::catalog::AppDef;
use crate::error::{HubError, HubResult};
use crate::installs::InstallInfo;
use crate::platform::{linux_format, LinuxFormat};
use crate::version;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

const DPKG_STATUS: &str = "/var/lib/dpkg/status";
const RPM_DBS: [&str; 2] = ["/var/lib/rpm", "/usr/lib/sysimage/rpm"];

/// PATH içinde (ya da standart yerlerde) bir program. AppImage olarak çalışırken Robz Hub'ın kendi paketindeki
/// klasörler atlanır: aranan, sistemin programıdır.
pub fn find_program(name: &str) -> Option<PathBuf> {
    let appdir = std::env::var_os("APPDIR").filter(|d| !d.is_empty()).map(PathBuf::from);
    let dirs = std::env::var_os("PATH").map(|p| std::env::split_paths(&p).collect::<Vec<_>>()).unwrap_or_default();
    dirs.into_iter()
        .filter(|dir| appdir.as_ref().is_none_or(|appdir| !dir.starts_with(appdir)))
        .chain(["/usr/bin", "/usr/local/bin", "/usr/sbin", "/bin", "/sbin"].map(PathBuf::from))
        .map(|dir| dir.join(name))
        .find(|p| p.is_file())
}

/// Bir klasörü ya da bağlantıyı sistemin varsayılan programıyla açar (dosya yöneticisi, tarayıcı). Program Robz
/// Hub'ın temizlenmiş ortamıyla başlar; yoksa AppImage'ın kitaplıklarını yüklemeye çalışıp açılmaz.
pub fn open_with_system(target: &str) -> HubResult<()> {
    let (program, args): (PathBuf, &[&str]) = if let Some(xdg) = find_program("xdg-open") {
        (xdg, &[])
    } else if let Some(gio) = find_program("gio") {
        (gio, &["open"])
    } else {
        return Err(HubError::with("open-failed", "xdg-open bulunamadı"));
    };
    let mut command = std::process::Command::new(program);
    command.args(args).arg(target);
    command.stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null());
    crate::platform::clean_env(&mut command);
    let mut child = command.spawn().map_err(|e| HubError::with("open-failed", e))?;
    // xdg-open işini yapıp çıkar; zombi kalmasın diye beklenir. Açamadıysa (ör. varsayılan dosya yöneticisi yok)
    // nedeni günlüğe yazılır.
    let target = target.to_string();
    std::thread::spawn(move || match child.wait() {
        Ok(status) if !status.success() => crate::log::write(format!("açılamadı ({status}): {target}")),
        Err(error) => crate::log::write(format!("açılamadı ({error}): {target}")),
        Ok(_) => {}
    });
    Ok(())
}

/// "1:0.2.0-1" -> "0.2.0": dönem (epoch) ve paket revizyonu atılır.
pub fn package_version(raw: &str) -> Option<String> {
    let text = raw.trim();
    let text = text.split_once(':').map_or(text, |(_, rest)| rest);
    let text = text.split_once('-').map_or(text, |(upstream, _)| upstream);
    version::clean(text)
}

// ---- kurulu sürüm -----------------------------------------------------------------------------------------------------

/// dpkg durum dosyasından kurulu paketler: ad -> sürüm. Dosya değişmedikçe yeniden okunmaz.
fn dpkg_installed() -> HashMap<String, String> {
    static CACHE: Mutex<Option<(SystemTime, HashMap<String, String>)>> = Mutex::new(None);
    let modified = std::fs::metadata(DPKG_STATUS).and_then(|m| m.modified()).ok();
    let mut cache = CACHE.lock().unwrap_or_else(|p| p.into_inner());
    if let (Some(modified), Some((at, map))) = (modified, cache.as_ref()) {
        if *at == modified {
            return map.clone();
        }
    }
    let map = std::fs::read_to_string(DPKG_STATUS).map(|text| parse_dpkg_status(&text)).unwrap_or_default();
    if let Some(modified) = modified {
        *cache = Some((modified, map.clone()));
    }
    map
}

pub fn parse_dpkg_status(text: &str) -> HashMap<String, String> {
    let mut installed = HashMap::new();
    for paragraph in text.split("\n\n") {
        let (mut name, mut status, mut ver) = (None, None, None);
        for line in paragraph.lines() {
            if let Some(v) = line.strip_prefix("Package: ") {
                name = Some(v.trim());
            } else if let Some(v) = line.strip_prefix("Status: ") {
                status = Some(v.trim());
            } else if let Some(v) = line.strip_prefix("Version: ") {
                ver = Some(v.trim());
            }
        }
        if let (Some(name), Some("install ok installed"), Some(ver)) = (name, status, ver) {
            installed.insert(name.to_string(), ver.to_string());
        }
    }
    installed
}

/// Klasörün ve içindeki dosyaların en yeni değişme zamanı. rpm'in sqlite veritabanı (Fedora, openSUSE) yerinde
/// yazılır: paket kurulunca dosyaların zamanı değişir, klasörünki değişmez; yalnız klasöre bakılırsa kurulum görülmez.
fn newest_in(dir: &Path) -> Option<SystemTime> {
    let modified = |p: &Path| std::fs::metadata(p).and_then(|m| m.modified()).ok();
    let files = std::fs::read_dir(dir).into_iter().flatten().flatten().filter_map(|e| modified(&e.path()));
    modified(dir).into_iter().chain(files).max()
}

fn rpm_db_modified() -> Option<SystemTime> {
    RPM_DBS.iter().filter_map(|p| newest_in(Path::new(p))).max()
}

/// `rpm -q` sonucu; rpm veritabanı değişmedikçe aynı paket yeniden sorulmaz.
fn rpm_version(package: &str) -> Option<String> {
    static CACHE: Mutex<Option<(Option<SystemTime>, HashMap<String, Option<String>>)>> = Mutex::new(None);
    let modified = rpm_db_modified();
    {
        let mut cache = CACHE.lock().unwrap_or_else(|p| p.into_inner());
        match cache.as_ref() {
            Some((at, map)) if *at == modified && modified.is_some() => {
                if let Some(found) = map.get(package) {
                    return found.clone();
                }
            }
            _ => *cache = Some((modified, HashMap::new())),
        }
    }
    let mut command = std::process::Command::new(find_program("rpm").unwrap_or_else(|| PathBuf::from("rpm")));
    crate::platform::clean_env(&mut command);
    let output = command.args(["-q", "--qf", "%{VERSION}", package]).output().ok();
    let found = output.filter(|o| o.status.success()).map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    let found = found.filter(|v| !v.is_empty());
    if let Some((_, map)) = CACHE.lock().unwrap_or_else(|p| p.into_inner()).as_mut() {
        map.insert(package.to_string(), found.clone());
    }
    found
}

fn installed_version(package: &str) -> Option<String> {
    match linux_format()? {
        LinuxFormat::Deb => dpkg_installed().remove(package),
        LinuxFormat::Rpm => rpm_version(package),
    }
}

/// Kurulu mu: önce sistem paketi, yoksa Robz Hub'ın kurduğu AppImage.
pub fn detect(def: &AppDef) -> InstallInfo {
    if let Some(linux) = &def.linux {
        if let Some(raw) = linux.package.as_deref().and_then(installed_version) {
            let exe = linux.exe.as_deref().and_then(find_program);
            return InstallInfo {
                installed: true,
                version: package_version(&raw),
                dir: exe.as_deref().and_then(Path::parent).map(|d| d.display().to_string()),
                exe_path: exe.map(|p| p.display().to_string()),
                uninstaller: None,
                scope: Some("machine"),
                source: Some("package"),
            };
        }
    }
    detect_appimage(&def.id)
}

// ---- AppImage ---------------------------------------------------------------------------------------------------------

/// Kullanıcının veri klasörü: $XDG_DATA_HOME ya da ~/.local/share
fn data_home() -> Option<PathBuf> {
    std::env::var_os("XDG_DATA_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").filter(|v| !v.is_empty()).map(|h| PathBuf::from(h).join(".local/share")))
}

pub fn appimage_dir(id: &str) -> Option<PathBuf> {
    data_home().map(|d| d.join("robz-hub").join("apps").join(id))
}

fn appimage_file(id: &str) -> Option<PathBuf> {
    appimage_dir(id).map(|d| d.join(format!("{id}.AppImage")))
}

fn desktop_file(id: &str) -> Option<PathBuf> {
    data_home().map(|d| d.join("applications").join(format!("robz-hub-{id}.desktop")))
}

pub fn detect_appimage(id: &str) -> InstallInfo {
    let (Some(dir), Some(file)) = (appimage_dir(id), appimage_file(id)) else { return InstallInfo::default() };
    if !file.is_file() {
        return InstallInfo::default();
    }
    let version = std::fs::read_to_string(dir.join("version")).ok().and_then(|v| version::clean(v.trim()));
    InstallInfo {
        installed: true,
        version,
        dir: Some(dir.display().to_string()),
        exe_path: Some(file.display().to_string()),
        uninstaller: None,
        scope: Some("user"),
        source: Some("appimage"),
    }
}

/// .desktop dosyasında tırnak içinde özel anlamı olan karakterler kaçırılır (masaüstü giriş belirtimi).
fn desktop_quote(path: &Path) -> String {
    let mut out = String::from('"');
    for c in path.display().to_string().chars() {
        if matches!(c, '"' | '`' | '$' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

/// AppImageLauncher kuruluysa (Arch'ta yaygın) başlatılan her AppImage'ın önüne geçip "tümleştir" diye sorar; kabul
/// edilirse dosyayı ~/Applications'a taşır ve Robz Hub kurduğu uygulamayı yerinde bulamaz ("Kur" düğmesi geri gelir).
/// Bu ortam değişkeniyle araya girmez; AppImageLauncher yoksa etkisi olmaz.
pub const NO_APPIMAGELAUNCHER: (&str, &str) = ("APPIMAGELAUNCHER_DISABLE", "1");

pub fn desktop_entry(def: &AppDef, exe: &Path, icon: Option<&Path>) -> String {
    let line = |s: &str| s.replace(['\n', '\r'], " ");
    let mut text = format!(
        "[Desktop Entry]\nType=Application\nName={}\nComment={}\nComment[tr]={}\nExec=env {}={} {} %U\nTerminal=false\nCategories=Utility;\nX-Robz-Hub-App={}\n",
        line(&def.name),
        line(&def.tagline.en),
        line(&def.tagline.tr),
        NO_APPIMAGELAUNCHER.0,
        NO_APPIMAGELAUNCHER.1,
        desktop_quote(exe),
        def.id,
    );
    if let Some(icon) = icon {
        text.push_str(&format!("Icon={}\n", icon.display()));
    }
    text
}

/// İndirilen AppImage'ı kurar ya da günceller: dosya yenisiyle değiştirilir, çalıştırılabilir yapılır, sürüm ve uygulama
/// menüsü girişi yazılır. `icon`: uygulamanın simgesi (dosya içeriği ve uzantısı), varsa.
pub fn install_appimage(def: &AppDef, file: &Path, version: &str, icon: Option<(Vec<u8>, &str)>) -> HubResult<()> {
    let io = |e: std::io::Error| HubError::with("installer-failed", e);
    let dir = appimage_dir(&def.id).ok_or_else(|| HubError::with("installer-failed", "HOME yok"))?;
    std::fs::create_dir_all(&dir).map_err(io)?;
    let target = dir.join(format!("{}.AppImage", def.id));
    let temp = dir.join(format!("{}.AppImage.new", def.id));
    std::fs::copy(file, &temp).map_err(io)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&temp, std::fs::Permissions::from_mode(0o755)).map_err(io)?;
    }
    std::fs::rename(&temp, &target).map_err(io)?;
    std::fs::write(dir.join("version"), version).map_err(io)?;

    let icon_path = icon.and_then(|(bytes, ext)| {
        let path = dir.join(format!("icon.{ext}"));
        std::fs::write(&path, bytes).ok().map(|_| path)
    });
    if let Some(desktop) = desktop_file(&def.id) {
        if let Some(parent) = desktop.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        // Menü girişi yazılamazsa da uygulama kuruludur; Robz Hub'dan başlatılabilir.
        let _ = std::fs::write(&desktop, desktop_entry(def, &target, icon_path.as_deref()));
    }
    Ok(())
}

pub fn uninstall_appimage(def: &AppDef) -> HubResult<()> {
    if let Some(desktop) = desktop_file(&def.id) {
        let _ = std::fs::remove_file(desktop);
    }
    let dir = appimage_dir(&def.id).ok_or_else(|| HubError::new("not-installed"))?;
    std::fs::remove_dir_all(&dir).map_err(|e| HubError::with("uninstall-failed", e))
}

// ---- kurma ve kaldırma ------------------------------------------------------------------------------------------------

/// Paket yöneticisi komutu (yönetici yetkisiyle çalışacak program ve argümanları).
fn install_command(file: &Path) -> HubResult<(PathBuf, Vec<String>)> {
    let file = file.display().to_string();
    let found = |name: &str| find_program(name);
    let command = match linux_format().ok_or_else(|| HubError::new("unsupported"))? {
        LinuxFormat::Deb => match found("apt-get") {
            // apt bağımlılıkları da kurar; dosya yolu tam verilmeli.
            Some(apt) => (apt, vec!["install".into(), "-y".into(), "--allow-downgrades".into(), file]),
            None => (found("dpkg").ok_or_else(|| HubError::new("unsupported"))?, vec!["-i".into(), file]),
        },
        LinuxFormat::Rpm => {
            if let Some(dnf) = found("dnf") {
                (dnf, vec!["install".into(), "-y".into(), file])
            } else if let Some(zypper) = found("zypper") {
                (zypper, vec!["--non-interactive".into(), "install".into(), "--allow-unsigned-rpm".into(), file])
            } else {
                (found("rpm").ok_or_else(|| HubError::new("unsupported"))?, vec!["-U".into(), "--replacepkgs".into(), file])
            }
        }
    };
    Ok(command)
}

fn remove_command(package: &str) -> HubResult<(PathBuf, Vec<String>)> {
    let package = package.to_string();
    let found = |name: &str| find_program(name);
    let command = match linux_format().ok_or_else(|| HubError::new("unsupported"))? {
        LinuxFormat::Deb => match found("apt-get") {
            Some(apt) => (apt, vec!["remove".into(), "-y".into(), package]),
            None => (found("dpkg").ok_or_else(|| HubError::new("unsupported"))?, vec!["-r".into(), package]),
        },
        LinuxFormat::Rpm => {
            if let Some(dnf) = found("dnf") {
                (dnf, vec!["remove".into(), "-y".into(), package])
            } else if let Some(zypper) = found("zypper") {
                (zypper, vec!["--non-interactive".into(), "remove".into(), package])
            } else {
                (found("rpm").ok_or_else(|| HubError::new("unsupported"))?, vec!["-e".into(), package])
            }
        }
    };
    Ok(command)
}

/// Komutu pkexec ile yönetici olarak çalıştırır. pkexec ortamı temizlediği için apt'nin soru sormaması
/// (DEBIAN_FRONTEND) env ile verilir.
async fn run_as_admin(code: &str, program: PathBuf, args: Vec<String>) -> HubResult<()> {
    let pkexec = find_program("pkexec").ok_or_else(|| HubError::with(code, "pkexec bulunamadı"))?;
    let env = find_program("env").unwrap_or_else(|| PathBuf::from("/usr/bin/env"));
    let mut command = tokio::process::Command::new(pkexec);
    crate::platform::clean_env_async(&mut command);
    command.arg(env).arg("DEBIAN_FRONTEND=noninteractive").arg(program).args(&args).kill_on_drop(true);
    let output = tokio::time::timeout(Duration::from_secs(20 * 60), command.output())
        .await
        .map_err(|_| HubError::with(code, "zaman aşımı"))?
        .map_err(|e| HubError::with(code, e))?;
    match output.status.code() {
        Some(0) => Ok(()),
        // 126: parola penceresi kapatıldı, 127: yetki verilmedi
        Some(126) | Some(127) => Err(HubError::new("auth-cancelled")),
        other => {
            let detail = String::from_utf8_lossy(&output.stderr);
            let last = detail.lines().rev().find(|l| !l.trim().is_empty()).unwrap_or_default().trim().to_string();
            Err(HubError::with(code, format!("çıkış kodu {}: {last}", other.unwrap_or(-1))))
        }
    }
}

pub async fn install(file: &Path) -> HubResult<()> {
    let (program, args) = install_command(file)?;
    run_as_admin("installer-failed", program, args).await
}

pub async fn uninstall(def: &AppDef) -> HubResult<()> {
    let package = def.linux.as_ref().and_then(|l| l.package.as_deref()).ok_or_else(|| HubError::new("unsupported"))?;
    let (program, args) = remove_command(package)?;
    run_as_admin("uninstall-failed", program, args).await?;
    for _ in 0..20 {
        if installed_version(package).is_none() {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    Err(HubError::with("uninstall-failed", "paket hâlâ kurulu"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_installed_packages_from_dpkg_status() {
        let status = "Package: robz-dpi\nStatus: install ok installed\nVersion: 0.2.0\n\n\
                      Package: removed\nStatus: deinstall ok config-files\nVersion: 1.0\n\n\
                      Package: libc6\nStatus: install ok installed\nVersion: 2.36-9+deb12u4\n";
        let installed = parse_dpkg_status(status);
        assert_eq!(installed.get("robz-dpi").map(String::as_str), Some("0.2.0"));
        assert!(!installed.contains_key("removed"));
        assert_eq!(package_version("1:0.2.0-1").as_deref(), Some("0.2.0"));
        assert_eq!(package_version("2.36-9+deb12u4").as_deref(), Some("2.36"));
    }

    // rpm'in sqlite veritabanı gibi: dosya yerinde değişir, klasörün zamanı aynı kalır.
    #[cfg(target_os = "linux")]
    #[test]
    fn notices_database_files_changed_in_place() {
        let dir = std::env::temp_dir().join(format!("robzhub-rpmdb-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let db = dir.join("rpmdb.sqlite");
        std::fs::write(&db, b"v1").unwrap();
        let old = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000);
        std::fs::File::options().write(true).open(&db).unwrap().set_modified(old).unwrap();
        std::fs::File::open(&dir).unwrap().set_modified(old).unwrap();
        assert_eq!(newest_in(&dir), Some(old));

        std::fs::File::options().write(true).open(&db).unwrap().set_modified(old + Duration::from_secs(60)).unwrap();
        assert_eq!(std::fs::metadata(&dir).unwrap().modified().unwrap(), old);
        assert_eq!(newest_in(&dir), Some(old + Duration::from_secs(60)));
        assert_eq!(newest_in(&dir.join("yok")), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn writes_a_menu_entry_for_appimages() {
        let catalog = crate::catalog::parse_str(crate::catalog::BUNDLED).unwrap();
        let def = catalog.app("robz-dpi").unwrap();
        let entry = desktop_entry(def, Path::new("/home/a b/.local/share/robz-hub/apps/robz-dpi/robz-dpi.AppImage"), None);
        assert!(entry.starts_with("[Desktop Entry]\nType=Application\nName=Robz DPI\n"));
        assert!(entry.contains("Exec=env APPIMAGELAUNCHER_DISABLE=1 \"/home/a b/.local/share/robz-hub/apps/robz-dpi/robz-dpi.AppImage\" %U\n"));
        assert!(!entry.contains("Icon="));
        assert_eq!(desktop_quote(Path::new("/x/$a\"b")), "\"/x/\\$a\\\"b\"");
    }

    // Gerçek dosya işlemleri: kurulum, sürüm, güncelleme, kaldırma (Linux'ta; ev klasörü geçici bir klasöre çevrilir).
    #[cfg(target_os = "linux")]
    #[test]
    fn installs_updates_and_removes_appimages() {
        let home = std::env::temp_dir().join(format!("robzhub-appimage-{}", std::process::id()));
        std::fs::create_dir_all(&home).unwrap();
        std::env::set_var("XDG_DATA_HOME", &home);
        let catalog = crate::catalog::parse_str(crate::catalog::BUNDLED).unwrap();
        let def = catalog.app("robz-dpi").unwrap();
        let download = home.join("indirilen.AppImage");

        std::fs::write(&download, b"v1").unwrap();
        install_appimage(def, &download, "0.2.0", Some((b"png".to_vec(), "png"))).unwrap();
        let info = detect_appimage(&def.id);
        assert!(info.installed);
        assert_eq!(info.version.as_deref(), Some("0.2.0"));
        assert_eq!(info.source, Some("appimage"));
        let exe = PathBuf::from(info.exe_path.unwrap());
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(std::fs::metadata(&exe).unwrap().permissions().mode() & 0o111, 0o111);
        }
        let desktop = std::fs::read_to_string(home.join("applications/robz-hub-robz-dpi.desktop")).unwrap();
        assert!(desktop.contains("Icon=") && desktop.contains("robz-dpi.AppImage"));

        std::fs::write(&download, b"v2").unwrap();
        install_appimage(def, &download, "0.3.0", None).unwrap();
        assert_eq!(std::fs::read(&exe).unwrap(), b"v2");
        assert_eq!(detect_appimage(&def.id).version.as_deref(), Some("0.3.0"));

        uninstall_appimage(def).unwrap();
        assert!(!detect_appimage(&def.id).installed);
        assert!(!home.join("applications/robz-hub-robz-dpi.desktop").exists());
        let _ = std::fs::remove_dir_all(&home);
    }
}
