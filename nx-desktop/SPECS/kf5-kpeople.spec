 %global framework kpeople

Name:    kf5-%{framework}
Version: 5.55.0
Release: 1%{?dist}
Summary: KDE Frameworks 5 Tier 3 library for contact and people aggregation

License: LGPLv2+
URL:     https://cgit.kde.org/%{framework}.git

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/frameworks/%{majmin}/%{framework}-%{version}.tar.xz
# http://download.kde.org/stable/frameworks/5.55/kpeople-5.55.0.tar.xz
# filter qml provides
%global __provides_exclude_from ^%{_kf5_qmldir}/.*\\.so$

BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-kcoreaddons-devel >= %{majmin}
BuildRequires:  kf5-ki18n-devel >= %{majmin}
BuildRequires:  kf5-kitemviews-devel >= %{majmin}
BuildRequires:  kf5-kservice-devel >= %{majmin}
BuildRequires:  kf5-kwidgetsaddons-devel >= %{majmin}
BuildRequires:  kf5-rpm-macros
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtdeclarative-devel

%description
KDE Frameworks 5 Tier 3 library for interaction with XML RPC services.

%package        devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       qt5-qtbase-devel
%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.


%prep
%autosetup -n %{framework}-%{version}


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} .. \
    -DENABLE_EXAMPLES:BOOL=OFF
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang %{name} --all-name


%ldconfig_scriptlets

%files -f %{name}.lang
%doc README.md
%license COPYING
%{_kf5_sysconfdir}/xdg/%{framework}.*
%{_kf5_libdir}/libKF5People.so.*
%{_kf5_libdir}/libKF5PeopleWidgets.so.*
%{_kf5_libdir}/libKF5PeopleBackend.so.*
%{_kf5_datadir}/kf5/kpeople/
%{_kf5_datadir}/kservicetypes5/*.desktop
%{_kf5_qmldir}/org/kde/people/

%files devel
%{_kf5_includedir}/KPeople/
%{_kf5_libdir}/libKF5People.so
%{_kf5_libdir}/libKF5PeopleWidgets.so
%{_kf5_libdir}/libKF5PeopleBackend.so
%{_kf5_libdir}/cmake/KF5People/
%{_kf5_archdatadir}/mkspecs/modules/qt_KPeople.pri
%{_kf5_archdatadir}/mkspecs/modules/qt_KPeopleWidgets.pri


%changelog
