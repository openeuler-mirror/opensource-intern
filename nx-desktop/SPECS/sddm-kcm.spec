Name:    sddm-kcm
Version: 5.15.5
Release: 1%{?dist}
Summary: SDDM KDE configuration module

License: GPLv2+
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/sddm-kcm-5.15.5.tar.xz
# filter plugin provides
%global __provides_exclude_from ^(%{_kf5_qtplugindir}/.*\\.so)$

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-rpm-macros
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  qt5-qttools-devel
BuildRequires:  qt5-qtdeclarative-devel

BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kxmlgui-devel
BuildRequires:  kf5-kauth-devel
BuildRequires:  kf5-kconfigwidgets-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-knewstuff-devel
BuildRequires:  kf5-karchive-devel

BuildRequires:  libX11-devel
BuildRequires:  libXcursor-devel
BuildRequires:  libxcb-devel
BuildRequires:  xcb-util-image-devel

Requires:       sddm


%description
This is a System Settings configuration module for configuring the
SDDM Display Manager

%prep
%setup -q -n %{name}-%{version}


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

make %{?_smp_mflags} -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}
%find_lang kcmsddm5_qt --with-qt --all-name


%files -f kcmsddm5_qt.lang
%license COPYING
%{_kf5_bindir}/sddmthemeinstaller
%{_kf5_qtplugindir}/kcm_sddm.so
%{_kf5_datadir}/kservices5/kcm_sddm.desktop
%{_kf5_datadir}/sddm-kcm
%{_kf5_libexecdir}/kauth/kcmsddm_authhelper
%{_sysconfdir}/dbus-1/system.d/org.kde.kcontrol.kcmsddm.conf
#%%{_datadir}/knsrcfiles/sddmtheme.knsrc
%{_sysconfdir}/xdg/sddmtheme.knsrc
%{_datadir}/dbus-1/system-services/org.kde.kcontrol.kcmsddm.service
%{_datadir}/polkit-1/actions/org.kde.kcontrol.kcmsddm.policy


%changelog

