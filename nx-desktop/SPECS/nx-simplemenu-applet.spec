Name:    nx-simplemenu-applet
Version: 3.9.6
Release: 1%{?dist}
Summary: Application menu launcher for NX Desktop

License: GPL-3.0+
URL:     https://github.com/nx-desktop/nx-simplemenu-applet.git
Source0: https://github.com/nx-desktop/%{name}.tar.gz


BuildRequires:  extra-cmake-modules
BuildRequires:  cmake
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-kactivities-devel
BuildRequires:  kf5-kdbusaddons-devel
BuildRequires:  kf5-kdeclarative-devel
BuildRequires:  kf5-kdelibs4support-devel
BuildRequires:  kf5-knotifications-devel
BuildRequires:  kf5-kpeople-devel
BuildRequires:  kf5-krunner-devel
BuildRequires:  kf5-solid-devel
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  plasma-workspace-devel
Requires: plasma-workspace

%description
Simple application menu launcher for NX Desktop.


%prep
%setup -q -n %{name}


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} -DLIB_INSTALL_DIR=lib  ../
popd

%make_build -C %{_target_platform}


%install
make install/fast  DESTDIR=%{buildroot} -C %{_target_platform}

%files
%{_kf5_datadir}/kservices5/plasma-applet-org.kde.plasma.nxmenu.desktop
%{_kf5_metainfodir}/org.kde.plasma.nxmenu.appdata.xml
%{_kf5_datadir}/plasma/plasmoids/org.kde.plasma.nxmenu/
%{_kf5_qmldir}/org/kde/plasma/private/nxmenu/libnxmenuplugin.so
%{_kf5_qmldir}/org/kde/plasma/private/nxmenu/qmldir

%changelog


