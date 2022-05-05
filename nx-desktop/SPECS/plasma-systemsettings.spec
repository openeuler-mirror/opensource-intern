%global  base_name systemsettings

Name:    plasma-%{base_name}
Summary: KDE System Settings application
Version: 5.15.5
Release: 1%{?dist}

License: GPLv2+
URL:     https://cgit.kde.org/%{base_name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global majmin_ver %(echo %{version} | cut -d. -f1,2).50
%global stable unstable
%else
%global majmin_ver %(echo %{version} | cut -d. -f1,2)
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{base_name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/systemsettings-5.15.5.tar.xz
BuildRequires: desktop-file-utils

BuildRequires: extra-cmake-modules
BuildRequires: kf5-rpm-macros
BuildRequires: cmake(KF5Crash)
BuildRequires: cmake(KF5ItemViews)
BuildRequires: cmake(KF5KCMUtils)
BuildRequires: cmake(KF5I18n)
BuildRequires: cmake(KF5KIO)
BuildRequires: cmake(KF5Service)
BuildRequires: cmake(KF5IconThemes)
BuildRequires: cmake(KF5WidgetsAddons)
BuildRequires: cmake(KF5WindowSystem)
BuildRequires: cmake(KF5XmlGui)
BuildRequires: cmake(KF5DBusAddons)
BuildRequires: cmake(KF5Config)
BuildRequires: cmake(KF5DocTools)
BuildRequires: cmake(KF5Package)
BuildRequires: cmake(KF5Declarative)
BuildRequires: cmake(KF5Activities)
BuildRequires: cmake(KF5ActivitiesStats)
BuildRequires: cmake(KF5KHtml)
BuildRequires: cmake(KF5ItemModels)

#BuildRequires: cmake(Qt5Qml)
BuildRequires: 	qt5-qtdeclarative-devel
#BuildRequires: cmake(Qt5Quick)
#BuildRequires: cmake(Qt5QuickWidgets)
#BuildRequires: cmake(Qt5Widgets)
BuildRequires: 	qt5-qtbase-devel

BuildRequires: cmake(KF5Kirigami2)
BuildRequires: kf5-kirigami2-devel >= 2.1
BuildRequires: plasma-workspace-devel >= %{version}
Requires:      libkworkspace5%{?_isa} >= %{majmin_ver}
Requires:      kf5-kirigami2%{?_isa} >= 2.1

# kde-cli-tools provides kcmshell5, which is not directly needed by
# systemsettings, but is an addition expected by users
Requires:       kde-cli-tools

# https://bugzilla.redhat.com/show_bug.cgi?id=1268493
# doc/HTML/en/systemsettings conflicts
Conflicts: kde-workspace < 5.0

# /usr/share/kservices5/settings-system-administration.desktop file conflict
Conflicts: kcm_systemd < 1.2.1-15

%description
%{summary}.

%package        devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.


%prep
%autosetup -n %{base_name}-%{version} -p1


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

make %{?_smp_mflags} -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang systemsettings5 --with-qt --with-html --all-name


%check
desktop-file-validate %{buildroot}%{_datadir}/applications/kdesystemsettings.desktop
desktop-file-validate %{buildroot}%{_datadir}/applications/systemsettings.desktop


%ldconfig_scriptlets

%files -f systemsettings5.lang
%license COPYING*
%{_bindir}/systemsettings5
%{_libdir}/libsystemsettingsview.so.*
%{_kf5_qtplugindir}/*.so
%{_datadir}/systemsettings/
%{_datadir}/applications/kdesystemsettings.desktop
%{_datadir}/applications/systemsettings.desktop
#%%{_datadir}/metainfo/org.kde.systemsettings.metainfo.xml
%{_kf5_datadir}/kservices5/*.desktop
%{_kf5_datadir}/kservicetypes5/*.desktop
%{_kf5_datadir}/kxmlgui5/systemsettings
%dir %{_kf5_datadir}/kpackage/genericqml/
%{_kf5_datadir}/kpackage/genericqml/org.kde.systemsettings.*/
%{_sysconfdir}/xdg/systemsettings.categories
#%%{_kf5_datadir}/qlogging-categories5/systemsettings.categories


%files devel
%{_includedir}/systemsettingsview/
%{_libdir}/libsystemsettingsview.so


%changelog

