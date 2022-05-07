Name:    kde-gtk-config
Summary: Configure the appearance of GTK apps in KDE
Version: 5.15.5
Release: 1%{?dist}

License: (GPLv2 or GPLv3) and (LGPLv2 or LGPLv3)
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/kde-gtk-config-%{version}.tar.xz
Patch0:  kde-gtk-config-hb.patch
## upstreamable patches

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-rpm-macros
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtsvg-devel

BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-kconfigwidgets-devel
BuildRequires:  kf5-knewstuff-devel
BuildRequires:  kf5-karchive-devel
BuildRequires:  kf5-kcmutils-devel
BuildRequires:  kf5-kdbusaddons-devel

BuildRequires:  gtk3-devel
BuildRequires:  gtk2-devel

# need kcmshell5 from kde-cli-tools
Requires:       kde-cli-tools

%if 0%{?fedora} > 23
Obsoletes:      kcm-gtk < 0.5.3-30
Obsoletes:      xsettings-kde < 0.12.3-30
%endif

%description
This is a System Settings configuration module for configuring the
appearance of GTK apps in KDE.


%prep
%setup -q -n kde-gtk-config-%{version} 
%patch0 -p1

%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5}  ..
popd

make -I/usr/include/harfbuzz %{?_smp_mflags} -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}
%find_lang kcmgtk5_qt --with-qt --all-name

%files -f kcmgtk5_qt.lang
# %%files
%license COPYING COPYING.LIB
%{_libexecdir}/reload_gtk_apps
%{_libexecdir}/gtk_preview
%{_libexecdir}/gtk3_preview
%{_qt5_plugindir}/kcm_kdegtkconfig.so
%{_sysconfdir}/xdg/cgcgtk3.knsrc
%{_sysconfdir}/xdg/cgctheme.knsrc
%{_datadir}/kcm-gtk-module/
%{_datadir}/icons/hicolor/
%{_datadir}/kservices5/kde-gtk-config.desktop
# %%{_libdir}/kconf_update_bin/gtk_theme
# %%{_datadir}/kconf_update/gtkconfig.upd

# %%{_qt5_plugindir}/kf5/kded/gtkconfig.so

%changelog

