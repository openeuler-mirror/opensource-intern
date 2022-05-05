Name:    nx-desktop-settings
Version: 3.9.6
Release: 1%{?dist}
Summary: Defaults settings for NX Desktop

License: GPL-3.0+
URL:     https://github.com/nx-desktop/nx-desktop-settings
Source0: https://github.com/nx-desktop/%{name}.tar.gz
Patch0:  fix-MenuRepresentation.patch
%define debug_package %{nil}

%description
 Default settings for:
  - KDE Plasma
  - Latte
  - Kvantum



%prep
%setup -q -n %{name}
%patch0 -p1


%build

%install

install -m 0755 -d %{buildroot}%{_datadir}/plasma-systemmonitor/
install -m 0644 -p usr/share/plasma-systemmonitor/* %{buildroot}%{_datadir}/plasma-systemmonitor/

install -m 0755 -d %{buildroot}%{_datadir}/plasma/plasmoids/org.kde.latte.separator/contents/ui/
install -m 0755 -d %{buildroot}%{_datadir}/plasma/plasmoids/org.kde.latte.separator/contents/config/
install -m 0644 -p usr/share/plasma/plasmoids/org.kde.latte.separator/contents/ui/* %{buildroot}%{_datadir}/plasma/plasmoids/org.kde.latte.separator/contents/ui/
install -m 0644 -p usr/share/plasma/plasmoids/org.kde.latte.separator/contents/config/* %{buildroot}%{_datadir}/plasma/plasmoids/org.kde.latte.separator/contents/config/
install -m 0644 -p usr/share/plasma/plasmoids/org.kde.latte.separator/metadata.desktop %{buildroot}%{_datadir}/plasma/plasmoids/org.kde.latte.separator/metadata.desktop

install -m 0755 -d %{buildroot}%{_datadir}/plasma/plasmoids/launchpadPlasmaMod/contents/ui/
install -m 0755 -d %{buildroot}%{_datadir}/plasma/plasmoids/launchpadPlasmaMod/contents/config/
install -m 0755 -d %{buildroot}%{_datadir}/plasma/plasmoids/launchpadPlasmaMod/contents/code/
install -m 0644 -p usr/share/plasma/plasmoids/launchpadPlasmaMod/contents/ui/* %{buildroot}%{_datadir}/plasma/plasmoids/launchpadPlasmaMod/contents/ui/
install -m 0644 -p usr/share/plasma/plasmoids/launchpadPlasmaMod/contents/config/* %{buildroot}%{_datadir}/plasma/plasmoids/launchpadPlasmaMod/contents/config/
install -m 0644 -p usr/share/plasma/plasmoids/launchpadPlasmaMod/contents/code/tools.js %{buildroot}%{_datadir}/plasma/plasmoids/launchpadPlasmaMod/contents/code/tools.js
install -m 0644 -p usr/share/plasma/plasmoids/launchpadPlasmaMod/metadata.desktop %{buildroot}%{_datadir}/plasma/plasmoids/launchpadPlasmaMod/metadata.desktop


install -m 0755 -d %{buildroot}%{_datadir}/plasma/plasmoids/com.github.configurable_button/contents/ui/
install -m 0755 -d %{buildroot}%{_datadir}/plasma/plasmoids/com.github.configurable_button/contents/config/
install -m 0644 -p usr/share/plasma/plasmoids/com.github.configurable_button/contents/ui/* %{buildroot}%{_datadir}/plasma/plasmoids/com.github.configurable_button/contents/ui/
install -m 0644 -p usr/share/plasma/plasmoids/com.github.configurable_button/contents/config/* %{buildroot}%{_datadir}/plasma/plasmoids/com.github.configurable_button/contents/config/
install -m 0644 -p usr/share/plasma/plasmoids/com.github.configurable_button/metadata.desktop %{buildroot}%{_datadir}/plasma/plasmoids/com.github.configurable_button/metadata.desktop

install -m 0755 -d %{buildroot}%{_datadir}/kwin/scripts/Parachute/contents/ui/images/
install -m 0644 -p usr/share/kwin/scripts/Parachute/contents/ui/images/*.svg %{buildroot}%{_datadir}/kwin/scripts/Parachute/contents/ui/images/
install -m 0644 -p usr/share/kwin/scripts/Parachute/contents/ui/*.qml %{buildroot}%{_datadir}/kwin/scripts/Parachute/contents/ui
install -m 0644 -p -D usr/share/kwin/scripts/Parachute/contents/config/main.xml  %{buildroot}%{_datadir}/kwin/scripts/Parachute/contents/config/main.xml
install -m 0644 -p usr/share/kwin/scripts/Parachute/metadata.desktop %{buildroot}%{_datadir}/kwin/scripts/Parachute/metadata.desktop

install -m 0755 -d %{buildroot}%{_datadir}/kwin/scripts/krohnkite/contents/ui/
install -m 0644 -p usr/share/kwin/scripts/krohnkite/contents/ui/*  %{buildroot}%{_datadir}/kwin/scripts/krohnkite/contents/ui/
install -m 0644 -p -D usr/share/kwin/scripts/krohnkite/contents/config/main.xml %{buildroot}%{_datadir}/kwin/scripts/krohnkite/contents/config/main.xml
install -m 0644 -p -D usr/share/kwin/scripts/krohnkite/contents/code/script.js %{buildroot}%{_datadir}/kwin/scripts/krohnkite/contents/code/script.js
install -m 0644 -p usr/share/kwin/scripts/krohnkite/metadata.desktop %{buildroot}%{_datadir}/kwin/scripts/krohnkite/metadata.desktop

install -m 0644 -p -D usr/share/drirc.d/99-mesa-customizations.conf %{buildroot}%{_datadir}/drirc.d/99-mesa-customizations.conf

install -m 0644 -p -D usr/lib/plasma-hud/plasma-hud %{buildroot}%{_usr}/lib/plasma-hud/plasma-hud

install -m 0755 -d %{buildroot}%{_sysconfdir}/xdg/autostart
install -m 0644 -p etc/xdg/*rc %{buildroot}%{_sysconfdir}/xdg/
install -m 0644 -p etc/xdg/kdeglobals %{buildroot}%{_sysconfdir}/xdg/
install -m 0644 -p etc/xdg/autostart/*.desktop %{buildroot}%{_sysconfdir}/xdg/autostart/


install -m 0644 -p -D etc/udev/rules.d/50-ioschedulers.rules %{buildroot}%{_sysconfdir}/udev/rules.d/50-ioschedulers.rules

install -m 0755 -d %{buildroot}%{_sysconfdir}/skel/Applications/
install -m 0755 -d %{buildroot}%{_sysconfdir}/skel/.npm-packages/
install -m 0755 -d %{buildroot}%{_sysconfdir}/skel/.local/share/applications/
install -m 0644 -p etc/skel/.local/share/applications/*.desktop %{buildroot}%{_sysconfdir}/skel/.local/share/applications/
install -m 0644 -p -D etc/skel/.config/xsettingsd/xsettingsd.conf %{buildroot}%{_sysconfdir}/skel/.config/xsettingsd/xsettingsd.conf
install -m 0644 -p -D etc/skel/.config/org.kde.maui/mauiproject.conf %{buildroot}%{_sysconfdir}/skel/.config/org.kde.maui/mauiproject.conf
install -m 0755 -d %{buildroot}%{_sysconfdir}/skel/.config/Maui
install -m 0644 -p etc/skel/.config/Maui/*.conf %{buildroot}%{_sysconfdir}/skel/.config/Maui
install -m 0755 -d %{buildroot}%{_sysconfdir}/skel/.config/latte
install -m 0644 -p etc/skel/.config/latte/*.latte %{buildroot}%{_sysconfdir}/skel/.config/latte
install -m 0644 -p -D etc/skel/.config/Kvantum/kvantum.kvconfig %{buildroot}%{_sysconfdir}/skel/.config/Kvantum/kvantum.kvconfig
install -m 0755 -d %{buildroot}%{_sysconfdir}/skel/.config/kde.org
install -m 0644 -p  etc/skel/.config/kde.org/*.conf %{buildroot}%{_sysconfdir}/skel/.config/kde.org
install -m 0644 -p -D etc/skel/.config/inkscape/preferences.xml %{buildroot}%{_sysconfdir}/skel/.config/inkscape/preferences.xml
install -m 0644 -p -D etc/skel/.config/gtk-4.0/settings.ini %{buildroot}%{_sysconfdir}/skel/.config/gtk-4.0/settings.ini
install -m 0755 -d %{buildroot}%{_sysconfdir}/skel/.config/gtk-3.0
install -m 0644 -p etc/skel/.config/gtk-3.0/* %{buildroot}%{_sysconfdir}/skel/.config/gtk-3.0
install -m 0755 -d %{buildroot}%{_sysconfdir}/skel/.config/autostart
install -m 0644 -p etc/skel/.config/autostart/* %{buildroot}%{_sysconfdir}/skel/.config/autostart
install -m 0644 -p etc/skel/.config/*rc %{buildroot}%{_sysconfdir}/skel/.config/
install -m 0644 -p etc/skel/.config/*.sh %{buildroot}%{_sysconfdir}/skel/.config/
install -m 0644 -p etc/skel/.config/gtkrc* %{buildroot}%{_sysconfdir}/skel/.config/
install -m 0644 -p etc/skel/.config/gamemode.ini %{buildroot}%{_sysconfdir}/skel/.config/

install -m 0644 -p etc/skel/.*rc %{buildroot}%{_sysconfdir}/skel/
install -m 0644 -p etc/skel/.p10k.zsh %{buildroot}%{_sysconfdir}/skel/
install -m 0644 -p etc/skel/.gtkrc-2.0-kde4 %{buildroot}%{_sysconfdir}/skel/

install -m 0644 -p etc/*.conf  %{buildroot}%{_sysconfdir}
install -m 0644 -p etc/*.yaml  %{buildroot}%{_sysconfdir}

%files
%{_sysconfdir}/
%{_datadir}/
%{_usr}/lib/plasma-hud/plasma-hud

%post
echo -e "XDG_CONFIG_DIRS=/etc/xdg" >> /etc/environment
echo -e "XDG_DATA_DIRS=/usr/local/share:/usr/share" >> /etc/environment
sed -i "s|secure_path\=.*$|secure_path=\"$PATH:/Applications\"|g" /etc/sudoers
sed -i "/env_reset/d" /etc/sudoers
#	let krohnkite be configured from system settings.

ln -sv /usr/share/kwin/scripts/krohnkite/metadata.desktop /usr/share/kservices5/krohnkite.desktop


#	let Parachute be configured from system settings.

ln -sv /usr/share/kwin/scripts/Parachute/metadata.desktop /usr/share/kservices5/parachute.desktop


#	let Force blur be configured from system settings.

ln -sv /usr/share/kwin/scripts/forceblur/metadata.desktop /usr/share/kservices5/forceblur.desktop

cp /usr/share/icons/nitrux_snow_cursors/index.theme /etc/X11/cursors/nitrux_cursors.theme
ln -svf /etc/X11/cursors/nitrux_cursors.theme /etc/alternatives/x-cursor-theme
sed -i '$ a Inherits=nitrux_snow_cursors' /etc/X11/cursors/nitrux_cursors.theme
chown -c root:root /etc/doas.conf
chmod -c 0400 /etc/doas.conf
git clone https://github.com/robbyrussell/oh-my-zsh.git /etc/skel/.oh-my-zsh
mkdir -p /etc/skel/.oh-my-zsh/themes/powerlevel10k
git clone --depth=1 https://github.com/romkatv/powerlevel10k.git /etc/skel/.oh-my-zsh/themes/powerlevel10k
%changelog


