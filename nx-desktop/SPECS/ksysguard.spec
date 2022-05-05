Name:    ksysguard
Version: 5.15.5
Release: 1%{?dist}
Summary: KDE Process Management application

License: GPLv2
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global majmin_ver %(echo %{version} | cut -d. -f1,2).50
%global stable unstable
%else
%global majmin_ver %(echo %{version} | cut -d. -f1,2)
%global stable stable
%endif
Source0:        http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/ksysguard-5.15.5.tar.xz
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtscript-devel
BuildRequires:  qt5-qtwebkit-devel

BuildRequires:  kf5-rpm-macros
BuildRequires:  extra-cmake-modules

BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kitemviews-devel
BuildRequires:  kf5-knewstuff-devel
BuildRequires:  kf5-kconfig-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-kdelibs4support-devel
BuildRequires:  kf5-kdoctools-devel

BuildRequires:  libksysguard-devel >= %{majmin_ver}

BuildRequires:  lm_sensors-devel
BuildRequires:  desktop-file-utils

Requires:       ksysguardd = %{version}-%{release}

%description
%{summary}.

%package -n    ksysguardd
Summary: Performance monitor daemon
%description -n ksysguardd
%{summary}.


%prep
%autosetup


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

make %{?_smp_mflags} -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}
%find_lang ksysguard5 --with-html --with-qt --all-name


%check
desktop-file-validate %{buildroot}%{_datadir}/applications/org.kde.ksysguard.desktop

%files -f ksysguard5.lang
%license COPYING*
%doc README
%{_bindir}/ksysguard
%{_kf5_libdir}/libkdeinit5_ksysguard.so
%{_datadir}/ksysguard
%{_datadir}/metainfo/org.kde.ksysguard.appdata.xml
%config %{_sysconfdir}/xdg/ksysguard.knsrc
%{_datadir}/applications/org.kde.ksysguard.desktop
%{_datadir}/icons/hicolor/*/apps/*
%{_kf5_datadir}/knotifications5/ksysguard.notifyrc
%{_kf5_datadir}/kxmlgui5/ksysguard/

%files -n ksysguardd
%license COPYING
%{_bindir}/ksysguardd
%config %{_sysconfdir}/ksysguarddrc


%changelog

