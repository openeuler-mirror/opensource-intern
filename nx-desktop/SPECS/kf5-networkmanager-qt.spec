%global framework networkmanager-qt

Name:    kf5-%{framework}
Version: 5.55.0
Release: 1%{?dist}
Summary: A Tier 1 KDE Frameworks 5 module that wraps NetworkManager DBus API

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
# http://download.kde.org/stable/frameworks/5.55/networkmanager-qt-5.55.0.tar.xz
BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-rpm-macros >= %{majmin}
BuildRequires:  qt5-qtbase-devel

BuildRequires:  pkgconfig(libnm)

%if 0%{?fedora} > 22
Recommends:     NetworkManager
%else
Requires:       NetworkManager >= 0.9.9.0
%endif
Requires:       kf5-filesystem >= %{majmin}

%description
A Tier 1 KDE Frameworks 5 Qt library for NetworkManager.

%package devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       qt5-qtbase-devel
Requires:       pkgconfig(libnm)
%description    devel
Qt libraries and header files for developing applications
that use NetworkManager.


%prep
%autosetup -p1 -n %{framework}-%{version}


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast  DESTDIR=%{buildroot} -C %{_target_platform}


%ldconfig_scriptlets

%files
%doc README.md
%license COPYING.LIB
%{_kf5_sysconfdir}/xdg/%{framework}.*
%{_kf5_libdir}/libKF5NetworkManagerQt.so.*

%files devel
%{_kf5_libdir}/libKF5NetworkManagerQt.so
%{_kf5_libdir}/cmake/KF5NetworkManagerQt/
%{_kf5_includedir}/NetworkManagerQt/
%{_kf5_includedir}/networkmanagerqt_version.h
%{_kf5_archdatadir}/mkspecs/modules/qt_NetworkManagerQt.pri


%changelog 

