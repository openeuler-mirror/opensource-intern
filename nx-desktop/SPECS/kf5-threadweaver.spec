 %global framework threadweaver

Name:           kf5-%{framework}
Version: 5.55.0
Release: 1%{?dist}
Summary:        KDE Frameworks 5 Tier 1 addon for advanced thread management

License:        LGPLv2+
URL:            https://cgit.kde.org/%{framework}.git

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0:        http://download.kde.org/%{stable}/frameworks/%{majmin}/%{framework}-%{version}.tar.xz
#  http://download.kde.org/stable/frameworks/5.55/threadweaver-5.55.0.tar.xz

BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-rpm-macros >= %{majmin}
BuildRequires:  qt5-qtbase-devel

Requires:       kf5-filesystem >= %{majmin}

%description
KDE Frameworks 5 Tier 1 addon for advanced thread management.

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
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}


%ldconfig_scriptlets

%files
%doc README.md
%license COPYING.LIB
%{_kf5_libdir}/libKF5ThreadWeaver.so.*

%files devel
%{_kf5_includedir}/ThreadWeaver/
%{_kf5_includedir}/threadweaver_version.h
%{_kf5_libdir}/libKF5ThreadWeaver.so
%{_kf5_libdir}/cmake/KF5ThreadWeaver/
%{_kf5_archdatadir}/mkspecs/modules/qt_ThreadWeaver.pri


%changelog

