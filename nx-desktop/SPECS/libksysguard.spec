 
Name:    libksysguard
Summary: Library for managing processes running on the system
Version: 5.15.5
Release: 1%{?dist}

License: GPLv2+
URL:     https://invent.kde.org/plasma/%{name}

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/libksysguard-5.15.5.tar.xz
# GCC 8 and older need stdc++fs link library set
# Patch1:            libksysguard-5.22.2.1_fix-processcore-on-gcc8.patch

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-rpm-macros
# kf5 required
BuildRequires:  cmake(KF5CoreAddons)
BuildRequires:  cmake(KF5Config)
BuildRequires:  cmake(KF5I18n)
BuildRequires:  cmake(KF5WindowSystem)
BuildRequires:  cmake(KF5Completion)
BuildRequires:  cmake(KF5Auth)
BuildRequires:  cmake(KF5WidgetsAddons)
BuildRequires:  cmake(KF5IconThemes)
BuildRequires:  cmake(KF5ConfigWidgets)
BuildRequires:  cmake(KF5Service)
# 	kf5-kglobalaccel-devel
BuildRequires: 	kf5-kglobalaccel-devel
BuildRequires:  cmake(KF5GlobalAccel)
BuildRequires:  cmake(KF5KIO)
BuildRequires:  cmake(KF5Declarative)
# 	kf5-knewstuff-devel
BuildRequires: 	kf5-knewstuff-devel
BuildRequires:  cmake(KF5NewStuff)
# kf5 optional
BuildRequires:  cmake(KF5Plasma)
# qt5 required
BuildRequires:  qt5-qttools-devel
# BuildRequires:  cmake(Qt5DBus)
# BuildRequires:  cmake(Qt5Network)
# BuildRequires:  cmake(Qt5Widgets)
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtx11extras-devel
# BuildRequires:  cmake(Qt5X11Extras)
BuildRequires: 	qt5-qtwebchannel-devel

# qt5 optional
%ifarch %{qt5_qtwebengine_arches}
# BuildRequires:  cmake(Qt5WebEngineWidgets)
BuildRequires: 	qt5-qtwebchannel-devel
%endif
BuildRequires:  libpcap-devel
BuildRequires:  pkgconfig(libpcap)
BuildRequires:  libnl3-devel
BuildRequires:  pkgconfig(libnl-3.0) pkgconfig(libnl-route-3.0)
BuildRequires:  libXres-devel
BuildRequires:  lm_sensors-devel
BuildRequires:  zlib-devel

Obsoletes:      kf5-ksysguard < 5.1.95
Provides:       kf5-ksysguard = %{version}-%{release}

Requires:       %{name}-common = %{version}-%{release}

## upgrade path, https://bugzilla.redhat.com/show_bug.cgi?id=1963354
Conflicts: ksysguard-backend < 5.21.90

%description
KSysGuard library provides API to read and manage processes
running on the system.

%package        devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
# Requires:       cmake(Qt5Core)
Requires:       qt5-qtbase-devel
# Requires:       cmake(Qt5Network)
# Requires:       cmake(Qt5Widgets)
Requires:       cmake(KF5Config)
Requires:       cmake(KF5I18n)
Requires:       cmake(KF5IconThemes)
Obsoletes:      kf5-ksysguard-devel < 5.1.95
Provides:       kf5-ksysguard-devel = %{version}-%{release}
Conflicts:      kde-workspace-devel < 1:4.11.16-11

%package        common
Summary:        Runtime data files shared by libksysguard and ksysguard-libs
Conflicts:      libksysguard < 5.2.1-2
Conflicts:      ksysguard < 5.2
%description    common
%{summary}.

%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.


%prep
%autosetup -n %{name}-%{version} -p1


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} .. \
  -DKDE_INSTALL_INCLUDEDIR:PATH=%{_kf5_includedir}
popd
%make_build -C %{_target_platform}

%install
make install/fast  DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang ksysguard_qt5 --with-qt --with-kde --all-name


%ldconfig_scriptlets

%files -f ksysguard_qt5.lang
%license COPYING COPYING.LIB
# /usr/lib64
%{_kf5_libdir}/liblsofui.so.*
%{_kf5_libdir}/libprocessui.so.*
%{_kf5_libdir}/libprocesscore.so.*
%{_kf5_libdir}/libksignalplotter.so.*
%{_kf5_libdir}/libksgrd.so.*

# %%{_kf5_libdir}/libKSysGuardFormatter.so*
#
# %%{_kf5_libdir}/libKSysGuardSensors.so*
# %%{_kf5_libdir}/libKSysGuardSensorFaces.so*

%{_kf5_datadir}/ksysguard/*
# %%{_kf5_datadir}/qlogging-categories5/libksysguard.categories
# %%{_qt5_plugindir}/kpackage/packagestructure/sensorface_packagestructure.so
# %%{_qt5_plugindir}/designer/ksignalplotter5widgets.so
# %%{_qt5_plugindir}/designer/ksysguard*.so
# %%{_qt5_qmldir}/org/kde/ksysguard/*

# %%{_kf5_libdir}/libKSysGuardSystemStats.so.*
# %%{_qt5_plugindir}/ksysguard/
# %%{_libexecdir}/ksysguard/
%{_kf5_sysconfdir}/xdg/libksysguard.categories
# %%{_kf5_datadir}/dbus-1/interfaces/org.kde.ksystemstats.xml

%files common
%{_kf5_libexecdir}/kauth/ksysguardprocesslist_helper
%{_kf5_sysconfdir}/dbus-1/system.d/org.kde.ksysguard.processlisthelper.conf
%{_datadir}/dbus-1/system-services/org.kde.ksysguard.processlisthelper.service
%{_datadir}/polkit-1/actions/org.kde.ksysguard.processlisthelper.policy
# %%{_datadir}/knsrcfiles/*

%files devel
# /usr/include/KF5
%{_kf5_includedir}/ksysguard/*

%{_kf5_libdir}/liblsofui.so
%{_kf5_libdir}/libprocessui.so
%{_kf5_libdir}/libprocesscore.so
%{_kf5_libdir}/libksignalplotter.so
%{_kf5_libdir}/libksgrd.so
#%%{_kf5_libdir}/cmake/KSysGuard/

%{_kf5_libdir}/cmake/KF5SysGuard/

#%%{_kf5_libdir}/libKSysGuardSystemStats.so


%changelog

