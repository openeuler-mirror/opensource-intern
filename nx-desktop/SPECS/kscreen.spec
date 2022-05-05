Name:    kscreen
Epoch:   1
Version: 5.14.5
Release: 1%{?dist}
Summary: KDE Display Management software

# KDE e.V. may determine that future GPL versions are accepted
License: GPLv2 or GPLv3
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
# http://download.kde.org/stable/plasma/5.15.5/kscreen-5.15.5.tar.xz
# filter plugin provides
%global __provides_exclude_from ^(%{_kf5_qtplugindir}/.*\\.so)$

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-rpm-macros
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  qt5-qtsensors-devel

BuildRequires:  libkscreen-qt5-devel >= %{majmin_ver}
Requires:       libkscreen-qt5%{?_isa} >= %{majmin_ver}

BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-kconfigwidgets-devel
BuildRequires:  kf5-kdbusaddons-devel
BuildRequires:  kf5-kxmlgui-devel
BuildRequires:  kf5-kglobalaccel-devel
BuildRequires:  cmake(KF5Declarative)
BuildRequires:  cmake(KF5IconThemes)
BuildRequires:  cmake(KF5Plasma)
BuildRequires:  cmake(KF5KCMUtils)

Requires:       qt5-qtgraphicaleffects

%description
KCM and KDED modules for managing displays in KDE.


%prep
%autosetup -p1 -n %{name}-%{version}


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang %{name} --with-kde --all-name


%files -f %{name}.lang
%license COPYING
%{_bindir}/kscreen-console
%{_kf5_qtplugindir}/plasma/applets/plasma_applet_kscreen.so
# %%{_kf5_qtplugindir}/kcms/kcm_kscreen.so
%{_kf5_plugindir}/kded/kscreen.so
%{_qt5_plugindir}/kcm_kscreen.so
%{_datadir}/kservices5/plasma-applet-org.kde.kscreen.desktop
%{_datadir}/metainfo/org.kde.kscreen.appdata.xml
%{_datadir}/plasma/plasmoids/org.kde.kscreen/contents/ui/PresentationModeItem.qml
%{_datadir}/plasma/plasmoids/org.kde.kscreen/contents/ui/ScreenLayoutSelection.qml
%{_datadir}/plasma/plasmoids/org.kde.kscreen/contents/ui/main.qml
%{_datadir}/plasma/plasmoids/org.kde.kscreen/metadata.desktop
%{_datadir}/plasma/plasmoids/org.kde.kscreen/metadata.json
%{_kf5_datadir}/kded_kscreen/qml/Osd.qml
%{_kf5_datadir}/kded_kscreen/qml/OsdItem.qml
%{_kf5_datadir}/kded_kscreen/qml/OsdSelector.qml
%{_kf5_datadir}/kded_kscreen/qml/OutputIdentifier.qml
%{_kf5_datadir}/kservices5/kcm_kscreen.desktop
#%%{_datadir}/icons/hicolor/*/actions/*
#%%{_kf5_datadir}/qlogging-categories5/kscreen.categories
%{_sysconfdir}/xdg/kscreen.categories
%{_kf5_datadir}/icons/hicolor/48x48/actions/kdocumentinfo.png
%{_kf5_datadir}/icons/hicolor/scalable/actions/kdocumentinfo.svgz
%{_kf5_datadir}/kcm_kscreen/qml/*.qml

# %%{_kf5_datadir}/kpackage/kcms/kcm_kscreen/contents/ui/Orientation.qml
# %%{_kf5_datadir}/kpackage/kcms/kcm_kscreen/contents/ui/Output.qml
# %%{_kf5_datadir}/kpackage/kcms/kcm_kscreen/contents/ui/OutputIdentifier.qml
# %%{_kf5_datadir}/kpackage/kcms/kcm_kscreen/contents/ui/OutputPanel.qml
# %%{_kf5_datadir}/kpackage/kcms/kcm_kscreen/contents/ui/Panel.qml
# %%{_kf5_datadir}/kpackage/kcms/kcm_kscreen/contents/ui/RotationButton.qml
# %%{_kf5_datadir}/kpackage/kcms/kcm_kscreen/contents/ui/Screen.qml
# %%{_kf5_datadir}/kpackage/kcms/kcm_kscreen/contents/ui/main.qml
# %%{_kf5_datadir}/kpackage/kcms/kcm_kscreen/metadata.desktop
# %%{_kf5_datadir}/kpackage/kcms/kcm_kscreen/metadata.json


%changelog

