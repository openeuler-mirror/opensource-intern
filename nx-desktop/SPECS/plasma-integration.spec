Name:    plasma-integration
Summary: Qt Platform Theme integration plugin for Plasma
Version: 5.15.5
Release: 1%{?dist}

# KDE e.V. may determine that future LGPL versions are accepted
License: LGPLv2 or LGPLv3
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global majmin_ver %(echo %{version} | cut -d. -f1,2).50
%global stable unstable
%else
%global majmin_ver %(echo %{version} | cut -d. -f1,2)
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/plasma-integration-5.15.5.tar.xz
BuildRequires:  kf5-rpm-macros
BuildRequires:  extra-cmake-modules

BuildRequires:  pkgconfig(x11)
BuildRequires:  pkgconfig(xcursor)

#BuildRequires:  cmake(Qt5Widgets)
#BuildRequires:  cmake(Qt5DBus)
BuildRequires:  qt5-qtbase-devel
BuildRequires:  pkgconfig(Qt5X11Extras)
BuildRequires:  pkgconfig(Qt5QuickControls2)
# Qt5PlatformSupport
BuildRequires:  qt5-qtbase-static

BuildRequires:  cmake(KF5Config)
BuildRequires:  cmake(KF5ConfigWidgets)
BuildRequires:  cmake(KF5I18n)
BuildRequires:  cmake(KF5IconThemes)
BuildRequires:  cmake(KF5KIO)
BuildRequires:  cmake(KF5Notifications)
BuildRequires:  cmake(KF5WidgetsAddons)
BuildRequires:  cmake(KF5WindowSystem)
BuildRequires:  cmake(KF5Wayland)

## TODO: verify this is needed, not 100% sure -- rex
BuildRequires: qt5-qtbase-private-devel
%{?_qt5:Requires: %{_qt5}%{?_isa} = %{_qt5_version}}

BuildRequires:  plasma-breeze-devel >= %{majmin_ver}
Requires:       plasma-breeze >= %{majmin_ver}
Requires:       breeze-cursor-theme >= %{majmin_ver}
Requires:       breeze-icon-theme
Requires:       plasma-workspace >= %{majmin_ver}

%description
%{summary}.


%prep
%autosetup -p1

sed -i.breeze_version \
  -e "s|^find_package(Breeze \${PROJECT_VERSION} |find_package(Breeze %{majmin_ver} |g" \
  CMakeLists.txt


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang plasmaintegration5


%files -f plasmaintegration5.lang
%doc README.md
%license COPYING.LGPL-3
%{_kf5_qtplugindir}/platformthemes/KDEPlasmaPlatformTheme.so
%{_kf5_datadir}/kconf_update/fonts_*


%changelog



