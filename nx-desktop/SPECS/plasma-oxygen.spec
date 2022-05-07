%global         base_name oxygen

Name:    plasma-%{base_name}
Version: 5.15.5
Release: 1%{?dist}
Summary: Plasma and Qt widget style and window decorations for Plasma 5 and KDE 4

License: GPLv2+
URL:     https://cgit.kde.org/%{base_name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global majmin_ver %(echo %{version} | cut -d. -f1,2).50
%global stable unstable
%else
%global majmin_ver %(echo %{version} | cut -d. -f1,2)
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{base_name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/oxygen-5.15.5.tar.xz
# filter plugins
%global __provides_exclude_from ^(%{_kde4_libdir}/kde4/.*\\.so|%{_kf5_qtplugindir}/.*\\.so)$

BuildRequires:  libxcb-devel

# Qt 5
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  qt5-qtdeclarative-devel

BuildRequires:  kf5-rpm-macros
BuildRequires:  extra-cmake-modules

# KF5
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kconfig-devel
BuildRequires:  kf5-kguiaddons-devel
BuildRequires:  kf5-kwidgetsaddons-devel
BuildRequires:  kf5-kservice-devel
BuildRequires:  kf5-kcompletion-devel
BuildRequires:  kf5-frameworkintegration-devel
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  kf5-kcmutils-devel
BuildRequires:  kf5-kwayland-devel

BuildRequires:  kdecoration-devel >= %{majmin}
Requires:       kf5-filesystem
Requires:       qt5-style-oxygen = %{version}-%{release}
Requires:       oxygen-cursor-themes = %{version}-%{release}
Requires:       oxygen-sound-theme = %{version}-%{release}
# for oxygen look-and-feel
Requires:       oxygen-icon-theme

# kwin-oxygen was removed in 5.1.95
Obsoletes:	kwin-oxygen < 5.1.95-1

%description
%{summary}.



%package -n     qt5-style-oxygen
Summary:        Oxygen widget style for Qt 5
Obsoletes:      plasma-oxygen < 5.1.1-2
Conflicts:      plasma-desktop < 5.16.90
%description -n qt5-style-oxygen
%{summary}.

%package -n     oxygen-cursor-themes
Summary:        Oxygen cursor themes
BuildArch:      noarch
Obsoletes:      plasma-oxygen-common < 5.1.1-2
%description -n oxygen-cursor-themes
%{summary}.

%package -n     oxygen-sound-theme
Summary:        Sounds for Oxygen theme
BuildArch:      noarch
Obsoletes:      plasma-oxygen-common < 5.1.1-2
%description -n oxygen-sound-theme
%{summary}.


%prep
%autosetup -n %{base_name}-%{version} -p1

%if ! 0%{?fedora}
sed -i.optional \
  -e 's| add_subdirectory(cursors)|#add_subdirectory(cursors)|' \
  -e 's| add_subdirectory(kdecoration)|#add_subdirectory(kdecoration)|' \
  CMakeLists.txt
%endif


%build

# Build for Qt 5
%global qt5_target_platform %{_target_platform}-qt5
mkdir %{qt5_target_platform}
pushd %{qt5_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{qt5_target_platform}

%install
%if 0%{?qt4}
make install/fast DESTDIR=%{buildroot} -C %{qt4_target_platform}
%endif
make install/fast DESTDIR=%{buildroot} -C %{qt5_target_platform}


## unpackaged files
# Don't bother with -devel subpackages, there are no headers anyway
rm -fv %{buildroot}%{_libdir}/liboxygenstyle5.so
rm -fv %{buildroot}%{_libdir}/liboxygenstyleconfig5.so
rm -fv %{buildroot}%{_kde4_libdir}/liboxygenstyle.so
rm -fv %{buildroot}%{_kde4_libdir}/liboxygenstyleconfig.so
%if ! 0%{?fedora}
rm -fv %{buildroot}%{_datadir}/locale/*/LC_MESSAGES/oxygen_kdecoration.mo
#rm -fv %{buildroot}%{_datadir}/sounds/Oxygen-*
rm -rfv %{buildroot}%{_datadir}/icons/{KDE_Classic,Oxygen_*}
rm -fv %{buildroot}%{_kf5_qtplugindir}/org.kde.kdecoration2/oxygendecoration.so
rm -fv %{buildroot}%{_kf5_datadir}/kservices5/oxygendecorationconfig.desktop
rm -rfv %{buildroot}%{_kf5_datadir}/plasma/look-and-feel/org.kde.oxygen/
%endif

%find_lang oxygen --with-qt --all-name


%if 0%{?fedora}
%files
%{_kf5_datadir}/plasma/look-and-feel/org.kde.oxygen/
%endif



%ldconfig_scriptlets -n qt5-style-oxygen

%files -n   qt5-style-oxygen -f oxygen.lang
%{_bindir}/oxygen-demo5
%{_bindir}/oxygen-settings5
%{_libdir}/liboxygenstyle5.so.*
%{_libdir}/liboxygenstyleconfig5.so.*
%{_kf5_qtplugindir}/styles/oxygen.so
%{_kf5_qtplugindir}/kstyle_oxygen_config.so
%if 0%{?fedora}
%{_kf5_qtplugindir}/org.kde.kdecoration2/oxygendecoration.so
%{_kf5_datadir}/kservices5/oxygendecorationconfig.desktop
%endif
%{_kf5_datadir}/kservices5/oxygenstyleconfig.desktop
%{_kf5_datadir}/kstyle/themes/oxygen.themerc
%{_kf5_datadir}/icons/hicolor/*/apps/oxygen-settings.*
#%{_kf5_datadir}/color-schemes/Oxygen.colors
#%{_kf5_datadir}/color-schemes/OxygenCold.colors

%if 0%{?fedora}
%files -n   oxygen-cursor-themes
%{_datadir}/icons/KDE_Classic/
%{_datadir}/icons/Oxygen_Black/
%{_datadir}/icons/Oxygen_Blue/
%{_datadir}/icons/Oxygen_White/
%{_datadir}/icons/Oxygen_Yellow/
%{_datadir}/icons/Oxygen_Zion/
%endif

%files -n   oxygen-sound-theme
%{_datadir}/sounds/Oxygen-*


%changelog

