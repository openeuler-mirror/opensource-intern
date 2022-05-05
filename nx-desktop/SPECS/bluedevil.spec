Name:    bluedevil
Summary: Bluetooth stack for KDE
Version: 5.15.5
Release: 1%{?dist}

License: GPLv2+
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
#Source1: http://download.kde.org/stable/plasma/5.15.5/bluedevil-5.15.5.tar.xz.sig

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtdeclarative-devel

BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-kwidgetsaddons-devel
BuildRequires:  kf5-kdbusaddons-devel
BuildRequires:  kf5-knotifications-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-kdeclarative-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kio-devel
# 5.11 is when kf5-bluez-qt became Framework and changed API
BuildRequires:  kf5-bluez-qt-devel >= 5.11
BuildRequires:  kf5-kded-devel
BuildRequires:  kf5-kwindowsystem-devel

BuildRequires:  shared-mime-info

BuildRequires:  desktop-file-utils

Provides:       dbus-bluez-pin-helper

Obsoletes:      kbluetooth < 0.4.2-3
Obsoletes:      bluedevil-devel < 2.0.0-0.10

Requires:       bluez >= 5
Requires:       kf5-kded
Requires:       pulseaudio-module-bluetooth

# When -autostart was removed
Obsoletes:      bluedevil-autostart < 5.2.95

%description
BlueDevil is the bluetooth stack for KDE.


%prep
%autosetup -n %{name}-%{version} -p1


%build
mkdir build
cd build
%{cmake_kf5} ..
cd ../
make %{?_smp_mflags} -C build

%install
make install DESTDIR=%{buildroot} -C build

%find_lang %{name} --all-name --with-html


%check
desktop-file-validate %{buildroot}%{_kf5_datadir}/applications/org.kde.bluedevilsendfile.desktop
desktop-file-validate %{buildroot}%{_kf5_datadir}/applications/org.kde.bluedevilwizard.desktop


%files -f %{name}.lang
%doc README
%license COPYING COPYING.LIB
# /usr/bin
%{_kf5_bindir}/bluedevil-sendfile
%{_kf5_bindir}/bluedevil-wizard

#/usr/lib64/qt5/plugins
%{_kf5_qtplugindir}/kcm_*.so
%{_kf5_qtplugindir}/kio_*.so
%{_kf5_qtplugindir}/kf5/kded/*.so

# /usr/share
%{_kf5_datadir}/plasma/plasmoids/org.kde.plasma.bluetooth/contents/ui/*.qml
%{_kf5_datadir}/plasma/plasmoids/org.kde.plasma.bluetooth/contents/ui/logic.js
%{_kf5_datadir}/plasma/plasmoids/org.kde.plasma.bluetooth/metadata.*

# /usr/lib64/qt5/plugins/kf5
%{_kf5_plugindir}/kded/*.so

%{_kf5_datadir}/remoteview/bluetooth-network.desktop

%{_kf5_datadir}/kservices5/*.desktop
%{_kf5_datadir}/kservices5/*.protocol
%{_kf5_datadir}/knotifications5/bluedevil.notifyrc

%{_kf5_datadir}/applications/org.kde.bluedevilsendfile.desktop
%{_kf5_datadir}/applications/org.kde.bluedevilwizard.desktop

%{_kf5_datadir}/bluedevilwizard/
%{_kf5_datadir}/plasma/plasmoids/org.kde.plasma.bluetooth
# /usr/lib64/qt5/qml
%{_kf5_qmldir}/org/kde/plasma/private/bluetooth/

# /usr/share/metainfo
%{_kf5_metainfodir}/org.kde.plasma.bluetooth.appdata.xml

%{_datadir}/mime/packages/*.xml


%changelog

