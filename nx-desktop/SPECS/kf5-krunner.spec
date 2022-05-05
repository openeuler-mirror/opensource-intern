%global framework krunner

Name:    kf5-%{framework}
Version: 5.55.0
Release: 1%{?dist}
Summary: KDE Frameworks 5 Tier 3 solution with parallelized query system

License: LGPLv2+ and BSD
URL:     https://cgit.kde.org/%{framework}.git

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/frameworks/%{majmin}/%{framework}-%{version}.tar.xz
# http://download.kde.org/stable/frameworks/5.55/krunner-5.55.0.tar.xz
# filter qml provides
%global __provides_exclude_from ^%{_kf5_qmldir}/.*\\.so$

BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-rpm-macros

BuildRequires:  kf5-kconfig-devel >= %{majmin}
BuildRequires:  kf5-kcoreaddons-devel >= %{majmin}
BuildRequires:  kf5-ki18n-devel >= %{majmin}
BuildRequires:  kf5-kio-devel >= %{majmin}
BuildRequires:  kf5-kservice-devel >= %{majmin}
BuildRequires:  kf5-plasma-devel >= %{majmin}
BuildRequires:  kf5-solid-devel >= %{majmin}
BuildRequires:  kf5-threadweaver-devel >= %{majmin}

BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtdeclarative-devel

%description
KRunner provides a parallelized query system extendable via plugins.

%package        devel
Summary:        Development files for %{name}
# krunner template moved here
Conflicts:      kapptemplate < 16.03.80
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       kf5-plasma-devel >= %{majmin}
Requires:       qt5-qtbase-devel
%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.


%prep
%autosetup -n %{framework}-%{version} -p1


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}


%ldconfig_scriptlets

%files
%doc README.md
%license COPYING.LIB
%{_kf5_sysconfdir}/xdg/%{framework}*
%{_kf5_libdir}/libKF5Runner.so.*
%{_kf5_qmldir}/org/kde/runnermodel/
%{_kf5_datadir}/kservicetypes5/plasma-runner.desktop

%files devel
%{_kf5_includedir}/krunner_version.h
%{_kf5_includedir}/KRunner/
%{_kf5_libdir}/libKF5Runner.so
%{_kf5_libdir}/cmake/KF5Runner/
%{_kf5_archdatadir}/mkspecs/modules/qt_KRunner.pri
%dir %{_kf5_datadir}/kdevappwizard/
%dir %{_kf5_datadir}/kdevappwizard/templates/
%{_kf5_datadir}/kdevappwizard/templates/runner*
%{_kf5_datadir}/dbus-1/interfaces/*


%changelog

