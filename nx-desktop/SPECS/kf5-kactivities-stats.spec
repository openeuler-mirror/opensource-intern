%global framework kactivities-stats

Name:           kf5-%{framework}
Summary:        A KDE Frameworks 5 Tier 3 library for accessing the usage data collected by the activities system
Version: 5.55.0
Release: 1%{?dist}

# KDE e.V. may determine that future GPL versions are accepted
License:        LGPLv2 or LGPLv3
URL:            https://cgit.kde.org/%{framework}.git

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0:        http://download.kde.org/%{stable}/frameworks/%{majmin}/%{framework}-%{version}.tar.xz
# http://download.kde.org/stable/frameworks/5.55/kactivities-stats-5.55.0.tar.xz

## upstream patches

BuildRequires:  boost-devel
BuildRequires:  extra-cmake-modules >= %{version}
BuildRequires:  kf5-kactivities-devel >= %{version}
BuildRequires:  kf5-kconfig-devel >= %{version}
BuildRequires:  kf5-rpm-macros >= %{version}
BuildRequires:  pkgconfig
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  qt5-qtbase-devel

%description
%{summary}.

%package devel
Summary:  Developer files for %{name}
Requires: %{name}%{?_isa} = %{version}-%{release}
Requires: qt5-qtbase-devel
%description devel
%{summary}.


%prep
%autosetup -n %{framework}-%{version} -p1


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} .. \
  -DBUILD_TESTING=ON
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}


# Currently includes no tests
%check
make test -C %{_target_platform}


%ldconfig_scriptlets

%files
%doc MAINTAINER README.developers TODO
%license COPYING*
%{_kf5_libdir}/libKF5ActivitiesStats.so.*

%files devel
%{_kf5_libdir}/libKF5ActivitiesStats.so
%{_kf5_includedir}/KActivitiesStats/
%{_kf5_includedir}/kactivitiesstats_version.h
%{_kf5_libdir}/cmake/KF5ActivitiesStats/
%{_kf5_libdir}/pkgconfig/libKActivitiesStats.pc
%{_qt5_archdatadir}/mkspecs/modules/qt_KActivitiesStats.pri

