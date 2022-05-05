 %global framework kjsembed

Name:    kf5-%{framework}
Version: 5.55.0
Release: 1%{?dist}
Summary: KDE Frameworks 5 Tier 3 addon for binding JS objects to QObjects

License: LGPLv2+
URL:     https://cgit.kde.org/%{framework}.git

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/frameworks/%{majmin}/portingAids/%{framework}-%{version}.tar.xz
# http://download.kde.org/stable/frameworks/5.55/portingAids/kjsembed-5.55.0.tar.xz
BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-kdoctools-devel >= %{majmin}
BuildRequires:  kf5-ki18n-devel >= %{majmin}
BuildRequires:  kf5-kjs-devel >= %{majmin}
BuildRequires:  kf5-rpm-macros
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtsvg-devel
BuildRequires:  qt5-qttools-static

%description
KSJEmbed provides a method of binding JavaScript objects to QObjects, so you
can script your applications.

%package        devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       kf5-kjs-devel >= %{majmin}
Requires:       kf5-ki18n-devel >= %{majmin}
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

%find_lang %{name} --all-name --with-man


%ldconfig_scriptlets

%files -f %{name}.lang
%doc README.md
%license COPYING.LIB
%{_kf5_bindir}/kjscmd5
%{_kf5_bindir}/kjsconsole
%{_kf5_libdir}/libKF5JsEmbed.so.*
%{_kf5_datadir}/man/man1/*

%files devel
%{_kf5_libdir}/libKF5JsEmbed.so
%{_kf5_libdir}/cmake/KF5JsEmbed/
%{_kf5_includedir}/KJsEmbed/
%{_kf5_archdatadir}/mkspecs/modules/qt_KJsEmbed.pri


%changelog

