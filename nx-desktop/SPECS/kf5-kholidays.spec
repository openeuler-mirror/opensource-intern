%global framework kholidays

# uncomment to enable bootstrap mode
#global bootstrap 1

%if !0%{?bootstrap}
%global tests 1
%endif

Name:    kf5-%{framework}
Epoch:   1
Version: 5.55.0
Release: 1%{?dist}
Summary: The KHolidays Library

License: LGPLv2+ and GPLv3+
URL:     https://cgit.kde.org/%{framework}.git

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/frameworks/%{majmin}/%{framework}-%{version}.tar.xz
# http://download.kde.org/stable/frameworks/5.55/kholidays-5.55.0.tar.xz

BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-rpm-macros

# qt5-qtbase-devel
BuildRequires:  pkgconfig(Qt5Core)
# qt5-qtdeclarative-devel
BuildRequires:  pkgconfig(Qt5Qml)
BuildRequires:  qt5-qttools-static
#BuildRequires:  cmake(Qt5LinguistTools)

# translations moved here
Conflicts: kde-l10n < 17.03

%description
The KHolidays library provides a C++ API that determines holiday
and other special events for a geographical region.

%package        devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{?epoch:%{epoch}:}%{version}-%{release}
%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.


%prep
%autosetup -n %{framework}-%{version} -p1


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} .. \
  -DBUILD_TESTING:BOOL=%{?tests:ON}%{!?tests:OFF}
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang_kf5 libkholidays5_qt


%check
%if 0%{?tests}
export CTEST_OUTPUT_ON_FAILURE=1
xvfb-run -a \
dbus-launch --exit-with-session \
time \
make test ARGS="--output-on-failure --timeout 20" -C %{_target_platform} ||:
%endif


%ldconfig_scriptlets

%files -f libkholidays5_qt.lang
%license COPYING*
%{_kf5_libdir}/libKF5Holidays.so.*
%{_kf5_qmldir}/org/kde/kholidays/

%files devel
%{_kf5_includedir}/kholidays_version.h
%{_kf5_includedir}/KHolidays/
%{_kf5_libdir}/libKF5Holidays.so
%{_kf5_libdir}/cmake/KF5Holidays/
%{_kf5_archdatadir}/mkspecs/modules/qt_KHolidays.pri


%changelog


