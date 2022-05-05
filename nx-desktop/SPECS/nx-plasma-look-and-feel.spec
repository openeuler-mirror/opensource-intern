Name:    nx-plasma-look-and-feel
Version: 3.9.6
Release: 1%{?dist}
Summary: NX Look and Feel package for Plasma 5.8.4+

License: GPL-3.0+
URL:     https://github.com/nx-desktop/nx-plasma-look-and-feel
Source0: https://github.com/nx-desktop/%{name}.tar.gz
#Patch0: fit-qt-5.11-sddm.patch
Patch0: remove-lockscreen.patch
BuildRequires:  extra-cmake-modules
BuildRequires:  cmake
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
BuildRequires:  kf5-kwindowsystem-devel
Requires: plasma-workspace
%define debug_package %{nil}
%description
Plasma look and feel package containing the artwork used in Nitrux.


%prep
%setup -q -n %{name}
%patch0 -p1
#%%patch1 -p1

%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} -DLIB_INSTALL_DIR=lib  ../
popd

%make_build -C %{_target_platform}


%install
make install/fast  DESTDIR=%{buildroot} -C %{_target_platform}

%files
%{_kf5_datadir}/aurorae/
%{_kf5_datadir}/color-schemes/
%{_kf5_datadir}/icons/nitrux_cursors/
%{_kf5_datadir}/icons/nitrux_snow_cursors/
%{_kf5_datadir}/konsole/
%{_kf5_datadir}/plasma/desktoptheme/nitrux-dark/
%{_kf5_datadir}/plasma/desktoptheme/nitrux/
%{_kf5_datadir}/plasma/look-and-feel/org.kde.nitrux.dark.desktop/
%{_kf5_datadir}/plasma/look-and-feel/org.kde.nitrux.desktop/
%{_kf5_datadir}/plasma/look-and-feel/org.kde.nitrux.mix.desktop/
%{_kf5_datadir}/plasma/plasmoids/org.kde.windowtitle/
%{_kf5_datadir}/plasma/wallpapers/com.darkeye.timedImage/
%{_kf5_datadir}/plasma/wallpapers/org.kde.video/
%{_kf5_datadir}/sddm/themes/nitrux-dark
%{_kf5_datadir}/sddm/themes/nitrux/
%{_kf5_datadir}/wallpapers/

%post
sed -i "s/#Current=01-breeze-fedora/Current=nitrux/g" /etc/sddm.conf
sed -i "s/#CursorTheme=/CursorTheme=nitrux_cursors/g" /etc/sddm.conf

%postun
sed -i "s/Current=nitrux/#Current=01-breeze-fedora/g" /etc/sddm.conf
sed -i "s/CursorTheme=nitrux_cursors/#CursorTheme=/g" /etc/sddm.conf


%changelog





