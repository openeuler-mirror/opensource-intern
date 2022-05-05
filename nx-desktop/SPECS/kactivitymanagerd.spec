Name:           kactivitymanagerd
Summary:        Plasma service to manage user's activities
Version: 5.15.5
Release: 1%{?dist}

License:        GPLv2+
URL:            https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0:        http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/kactivitymanagerd-5.15.5.tar.xz
# filter plugin provides
%global __provides_exclude_from ^(%{_kf5_qtplugindir}/.*\\.so)$

BuildRequires:  kf5-rpm-macros
BuildRequires:  extra-cmake-modules
#BuildRequires:  cmake(Qt5Core)
#BuildRequires:  cmake(Qt5DBus)
BuildRequires:  qt5-qtbase-devel

BuildRequires:  cmake(KF5Crash)
BuildRequires:  cmake(KF5CoreAddons)
BuildRequires:  cmake(KF5Config)
BuildRequires:  cmake(KF5WindowSystem)
BuildRequires:  cmake(KF5GlobalAccel)
BuildRequires:  cmake(KF5XmlGui)
BuildRequires:  cmake(KF5KIO)
BuildRequires:  cmake(KF5DBusAddons)
BuildRequires:  cmake(KF5I18n)

BuildRequires:  boost-devel

# The kactivitymanagerd was split from KActivities in KF5 5.21,
# but thanks to our clever packaging kf5-kactivities package
# already contained only the kactivitymanagerd files
Obsoletes:      kf5-kactivities < 5.21.0

# older ones (previously in kf5-kactivities)
Obsoletes:      kactivities < 4.90.0
Provides:       kactivities%{?_isa} = %{version}-%{release}
Provides:       kactivities = %{version}-%{release}

%description
%{summary}.


%prep
%autosetup -n %{name}-%{version} -p1


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

make %{?_smp_mflags} -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang kactivities5 --with-qt

# unpackaged files
rm -fv %{buildroot}%{_kf5_qmldir}/org/kde/activities/{libkactivitiesextensionplugin.so,qmldir}


%files -f kactivities5.lang
%license COPYING*
%doc README.md
%{_sysconfdir}//xdg/kactivitymanagerd.categories
%{_libexecdir}/kactivitymanagerd
%{_kf5_libdir}/libkactivitymanagerd_plugin.so
%{_kf5_qtplugindir}/kactivitymanagerd/
%{_kf5_datadir}/dbus-1/services/org.kde.activitymanager.service
%{_kf5_datadir}/kservices5/kactivitymanagerd.desktop
%{_kf5_datadir}/kservicetypes5/kactivitymanagerd-plugin.desktop


%changelog

