Name:    kdeplasma-addons
Summary: Additional Plasmoids for Plasma 5
Version: 5.15.5
Release: 1%{?dist}

License: GPLv2+
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
# http://download.kde.org/stable/plasma/5.15.5/kdeplasma-addons-5.15.5.tar.xz
%ifnarch %{qt5_qtwebengine_arches}
Patch0:  kdeplasma-addons-no-dict-applet-on-secondary-arches.patch
%else
#BuildRequires:  qt5-qtwebengine-devel
%endif

## upstream patches

Obsoletes: kdeplasma-addons-libs < 5.0.0

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-kactivities-devel
BuildRequires:  kf5-kcmutils-devel
BuildRequires:  kf5-kconfig-devel
BuildRequires:  kf5-kconfigwidgets-devel
BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-kdelibs4support-devel
BuildRequires:  kf5-kholidays-devel
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-knewstuff-devel
BuildRequires:  kf5-kross-devel
BuildRequires:  kf5-krunner-devel
BuildRequires:  kf5-kservice-devel
BuildRequires:  kf5-kunitconversion-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
BuildRequires:  libxcb-devel
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  xcb-util-image-devel
BuildRequires:  xcb-util-keysyms-devel
BuildRequires:  cmake(KF5Declarative)
BuildRequires:  cmake(KF5Holidays)

BuildRequires:  plasma-workspace-devel >= %{majmin_ver}
BuildRequires:  libksysguard-devel

# for notes.svgz
Requires:       kf5-plasma >= 5.17

%description
%{summary}.

%package devel
Summary:        Development files for %{name}
# headers only: fixme: confirm need for dep on main pkg? -- rdieter
Requires: %{name} = %{version}-%{release}
#find_dependency(Qt5Gui "5.12.0")
#find_dependency(KF5CoreAddons "5.58.0")
#Requires: cmake(Qt5Gui)
Requires: qt5-qtbase-devel
Requires: cmake(KF5CoreAddons)
%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.

%prep
%autosetup -n %{name}-%{version} -p1


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang kdeplasmaaddons5_qt --with-qt --all-name


%ldconfig_scriptlets

%files -f kdeplasmaaddons5_qt.lang
%license COPYING COPYING.LIB
%{_kf5_datadir}/plasma/plasmoids/*
%{_kf5_datadir}/plasma/desktoptheme/default/widgets/*
%{_kf5_datadir}/plasma/desktoptheme/default/weather/*
%{_kf5_datadir}/plasma/wallpapers/*
%{_kf5_datadir}/plasma/services/*.operations
%{_kf5_qtplugindir}/plasma/dataengine/*.so
%{_kf5_qtplugindir}/plasma/applets/*.so
%{_kf5_qtplugindir}/*.so
%{_kf5_qtplugindir}/potd/
%{_kf5_datadir}/kservices5/*.desktop
%{_kf5_datadir}/kservices5/kwin/*.desktop
%{_kf5_qmldir}/org/kde/plasma/*
%{_datadir}/kwin/desktoptabbox/
%{_datadir}/kwin/tabbox/
%{_datadir}/icons/hicolor/*/apps/fifteenpuzzle.*
%ifarch %{qt5_qtwebengine_arches}
#%%{_datadir}/icons/hicolor/*/apps/accessories-dictionary.svgz
%{_datadir}/plasma/desktoptheme/default/icons/quota.svg
%endif
#%%{_datadir}/knsrcfiles/comic.knsrc
%{_sysconfdir}/xdg/comic.knsrc
%{_kf5_libdir}/libplasmacomicprovidercore.so.*
%{_kf5_libdir}/libplasmapotdprovidercore.so*
%{_kf5_qtplugindir}/kpackage/packagestructure/plasma_packagestructure_comic.so
%{_kf5_datadir}/kservicetypes5/plasma_comicprovider.desktop
%{_kf5_metainfodir}/*.appdata.xml

%{_libdir}/qt5/plugins/plasmacalendarplugins/*
%{_libdir}/qt5/qml/org/kde/plasmacalendar/astronomicaleventsconfig/*


%files devel
%{_libdir}/cmake/PlasmaPotdProvider/
%{_includedir}/plasma/potdprovider/
%{_kf5_datadir}/kdevappwizard/templates/plasmapotdprovider.tar.bz2


%changelog

