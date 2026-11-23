# Değişiklikler

Her sürümün notları burada `## X.Y.Z` başlığı altında durur; sürüm iş akışı (.github/workflows/release.yml) GitHub
sürüm notunu bu bölümden alır.

## 0.1.9

<!-- tr -->
- **Düzeltme: AppImageLauncher kurduğun uygulamayı taşıyordu.** Arch gibi sistemlerde AppImageLauncher kuruluysa, Robz Hub'ın kurduğu bir uygulama ilk açılışta "tümleştir" sorusuyla ~/Applications klasörüne taşınabiliyordu; Robz Hub onu yerinde bulamayınca yeniden "Kur" gösteriyordu. Robz Hub'ın açtığı uygulamalar ve menüye eklediği kısayollar artık AppImageLauncher araya girmeden açılır. Daha önce taşınmış bir uygulamayı Robz Hub'dan yeniden kurman yeterli.
- **Düzeltme: rpm ile kurulan uygulama kurulumdan sonra bulunamıyordu.** Fedora ve openSUSE gibi rpm kullanan dağıtımlarda kurulum bittiği hâlde "Kur" düğmesi kalıyordu. rpm veritabanındaki değişiklik artık doğru algılanır.
- İki düzeltme de bberkay'ın katkısı ([#1](https://github.com/kutsalkiremitci/robz-hub/pull/1), [#2](https://github.com/kutsalkiremitci/robz-hub/pull/2)).

<!-- en -->
<details><summary>English</summary>

- **Fix: AppImageLauncher moved the apps you installed.** On systems such as Arch with AppImageLauncher installed, an app installed by Robz Hub could be moved to ~/Applications by its "integrate" prompt on first launch; Robz Hub then could not find it and showed "Install" again. Apps started by Robz Hub and the menu shortcuts it creates now open without AppImageLauncher stepping in. If an app was already moved, reinstall it from Robz Hub.
- **Fix: an app installed with rpm was not found after installing.** On rpm-based distributions such as Fedora and openSUSE the "Install" button stayed even though the install had finished. Changes to the rpm database are now detected correctly.
- Both fixes were contributed by bberkay ([#1](https://github.com/kutsalkiremitci/robz-hub/pull/1), [#2](https://github.com/kutsalkiremitci/robz-hub/pull/2)).

</details>

## 0.1.8

<!-- tr -->
- **Taşınabilir Windows sürümü.** RobzHub-Portable.exe kurulmadan çalışır: ayarlar, önbellek ve indirmeler exe'nin yanındaki RobzHub-Data klasöründe durur, bilgisayarda iz kalmaz. USB bellekte taşınabilir ve kendini de günceller. Ayarlar sayfasında "Taşınabilir" etiketi ve verilerin yeri görünür.
- Web sitesinde Windows ziyaretçilerine taşınabilir sürümün bağlantısı da gösterilir.
- **Düzeltme: açılışta uzun süren çark.** Linux'ta Robz Hub, GitHub'a yavaş ulaşılırken ya da ulaşılamazken yarım dakikaya kadar boş bir çarkta bekliyordu. Artık kayıtlı bilgilerle hemen açılır; GitHub denetimi arkada sürer ve daha kısa sürer. Açılış ekranında ne olduğunu söyleyen bir yazı da var.
- **Düzeltme: "Kurulum klasörünü aç" ve bağlantılar Linux'ta çalışmıyordu.** Robz Hub AppImage olarak çalışırken başlattığı programlar (dosya yöneticisi, tarayıcı, kurduğun uygulamalar) onun paketindeki kitaplıkları yüklemeye çalışıp açılmıyordu. Artık temiz bir ortamla başlatılır.
- **Düzeltme: kurulum bitince düğme bir an yeniden "Kur" görünebiliyordu.** Durum artık sırayla ve tek seferde güncellenir; eski bir durum yenisinin üzerine yazılamaz. Ekran 30 saniyede bir kendini de eşitler.
- **Günlük dosyası.** Ayarlar › Hakkında'daki "Günlük dosyasını göster", kurulum hatalarının ayrıntısının yazıldığı robz-hub.log dosyasını açar; sorun bildirirken eklenir. Hiçbir yere gönderilmez.

<!-- en -->
<details><summary>English</summary>

- **Portable Windows version.** RobzHub-Portable.exe runs without installing: settings, cache and downloads stay in the RobzHub-Data folder next to the exe and nothing is left on the PC. Carry it on a USB stick; it updates itself too. The Settings page shows a "Portable" label and where the data lives.
- The website also shows Windows visitors a link to the portable version.
- **Fix: long spinner at startup.** On Linux, Robz Hub could sit on an empty spinner for up to half a minute when GitHub was slow or unreachable. It now opens right away with what it already knows; the GitHub check continues in the background and takes less time. The startup screen also says what is happening.
- **Fix: "Open install folder" and links did nothing on Linux.** When Robz Hub runs as an AppImage, the programs it starts (file manager, browser, the apps you install) tried to load the libraries inside its package and failed to open. They now start with a clean environment.
- **Fix: after an install the button could briefly read "Install" again.** State now updates in order and in one step; an older state can no longer overwrite a newer one. The screen also resyncs every 30 seconds.
- **Log file.** "Show the log file" under Settings › About opens robz-hub.log, where install errors are recorded in detail; attach it when reporting a problem. It is never sent anywhere.

</details>
<!-- end -->

## 0.1.7

<!-- tr -->
- **Her Linux dağıtımında.** Robz Hub artık uygulamaların AppImage sürümlerini de kurar: Arch, Manjaro ve .deb/.rpm kullanmayan bütün dağıtımlarda. AppImage parola istemez, senin klasörüne yerleşir ve uygulama menüsüne eklenir; güncelleme ve kaldırma da Robz Hub'dan.
- **Sistemine uyan dosya.** Robz Hub her uygulamanın sürümündeki dosyalara bakar ve tek bir "Kur" düğmesi gösterir: Windows'ta kurulum programı, Debian/Ubuntu'da .deb, Fedora/openSUSE'de .rpm, diğerlerinde AppImage. Uygun dosya yoksa nedeniyle yazar: "Windows'ta desteklenmiyor", "Linux'ta desteklenmiyor" ya da "Bu dağıtımda desteklenmiyor". Dağıtım artık doğru algılanır (Arch'ta rpm aracı kurulu olsa bile).
- **Yeni biçimler kendiliğinden.** Bir uygulama sonradan yeni bir biçim yayınlarsa (ör. sürümüne AppImage eklerse) Robz Hub bir sonraki denetimde onu kullanır; Robz DPI'ın 0.3.0 AppImage'ı böylece Arch'ta kurulabiliyor.
- **Bütün Robz uygulamaları listede.** GitHub'da robz- ile başlayan her uygulama, hangi sistemi desteklerse desteklesin listede görünür; henüz sürümü yoksa "henüz sürüm yok" der.
- **Web sitesinde tek düğme.** Site ziyaretçinin sistemini tanır: Windows'ta kurulum dosyası, Linux'ta dağıtıma göre .deb, .rpm ya da AppImage; diğer biçimler altta.
- **Sürümde tek dosya.** Her dosyanın tek, sabit adı var; aynı dosyanın sürüm numaralı kopyaları kaldırıldı.
- **Düzeltme:** kendiliğinden bulunan bir uygulamanın sürüm notları sekmesi açılırken hata veriyordu.

<!-- en -->
<details><summary>English</summary>

- **On every Linux distribution.** Robz Hub now installs apps' AppImage versions too: on Arch, Manjaro and every distribution that doesn't use .deb/.rpm. AppImages need no password, go into your own folder and into the app menu; updating and uninstalling work from Robz Hub as well.
- **The file for your system.** Robz Hub looks at the files in each app's release and shows a single "Install" button: the installer on Windows, the .deb on Debian/Ubuntu, the .rpm on Fedora/openSUSE, the AppImage elsewhere. Without a suitable file it says why: "Not available on Windows", "Not available on Linux" or "Not available on this distribution". The distribution is now detected correctly (even on Arch with the rpm tool installed).
- **New formats on their own.** If an app later publishes a new format (for example adds an AppImage to its release), Robz Hub uses it on the next check; this is how Robz DPI 0.3.0's AppImage installs on Arch.
- **Every Robz app in the list.** Every GitHub app starting with robz- shows up whatever systems it supports; without a release yet it says "no release yet".
- **One button on the website.** The site recognizes the visitor's system: the installer on Windows, the .deb, .rpm or AppImage on Linux depending on the distribution; other formats below.
- **One file per format.** Each release file has a single fixed name; the version-numbered copies of the same files are gone.
- **Fix:** the release notes tab of an automatically found app failed to open.

</details>
<!-- end -->

## 0.1.6

<!-- tr -->
- **Linux desteği.** Robz Hub artık Linux'ta da çalışır: .deb (Debian, Ubuntu, Mint), .rpm (Fedora, openSUSE) ve AppImage. Linux paketi olan uygulamalar paket yöneticisiyle kurulur, güncellenir ve kaldırılır; bunun için sistemin parola penceresi açılır. Linux sürümü olmayan uygulamalarda "Linux'ta desteklenmiyor" yazar. Robz Hub kendini Linux'ta da günceller.
- **Yeni uygulamaları kendisi bulur.** GitHub'da robz- ile başlayan yeni bir depo kurulum dosyası yayınlayınca, kataloğa eklenmeyi beklemeden Robz Hub'da görünür ve bildirim gelir. Denetim saatte en çok bir kez, tek istekle yapılır.
- **Robz DPI eklendi.** Yalnız seçtiğin sitelerde çalışan açık kaynak DPI aracı; Windows'ta ve Linux'ta kurulur.
- **İndirme ve yıldız sayıları.** Uygulama sayfasında GitHub'daki toplam indirme ve yıldız sayısı görünür; ana sayfadaki kartlarda indirme sayısı beyaz, yıldız sarı.
- Simgesi yüklenemeyen uygulamada adının baş harfleri görünür.

<!-- en -->
<details><summary>English</summary>

- **Linux support.** Robz Hub now runs on Linux too: .deb (Debian, Ubuntu, Mint), .rpm (Fedora, openSUSE) and AppImage. Apps with a Linux package install, update and uninstall through your package manager, which opens your system's password prompt. Apps without a Linux version say "Not available on Linux". Robz Hub updates itself on Linux too.
- **Finds new apps on its own.** When a new GitHub repository starting with robz- publishes an installer, it shows up in Robz Hub without waiting for the catalog, and you get a notification. It checks at most once an hour, with a single request.
- **Robz DPI added.** An open-source DPI tool that works only on the sites you choose; installs on Windows and Linux.
- **Downloads and stars.** The app page shows the total downloads and stars on GitHub; on the home page cards, downloads are white and stars yellow.
- An app whose icon can't load shows the initials of its name.

</details>
<!-- end -->

## 0.1.5

<!-- tr -->
- **Yeni ana sayfa.** Üstte dönen bir tanıtım alanı var: GTA VI paketi ve duyurular. Tek düğmeyle iki uygulamayı birden kurar, günceller ya da başlatırsın. Altında kapak görselli uygulama kartları, bütün uygulamaların son sürümleri ve Topluluk bölümü yer alır.
- **İlk duyurular Discord'da.** Yeni sürümler ve uygulamalar her zaman önce Discord sunucusunda duyurulur. Topluluk bölümünde Discord en üstte; "Sıradaki Robz uygulaması" kartı da Discord'a götürür. Canlı yayınlar Kick'te.
- **Gerçek Discord ve Kick logoları** kenar çubuğunda, Ayarlar'da ve ana sayfada.
- **Atlanan sürümlerin notları.** Robz Hub'ın birkaç sürümünü atladıysan, güncelleme kartı aradaki bütün sürümlerin notlarını gösterir; en yenisi üstte.
- **Düzeltme:** GitHub'da yeni bir sürüm çıktığında "Güncelleme var", son sürüm ve indirme sayısı hemen yenilenir. 0.1.4'te bunlar, uygulamanın kurulumu değişene kadar eski kalabiliyordu.

0.1.2 ile 0.1.4 arasında gelenler de bu sürümde:
- Bayraklı dil seçimi.
- GitHub indirme sayıları.
- Uygulamanın kendi içinden yapılan güncellemeler birkaç saniyede görünür.
- Kendini güncelledikten sonra indirdiği kurulum dosyasını siler.

<!-- en -->
<details><summary>English</summary>

- **New home page.** At the top, a rotating promo area shows the GTA VI bundle and announcements. One button installs, updates or launches both apps at once. Below it are app cards with cover images, the latest releases of every app and a Community section.
- **Announced first on Discord.** New releases and apps are always announced on the Discord server first. Discord is at the top of the Community section, and "The next Robz app" card links to Discord. Live streams are on Kick.
- **Real Discord and Kick logos** in the sidebar, Settings and the home page.
- **Notes for skipped versions.** If you skipped a few Robz Hub versions, the update card shows the notes of every version in between, newest first.
- **Fix:** when a new release appears on GitHub, "Update available", the latest version and the download count refresh right away. In 0.1.4 they could stay stale until the app's installation changed.

Also in this release, from 0.1.2 to 0.1.4:
- A language picker with flags.
- GitHub download counts.
- Updates made inside an app show up within seconds.
- Robz Hub deletes the installer it downloaded after updating itself.

</details>
<!-- end -->

## 0.1.4

<!-- tr -->
- **Dışarıda yapılan değişiklikler birkaç saniyede görünür.** Bir uygulamayı kendi içindeki "Güncelle" düğmesiyle güncellersen, Robz Hub dışında kurarsan ya da Windows Ayarlar'dan kaldırırsan Robz Hub bunu birkaç saniye içinde görür; penceresi kapalıyken ya da tepsideyken de. Sürüm, "Güncelleme var" sayacı ve tepsi menüsü kendiliğinden güncellenir.

<!-- en -->
<details><summary>English</summary>

- **Changes made elsewhere show up within seconds.** If you update an app with its own "Update" button, install it outside Robz Hub or uninstall it from Windows Settings, Robz Hub notices within a few seconds, even while its window is closed or it's in the tray. The version, the update counter and the tray menu update on their own.

</details>
<!-- end -->

## 0.1.3

<!-- tr -->
- **İndirme sayıları.** Her uygulamanın GitHub'daki toplam indirme sayısı uygulama listesinde ve uygulama sayfasında, her sürümün indirme sayısı sürüm notlarında görünür.
- **Bayraklı dil seçimi.** Kenar çubuğundaki TR / EN düğmelerinin yerine Türk ve İngiliz bayraklı bir seçim kutusu geldi; seçtiğin an dil değişir. Ayarlar'da "Windows dili" seçeneği de bu kutuda.

<!-- en -->
<details><summary>English</summary>

- **Download counts.** Each app's total downloads on GitHub show in the app list and on the app page, and each release's download count shows in the release notes.
- **Flag language picker.** The TR / EN buttons in the sidebar are replaced by a picker with the Turkish and British flags; the language changes as soon as you choose. In Settings, the "Windows language" option is in the same picker.

</details>
<!-- end -->

## 0.1.2

<!-- tr -->
- Robz Hub kendini güncelledikten sonra, güncelleme için indirdiği kurulum dosyasını siler; indirilenler klasöründe yer kaplamaz.
- Sırada bekleyen bir kurulumun ilerleme çubuğu artık boş durur; çubuk yalnız indirme, kapatma, kurma ve kaldırma sırasında hareket eder.

<!-- en -->
<details><summary>English</summary>

- After updating itself, Robz Hub deletes the installer it downloaded for the update, so it no longer takes up space in the downloads folder.
- A queued install's progress bar now stays empty; the bar only moves while downloading, closing, installing or uninstalling.

</details>
<!-- end -->

## 0.1.1

<!-- tr -->
- **Yalnız kurulu kopya kapatılır.** Bir uygulamanın açık olup olmadığı artık exe adıyla değil, kurulu olduğu klasörle anlaşılır. Aynı adla başka bir yerden çalışan bir kopya (ör. geliştirme klasöründen açılmış bir derleme) "Çalışıyor" sayılmaz; kurulum, güncelleme ve "Kapat" ona dokunmaz.
- Paket kutusunda bir uygulama eksik, diğerinde güncelleme varken düğme "Kur ve güncelle" yazar.
- Robz Hub'ın kendi sürüm notları seçili dilde gösterilir.
- Yeni turuncu logo ve ana düğmeler.

<!-- en -->
<details><summary>English</summary>

- **Only the installed copy is closed.** Whether an app is running is now decided by the folder it's installed in, not by its exe name. A copy with the same name running from elsewhere (for example a build in a development folder) doesn't count as "Running", and installing, updating or "Close" leave it alone.
- When one app in a bundle is missing and the other has an update, the button reads "Install and update".
- Robz Hub's own release notes show in the selected language.
- New orange logo and primary buttons.

</details>
<!-- end -->

## 0.1.0

<!-- tr -->
İlk sürüm.

- **Tek yerden kur, güncelle, kaldır.** Robz Play ve Robz Vision tek tıkla kurulur; yeni sürüm çıkınca "Güncelle" düğmesi belirir. Kurulum dosyası GitHub'dan iner ve SHA-256 özetiyle doğrulanır.
- **Başlat ve kapat.** Kurulu uygulamalar buradan açılır ve kapatılır; çalışanlar yeşil noktayla görünür.
- **GTA VI paketi.** "İkisini de kur" ile Robz Play (60 FPS) ve Robz Vision (Türkçe altyazı) birlikte kurulur, "İkisini de başlat" ile birlikte açılır.
- **Sürüm notları.** Her uygulamanın bütün sürümleri; kurulu sürümden sonra gelenler "Yeni" etiketiyle.
- **Türkçe ve İngilizce.** Kenar çubuğundaki TR / EN ile değişir.
- **Tepside çalışma, Windows açılışında başlatma ve yeni sürüm bildirimi** (Ayarlar'dan).

<!-- en -->
<details><summary>English</summary>

First release.

- **Install, update and uninstall from one place.** Robz Play and Robz Vision install with one click; an "Update" button appears when a new version is out. Installers download from GitHub and are verified against their SHA-256 digest.
- **Launch and close.** Open and close installed apps from here; running ones show a green dot.
- **GTA VI bundle.** "Install both" sets up Robz Play (60 FPS) and Robz Vision (Turkish subtitles) together; "Launch both" opens them together.
- **Release notes.** Every release of every app; the ones newer than your installed version are marked "New".
- **Turkish and English.** Switch with TR / EN in the sidebar.
- **Tray mode, start with Windows and update notifications** (in Settings).

</details>
<!-- end -->
