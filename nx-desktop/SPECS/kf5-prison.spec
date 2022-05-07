%global framework prison

Name:    kf5-%{framework}
Summary: KDE Frameworks 5 Tier 1 barcode library
Version: 5.55.0
Release: 1%{?dist}

License: BSD
URL:     https://cgit.kde.org/%{framework}.git

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/frameworks/%{majmin}/%{framework}-%{version}.tar.xz

BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-rpm-macros >= %{majmin}

BuildRequires:  pkgconfig(Qt5Gui)

#BuildRequires:  pkgconfig(libdmtx)
BuildRequires:  libdmtx-devel
#BuildRequires:  pkgconfig(libqrencode)
BuildRequires:  qrencode-devel

Requires: kf5-filesystem >= %{majmin}

%description
Prison is a Qt-based barcode abstraction layer/library that provides
an uniform access to generation of barcodes with data.

%package        devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
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
%doc README*
%license LICENSE
%{_kf5_sysconfdir}/xdg/%{framework}.*
%{_kf5_libdir}/libKF5Prison.so.5*
%{_kf5_qmldir}/org/kde/prison/
%files devel
%{_kf5_includedir}/prison_version.h
%{_kf5_includedir}/prison/
%{_kf5_libdir}/libKF5Prison.so
%{_kf5_libdir}/cmake/KF5Prison/
%{_kf5_archdatadir}/mkspecs/modules/qt_Prison.pri


%changelog

