%global framework modemmanager-qt

Name:    kf5-%{framework}
Version: 5.55.0
Release: 1%{?dist}
Summary: A Tier 1 KDE Frameworks module wrapping ModemManager DBus API

License: LGPLv2+
URL:     https://cgit.kde.org/%{framework}.git

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/frameworks/%{majmin}/modemmanager-qt-%{version}.tar.xz
# http://download.kde.org/stable/frameworks/5.55/modemmanager-qt-5.55.0.tar.xz
BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-rpm-macros >= %{majmin}

BuildRequires:  ModemManager-devel >= 1.0.0
BuildRequires:  qt5-qtbase-devel

Requires:       kf5-filesystem >= %{majmin}

%description
A Qt 5 library for ModemManager.

%package        devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       ModemManager-devel
Requires:       qt5-qtbase-devel
%description    devel
Qt 5 libraries and header files for developing applications
that use ModemManager.


%prep
%autosetup -n %{framework}-%{version}


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
%doc README README.md
%license COPYING.LIB
#%%{_kf5_datadir}/qlogging-categories5/*.categories
%{_kf5_libdir}/libKF5ModemManagerQt.so.*
%{_sysconfdir}/xdg/modemmanager-qt.categories
%{_kf5_libdir}/qt5/mkspecs/modules/qt_ModemManagerQt.pri

%files devel
%{_kf5_libdir}/libKF5ModemManagerQt.so
%{_kf5_libdir}/cmake/KF5ModemManagerQt/
%{_kf5_includedir}/ModemManagerQt/
%{_kf5_includedir}/modemmanagerqt_version.h
#{_kf5_archdatadir}/mkspecs/modules/qt_ModemManagerQt.pri


%changelog

