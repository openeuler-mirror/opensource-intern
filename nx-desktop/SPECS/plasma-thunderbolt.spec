%global base_name    plasma-thunderbolt

Name:    plasma-thunderbolt
Summary: Plasma integration for controlling Thunderbolt devices
Version: 5.17.5
Release: 1%{?dist}

License: GPLv2+ and BSD
URL:     https://cgit.kde.org/%{base_name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0:        http://download.kde.org/%{stable}/plasma/%{version}/%{base_name}-%{version}.tar.xz
#  http://download.kde.org/stable/plasma/5.18.4/plasma-thunderbolt-5.18.4.tar.xz
BuildRequires:  gcc-c++

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-rpm-macros
BuildRequires:  cmake(KF5CoreAddons)
BuildRequires:  cmake(KF5DBusAddons)
BuildRequires:  cmake(KF5Declarative)
BuildRequires:  cmake(KF5I18n)
BuildRequires:  cmake(KF5KCMUtils)
BuildRequires:  cmake(KF5Notifications)

# BuildRequires:  cmake(Qt5DBus)
BuildRequires:	qt5-qtbase-devel
#BuildRequires:  cmake(Qt5Core)
#BuildRequires:  cmake(Qt5Quick)
BuildRequires:	qt5-qtdeclarative-devel

Requires:       bolt

%description
Plasma Sytem Settings module and a KDED module to handle authorization of
Thunderbolt devices connected to the computer. There's also a shared library
(libkbolt) that implements common interface between the modules and the
system-wide bolt daemon, which does the actual hard work of talking to the
kernel.


%prep
%autosetup -n %{base_name}-%{version} -p1


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
%make_install -C %{_target_platform}

%find_lang %{name} --all-name

%ldconfig_scriptlets


%files -f %{name}.lang
%doc README.md
%license COPYING
%{_kf5_libdir}/libkbolt.so
%{_kf5_qtplugindir}/kcms/kcm_bolt.so
%{_kf5_qtplugindir}/kf5/kded/kded_bolt.so
%{_kf5_datadir}/knotifications5/kded_bolt.notifyrc
%dir %{_kf5_datadir}/kpackage/kcms/kcm_bolt
%{_kf5_datadir}/kpackage/kcms/kcm_bolt/*
%{_kf5_datadir}/kservices5/kcm_bolt.desktop


%changelog

