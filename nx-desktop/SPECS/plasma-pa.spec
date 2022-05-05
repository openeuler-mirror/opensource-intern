Name:    plasma-pa
Version: 5.15.5
Release: 1%{?dist}
Summary: Plasma applet for audio volume management using PulseAudio

License: LGPLv2+ and GPLv2+
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/plasma-pa-5.15.5.tar.xz
BuildRequires:  extra-cmake-modules
BuildRequires:  glib2-devel
BuildRequires:  kde-filesystem
BuildRequires:  kf5-kconfigwidgets-devel
BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-kdeclarative-devel
BuildRequires:  kf5-kdoctools-devel
BuildRequires:  kf5-kglobalaccel-devel
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kpackage-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
#BuildRequires:  pkgconfig(gconf-2.0)
BuildRequires:  GConf2-devel
BuildRequires:  pkgconfig(libcanberra)
BuildRequires:  perl-generators
BuildRequires:  pulseaudio-libs-devel
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  cmake(KF5Notifications)
%if ! 0%{?bootstrap}
# required kpackage plugins (-packagestructure comes from plasma-workspace)
BuildRequires:  plasma-packagestructure
%endif

Requires: pulseaudio
# support systemsettings->multimedia->audio volume->advanced->automatically switch streams when a new output becomes available
Recommends: pulseaudio-module-gconf%{?_isa}

%description
%{summary}.


%prep
%autosetup


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast  DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang %{name} --all-name --with-html

## unpackaged files
rm -rfv %{buildroot}%{_kde4_appsdir}/kconf_update/


%files -f %{name}.lang
%license COPYING*
%{_kf5_datadir}/plasma/plasmoids/org.kde.plasma.volume/
%{_kf5_qmldir}/org/kde/plasma/private/volume/
%{_kf5_qtplugindir}/kcms/kcm_pulseaudio.so
%{_kf5_datadir}/kservices5/plasma-applet-org.kde.plasma.volume.desktop
%{_kf5_datadir}/kconf_update/*
%{_kf5_datadir}/kpackage/kcms/kcm_pulseaudio
%{_kf5_datadir}/kservices5/kcm_pulseaudio.desktop
%{_kf5_metainfodir}/org.kde.plasma.volume.appdata.xml


%changelog

