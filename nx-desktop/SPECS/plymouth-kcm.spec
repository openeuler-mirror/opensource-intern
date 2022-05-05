 
%global base_name    plymouth-kcm

Name:    plymouth-kcm
Summary: Plymouth configuration module for systemsettings
Version: 5.15.5
Release: 1%{?dist}

License: GPLv2+
URL:     https://cgit.kde.org/%{base_name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0:        http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/plymouth-kcm-5.15.5.tar.xz
# Patch1:         0001-fedora.patch

# filter plugin provides
%global __provides_exclude_from ^(%%{_kf5_qtplugindir}/.*\\.so)$

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-rpm-macros
BuildRequires:  kf5-kcmutils
BuildRequires:  qt5-qtbase-devel
BuildRequires:  plymouth-devel

#BuildRequires:  cmake(Qt5Quick)
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  cmake(KF5Archive)
BuildRequires:  cmake(KF5NewStuff)
BuildRequires:  cmake(KF5NewStuffCore)
BuildRequires:  cmake(KF5KIO)
BuildRequires:  cmake(KF5Declarative)
BuildRequires:  cmake(KF5I18n)
BuildRequires:  cmake(KF5Config)
BuildRequires:  cmake(KF5ConfigWidgets)

Requires:   plymouth

%description
This is a System Settings configuration module for configuring the
plymouth splash screen.


%prep
%setup -q -n %{name}-%{version}

#%%patch1 -p1 -b .fedora


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang kcm_plymouth --all-name --with-html


%files -f kcm_plymouth.lang
%license COPYING
# %%{_kf5_datadir}/dbus-1/system.d/org.kde.kcontrol.kcmplymouth.conf
%{_sysconfdir}/dbus-1/system.d/org.kde.kcontrol.kcmplymouth.conf
%{_sysconfdir}/xdg/plymouth.knsrc
%{_bindir}/kplymouththemeinstaller
%{_kf5_qtplugindir}/kcms/kcm_plymouth.so
%{_kf5_libexecdir}/kauth/plymouthhelper
%{_datadir}/dbus-1/system-services/org.kde.kcontrol.kcmplymouth.service
%{_datadir}/kpackage/kcms/kcm_plymouth/
%{_datadir}/kservices5/kcm_plymouth.desktop
%{_datadir}/polkit-1/actions/org.kde.kcontrol.kcmplymouth.policy


%changelog
