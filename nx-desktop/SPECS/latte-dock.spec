
Name:     latte-dock
Version:  0.10.7
Release:  1%{?dist}
Summary:  Latte is a dock based on plasma frameworks

License:  GPLv2+
URL:      https://invent.kde.org/plasma/%{name}
Source0:  https://invent.kde.org/plasma/%{name}/-/archive/v%{version}/latte-dock-v%{version}.tar.gz
# https://invent.kde.org/plasma/latte-dock
BuildRequires:  libxcb-devel
BuildRequires:  xcb-util-devel
BuildRequires:  libSM-devel
BuildRequires:  extra-cmake-modules
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  kf5-karchive-devel
BuildRequires:  kf5-kactivities-devel
BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-kdbusaddons-devel
BuildRequires:  kf5-kdeclarative-devel
BuildRequires:  kf5-knewstuff-devel
BuildRequires:  kf5-knotifications-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kpackage-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-kwayland-devel
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  kf5-kxmlgui-devel
BuildRequires:  kf5-kglobalaccel-devel
BuildRequires:  kf5-kguiaddons-devel
BuildRequires:  kf5-kcrash-devel

%description
Latte is a dock based on plasma frameworks that provides an elegant and
intuitive experience for your tasks and plasmoids. It animates its contents by
using parabolic zoom effect and tries to be there only when it is needed.

"Art in Coffee"

%prep
%setup -q -n %{name}-v%{version}

%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}

%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}
find %{buildroot} -size 0 -delete

%files
%{_bindir}/latte-dock
%{_datadir}/metainfo/org.kde.latte-dock.appdata.xml
%{_datadir}/metainfo/org.kde.latte.plasmoid.appdata.xml
%{_datadir}/metainfo/org.kde.latte.shell.appdata.xml
%{_kf5_datadir}/applications/org.kde.latte-dock.desktop
%{_kf5_datadir}/dbus-1/interfaces/org.kde.LatteDock.xml
%{_kf5_datadir}/icons/breeze/*/*/*
%{_kf5_datadir}/icons/hicolor/*/*/*
%{_kf5_datadir}/knotifications5/lattedock.notifyrc
%{_kf5_datadir}/kservices5/plasma-applet-org.kde.latte.containment.desktop
%{_kf5_datadir}/kservices5/plasma-applet-org.kde.latte.plasmoid.desktop
%{_kf5_datadir}/kservices5/plasma-shell-org.kde.latte.shell.desktop
%{_kf5_datadir}/kservices5/plasma-containmentactions-lattecontextmenu.desktop
%{_kf5_datadir}/kservicetypes5/latte-indicator.desktop
%{_kf5_datadir}/plasma/plasmoids/org.kde.latte.containment/
%{_kf5_datadir}/plasma/plasmoids/org.kde.latte.plasmoid/
%{_kf5_datadir}/plasma/shells/org.kde.latte.shell/
%{_kf5_datadir}/latte
%{_kf5_qmldir}/org/kde/latte
%{_qt5_plugindir}/plasma_containmentactions_lattecontextmenu.so
%{_qt5_plugindir}/kpackage/packagestructure/latte_packagestructure_indicator.so
%{_qt5_settingsdir}/latte-layouts.knsrc
%{_sysconfdir}/xdg/latte-indicators.knsrc

%changelog

