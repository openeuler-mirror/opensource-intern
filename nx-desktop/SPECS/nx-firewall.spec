Name:    nx-firewall
Version: 3.9.6
Release: 1%{?dist}
Summary: NX Firewall - Protect your workstation!

License: GPL-3.0+
URL:     https://github.com/nx-desktop/nx-firewall.git
Source0: https://github.com/nx-desktop/%{name}.tar.gz
Patch0:  nx-firewall-werror.patch
BuildRequires:  extra-cmake-modules
BuildRequires:  cmake
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  kf5-kdeclarative-devel
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  kf5-karchive-devel
BuildRequires:  kf5-knewstuff-devel
BuildRequires:  kf5-kauth-devel
BuildRequires:  kf5-kconfig-devel
BuildRequires:  kf5-kconfigwidgets-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-kcmutils-devel
BuildRequires:  kf5-kdelibs4support-devel
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  netstat-nat
Requires: plasma-workspace
Requires: python3 >= 3.2
Requires: ufw

%description
NX Firewall allows you to protect your workstation from
network atacks and also to avoid your data to be exposed
on the network by an imprudent application.

%prep
%setup -q -n %{name}
%patch0 -p1

%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} -DLIB_INSTALL_DIR=lib  ../
popd

make -C %{_target_platform}


%install
make install/fast  DESTDIR=%{buildroot} -C %{_target_platform}

%files
%{_sysconfdir}/dbus-1/system.d/
%{_qt5_plugindir}/kcms/org.nxos.firewall.so
%{_qt5_qmldir}/org/nomad/
%{_libexecdir}/
%{_kf5_datadir}/

%changelog

