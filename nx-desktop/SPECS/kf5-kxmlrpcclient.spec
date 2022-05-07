%global framework kxmlrpcclient

Name:    kf5-%{framework}
Version: 5.55.0
Release: 1%{?dist}
Summary: KDE Frameworks 5 Tier 3 library for interaction with XML RPC services

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
# http://download.kde.org/stable/frameworks/5.55/kxmlrpcclient-5.55.0.tar.xz
BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-ki18n-devel >= %{majmin}
BuildRequires:  kf5-kio-devel >= %{majmin}
BuildRequires:  kf5-rpm-macros
BuildRequires:  qt5-qtbase-devel

%description
KDE Frameworks 5 Tier 3 library for interaction with XML RPC services.

%package        devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       kf5-kio-devel >= %{majmin}
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

%find_lang %{name} --all-name


%ldconfig_scriptlets

%files -f %{name}.lang
%doc README.md
%license COPYING.BSD
%{_kf5_sysconfdir}/xdg/%{framework}.*
%{_kf5_libdir}/libKF5XmlRpcClient.so.*

%files devel
%{_kf5_includedir}/kxmlrpcclient_version.h
%{_kf5_includedir}/KXmlRpcClient/
%{_kf5_libdir}/libKF5XmlRpcClient.so
%{_kf5_libdir}/cmake/KF5XmlRpcClient/
%{_kf5_archdatadir}/mkspecs/modules/qt_KXmlRpcClient.pri


%changelog

