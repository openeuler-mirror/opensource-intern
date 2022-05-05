
%global kf5_version 5.42.0

Name:    powerdevil
Version: 5.15.5
Release: 1%{?dist}
Summary: Manages the power consumption settings of a Plasma Shell

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
# http://download.kde.org/stable/plasma/5.15.5/powerdevil-5.15.5.tar.xz
## upstream patches

# filter plugin provides
%global __provides_exclude_from ^(%{_kf5_qtplugindir}/.*\\.so)$

# plasma deps
BuildRequires:  plasma-workspace-devel >= %{version}
Requires: libkworkspace5%{?_isa} >= %{version}

# kf5
BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-bluez-qt-devel
BuildRequires:  kf5-kactivities-devel
BuildRequires:  kf5-kauth-devel
BuildRequires:  kf5-kconfig-devel
BuildRequires:  kf5-kdelibs4support-devel
BuildRequires:  kf5-kglobalaccel-devel
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kidletime-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-knotifyconfig-devel
BuildRequires:  kf5-kwayland-devel
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  kf5-networkmanager-qt-devel >= %{kf5_version}
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-rpm-macros
BuildRequires:  kf5-solid-devel
BuildRequires:  libkscreen-qt5-devel

BuildRequires:  libxcb-devel
BuildRequires:  libXrandr-devel
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  systemd-devel
BuildRequires:  xcb-util-image-devel
BuildRequires:  xcb-util-keysyms-devel
BuildRequires:  xcb-util-wm-devel

%{?_qt5:Requires: %{_qt5}%{?_isa} >= %{_qt5_version}}

%description
Powerdevil is an utility for powermanagement. It consists
of a daemon (a KDED module) and a KCModule for its configuration.


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

%find_lang powerdevil5 --with-html --all-name

# Don't bother with -devel
rm %{buildroot}/%{_libdir}/libpowerdevil{configcommonprivate,core,ui}.so


%ldconfig_scriptlets

%files -f powerdevil5.lang
%license COPYING*
%{_sysconfdir}/dbus-1/system.d/org.kde.powerdevil.backlighthelper.conf
%{_sysconfdir}/dbus-1/system.d/org.kde.powerdevil.discretegpuhelper.conf
%{_datadir}/dbus-1/system-services/org.kde.powerdevil.backlighthelper.service
%{_datadir}/dbus-1/system-services/org.kde.powerdevil.discretegpuhelper.service
%{_datadir}/polkit-1/actions/org.kde.powerdevil.backlighthelper.policy
%{_datadir}/polkit-1/actions/org.kde.powerdevil.discretegpuhelper.policy
%{_kf5_libexecdir}/kauth/backlighthelper
%{_kf5_libexecdir}/kauth/discretegpuhelper
%{_sysconfdir}/xdg/autostart/powerdevil.desktop
%{_libexecdir}/org_kde_powerdevil
%{_kf5_libdir}/libpowerdevilconfigcommonprivate.so.*
%{_kf5_libdir}/libpowerdevilcore.so.*
%{_kf5_libdir}/libpowerdevilui.so.*
%{_kf5_qtplugindir}/*.so
%{_kf5_plugindir}/powerdevil/
%{_kf5_datadir}/knotifications5/powerdevil.notifyrc
%{_kf5_datadir}/kservices5/*.desktop
%{_kf5_datadir}/kservicetypes5/*.desktop


%changelog

