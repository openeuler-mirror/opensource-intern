Name:   luv-icon-theme
Version: 3.9.6
Release: 1%{?dist}
Summary: Lüv is an icon theme for freedesktop environments.
License: CC BY-SA 4.0
URL:     https://github.com/Nitrux/luv-icon-theme.git
Source0: https://github.com/Nitrux/%{name}.tar.gz
Requires:  hicolor-icon-theme
BuildArch: noarch


%description
Lüv is the spiritual successor to Flattr, a flat but complex icon theme for freedesktop environments.

%prep
%setup -q -n %{name}

%install
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/actions/16/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/actions/22/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/actions/32/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/actions/48/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/actions/symbolic/
install -m 0644 -p  Luv/actions/16/* %{buildroot}%{_datadir}/icons/Luv/actions/16/
install -m 0644 -p  Luv/actions/22/* %{buildroot}%{_datadir}/icons/Luv/actions/22/
install -m 0644 -p  Luv/actions/32/* %{buildroot}%{_datadir}/icons/Luv/actions/32/
install -m 0644 -p  Luv/actions/48/* %{buildroot}%{_datadir}/icons/Luv/actions/48/
install -m 0644 -p  Luv/actions/symbolic/* %{buildroot}%{_datadir}/icons/Luv/actions/symbolic/

install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/apps/16/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/apps/22/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/apps/32/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/apps/48/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/apps/64/
install -m 0644 -p  Luv/apps/16/* %{buildroot}%{_datadir}/icons/Luv/apps/16/
install -m 0644 -p  Luv/apps/22/* %{buildroot}%{_datadir}/icons/Luv/apps/22/
install -m 0644 -p  Luv/apps/32/* %{buildroot}%{_datadir}/icons/Luv/apps/32/
install -m 0644 -p  Luv/apps/48/* %{buildroot}%{_datadir}/icons/Luv/apps/48/
install -m 0644 -p  Luv/apps/64/* %{buildroot}%{_datadir}/icons/Luv/apps/64/

install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/categories/16/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/categories/22/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/categories/32/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/categories/48/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/categories/64/
install -m 0644 -p  Luv/categories/16/* %{buildroot}%{_datadir}/icons/Luv/categories/16/
install -m 0644 -p  Luv/categories/22/* %{buildroot}%{_datadir}/icons/Luv/categories/22/
install -m 0644 -p  Luv/categories/32/* %{buildroot}%{_datadir}/icons/Luv/categories/32/
install -m 0644 -p  Luv/categories/48/* %{buildroot}%{_datadir}/icons/Luv/categories/48/
install -m 0644 -p  Luv/categories/64/* %{buildroot}%{_datadir}/icons/Luv/categories/64/

install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/devices/16/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/devices/22/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/devices/32/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/devices/48/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/devices/64/
install -m 0644 -p  Luv/devices/16/* %{buildroot}%{_datadir}/icons/Luv/devices/16/
install -m 0644 -p  Luv/devices/22/* %{buildroot}%{_datadir}/icons/Luv/devices/22/
install -m 0644 -p  Luv/devices/32/* %{buildroot}%{_datadir}/icons/Luv/devices/32/
install -m 0644 -p  Luv/devices/48/* %{buildroot}%{_datadir}/icons/Luv/devices/48/
install -m 0644 -p  Luv/devices/64/* %{buildroot}%{_datadir}/icons/Luv/devices/64/

install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/emblems/8/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/emblems/16/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/emblems/22/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/emblems/32/

install -m 0644 -p  Luv/emblems/8/*  %{buildroot}%{_datadir}/icons/Luv/emblems/8/
install -m 0644 -p  Luv/emblems/16/* %{buildroot}%{_datadir}/icons/Luv/emblems/16/
install -m 0644 -p  Luv/emblems/22/* %{buildroot}%{_datadir}/icons/Luv/emblems/22/
install -m 0644 -p  Luv/emblems/32/* %{buildroot}%{_datadir}/icons/Luv/emblems/32/

install -m 0644 -p -D Luv/emotes/32/face-smile.svg %{buildroot}%{_datadir}/icons/Luv/emotes/32/face-smile.svg


install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/mimetypes/16/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/mimetypes/22/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/mimetypes/32/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/mimetypes/48/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/mimetypes/64/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/mimetypes/128/
install -m 0644 -p  Luv/mimetypes/16/* %{buildroot}%{_datadir}/icons/Luv/mimetypes/16/
install -m 0644 -p  Luv/mimetypes/22/* %{buildroot}%{_datadir}/icons/Luv/mimetypes/22/
install -m 0644 -p  Luv/mimetypes/32/* %{buildroot}%{_datadir}/icons/Luv/mimetypes/32/
install -m 0644 -p  Luv/mimetypes/48/* %{buildroot}%{_datadir}/icons/Luv/mimetypes/48/
install -m 0644 -p  Luv/mimetypes/64/* %{buildroot}%{_datadir}/icons/Luv/mimetypes/64/
install -m 0644 -p  Luv/mimetypes/128/* %{buildroot}%{_datadir}/icons/Luv/mimetypes/128/


install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/places/16/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/places/22/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/places/32/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/places/48/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/places/64/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/places/96/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/places/128/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/places/symbolic/
install -m 0644 -p  Luv/places/16/* %{buildroot}%{_datadir}/icons/Luv/places/16/
install -m 0644 -p  Luv/places/22/* %{buildroot}%{_datadir}/icons/Luv/places/22/
install -m 0644 -p  Luv/places/32/* %{buildroot}%{_datadir}/icons/Luv/places/32/
install -m 0644 -p  Luv/places/48/* %{buildroot}%{_datadir}/icons/Luv/places/48/
install -m 0644 -p  Luv/places/64/* %{buildroot}%{_datadir}/icons/Luv/places/64/
install -m 0644 -p  Luv/places/96/* %{buildroot}%{_datadir}/icons/Luv/places/96/
install -m 0644 -p  Luv/places/128/* %{buildroot}%{_datadir}/icons/Luv/places/128/
install -m 0644 -p  Luv/places/symbolic/* %{buildroot}%{_datadir}/icons/Luv/places/symbolic/

install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/status/16/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/status/22/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/status/24/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/status/32/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/status/48/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/status/64/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/status/128/
install -m 0755 -d %{buildroot}%{_datadir}/icons/Luv/status/symbolic/
install -m 0644 -p  Luv/status/16/* %{buildroot}%{_datadir}/icons/Luv/status/16/
install -m 0644 -p  Luv/status/22/* %{buildroot}%{_datadir}/icons/Luv/status/22/
install -m 0644 -p  Luv/status/24/* %{buildroot}%{_datadir}/icons/Luv/status/24/
install -m 0644 -p  Luv/status/32/* %{buildroot}%{_datadir}/icons/Luv/status/32/
install -m 0644 -p  Luv/status/48/* %{buildroot}%{_datadir}/icons/Luv/status/48/
install -m 0644 -p  Luv/status/64/* %{buildroot}%{_datadir}/icons/Luv/status/64/
install -m 0644 -p  Luv/status/128/* %{buildroot}%{_datadir}/icons/Luv/status/128/
install -m 0644 -p  Luv/status/symbolic/* %{buildroot}%{_datadir}/icons/Luv/status/symbolic/

install -m 0644 -p Luv/icon-theme.cache %{buildroot}%{_datadir}/icons/Luv/
install -m 0644 -p Luv/index.theme %{buildroot}%{_datadir}/icons/Luv/
install -m 0644 -p Luv/LICENSE %{buildroot}%{_datadir}/icons/Luv/

install -m 0644 -p -D Wallpapers/Fifth/contents/images/2560x1440.png %{buildroot}%{_datadir}/wallpapers/Fifth/contents/images/2560x1440.png
install -m 0644 -p Wallpapers/Fifth/contents/*.png  %{buildroot}%{_datadir}/wallpapers/Fifth/contents/
install -m 0644 -p Wallpapers/Fifth/metadata.desktop %{buildroot}%{_datadir}/wallpapers/Fifth

install -m 0644 -p -D Wallpapers/Lines/contents/images/2560x1440.png %{buildroot}%{_datadir}/wallpapers/Lines/contents/images/2560x1440.png
install -m 0644 -p Wallpapers/Lines/contents/*.png  %{buildroot}%{_datadir}/wallpapers/Lines/contents/
install -m 0644 -p Wallpapers/Lines/metadata.desktop %{buildroot}%{_datadir}/wallpapers/Lines

install -m 0644 -p -D Wallpapers/Night/contents/images/2560x1440.png %{buildroot}%{_datadir}/wallpapers/Night/contents/images/2560x1440.png
install -m 0644 -p Wallpapers/Night/contents/*.png  %{buildroot}%{_datadir}/wallpapers/Night/contents/
install -m 0644 -p Wallpapers/Night/metadata.desktop %{buildroot}%{_datadir}/wallpapers/Night

install -m 0644 -p -D Wallpapers/Parallel/contents/images/2560x1440.png %{buildroot}%{_datadir}/wallpapers/Parallel/contents/images/2560x1440.png
install -m 0644 -p Wallpapers/Parallel/contents/*.png  %{buildroot}%{_datadir}/wallpapers/Parallel/contents/
install -m 0644 -p Wallpapers/Parallel/metadata.desktop %{buildroot}%{_datadir}/wallpapers/Parallel

install -m 0644 -p -D Wallpapers/Place/contents/images/2560x1440.png %{buildroot}%{_datadir}/wallpapers/Place/contents/images/2560x1440.png
install -m 0644 -p Wallpapers/Place/contents/*.png  %{buildroot}%{_datadir}/wallpapers/Place/contents/
install -m 0644 -p Wallpapers/Place/metadata.desktop %{buildroot}%{_datadir}/wallpapers/Place

install -m 0644 -p -D Wallpapers/Shine/contents/images/2560x1440.png %{buildroot}%{_datadir}/wallpapers/Shine/contents/images/2560x1440.png
install -m 0644 -p Wallpapers/Shine/contents/*.png  %{buildroot}%{_datadir}/wallpapers/Shine/contents/
install -m 0644 -p Wallpapers/Shine/metadata.desktop %{buildroot}%{_datadir}/wallpapers/Shine

%files
%license Luv/LICENSE
%{_datadir}/icons/Luv/
%{_datadir}/wallpapers/


