# Set (as 1) to enable bootstrap when building plasma-workspace on a new
# repo or arch where there's no package that would provide plasmashell
%global bootstrap 1

%global kf5_version_min 5.50.0
%global kf5_version 5.55.0

Name:    plasma-workspace
Summary: Plasma workspace, applications and applets
Version: 5.15.5
Release: 1%{?dist}

License: GPLv2+
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global majmin_ver %(echo %{version} | cut -d. -f1,2).50
%global stable unstable
%else
%global majmin_ver %(echo %{version} | cut -d. -f1,2)
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz

# filter qml/plugins provides
%global __provides_exclude_from ^(%{_kf5_qmldir}/.*\\.so|%{_kf5_qtplugindir}/.*\\.so)$

# This goes to PAM
# TODO: this should arguably be in kde-settings with the other pam-related configs
Source10:       kde
#Source15:       fedora.desktop

# breeze fedora sddm theme components
# includes f25-based preview (better than breeze or nothing at least)
#Source20:       breeze-fedora-0.2.tar.gz

## downstream Patches
#Patch100:       plasma-workspace-5.12.5-konsole-in-contextmenu.patch
#Patch101:       plasma-workspace-5.3.0-set-fedora-default-look-and-feel.patch
# remove stuff we don't want or need, plus a minor bit of customization --rex
#Patch102:       startkde.patch
# default to folderview (instead of desktop) containment, see also
# https://mail.kde.org/pipermail/distributions/2016-July/000133.html
# and example,
# https://github.com/notmart/artwork-lnf-netrunner-core/blob/master/usr/share/plasma/look-and-feel/org.kde.netrunner-core.desktop/contents/defaults
#Patch105:       plasma-workspace-5.7.3-folderview_layout.patch

## upstreamable Patches

## upstream Patches lookaside cache

## upstream Patches (master branch)

# udev
BuildRequires:  zlib-devel
BuildRequires:  dbusmenu-qt5-devel
BuildRequires:  libGL-devel
BuildRequires:  mesa-libGLES-devel
BuildRequires:  libSM-devel
BuildRequires:  libX11-devel
BuildRequires:  libXau-devel
BuildRequires:  libXdmcp-devel
BuildRequires:  libxkbfile-devel
BuildRequires:  libXcomposite-devel
BuildRequires:  libXdamage-devel
BuildRequires:  libXrender-devel
BuildRequires:  libXfixes-devel
BuildRequires:  libXrandr-devel
BuildRequires:  libXcursor-devel
BuildRequires:  libXtst-devel
BuildRequires:  libxcb-devel
BuildRequires:  xcb-util-keysyms-devel
BuildRequires:  xcb-util-image-devel
BuildRequires:  xcb-util-renderutil-devel
BuildRequires:  xcb-util-wm-devel
BuildRequires:  xcb-util-devel
BuildRequires:  glib2-devel
BuildRequires:  fontconfig-devel
BuildRequires:  boost-devel
BuildRequires:  libusb-devel
BuildRequires:  libbsd-devel
BuildRequires:  pam-devel
BuildRequires:  lm_sensors-devel
BuildRequires:  pciutils-devel
%ifnarch s390 s390x
BuildRequires:  libraw1394-devel
%endif
BuildRequires:  gpsd-devel
BuildRequires:  libqalculate-devel
%global kf5_pim 1
BuildRequires:  kf5-kholidays-devel
BuildRequires:  kf5-prison-devel

BuildRequires:  qt5-qtbase-devel >= 5.7.0
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  qt5-qtscript-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  qt5-qtwebkit-devel
BuildRequires:  phonon-qt5-devel

BuildRequires:  kf5-rpm-macros >= %{kf5_version_min}
BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-baloo-devel >= %{kf5_version_min}
BuildRequires:  kf5-kactivities-stats-devel >= %{kf5_version_min}
BuildRequires:  kf5-kcmutils-devel >= %{kf5_version_min}
BuildRequires:  kf5-kcrash-devel >= %{kf5_version_min}
BuildRequires:  kf5-kdeclarative-devel >= %{kf5_version_min}
BuildRequires:  kf5-kdelibs4support-devel >= %{kf5_version_min}
BuildRequires:  kf5-kdesu-devel >= %{kf5_version_min}
BuildRequires:  kf5-kdewebkit-devel >= %{kf5_version_min}
BuildRequires:  kf5-kdoctools-devel >= %{kf5_version_min}
BuildRequires:  kf5-kglobalaccel-devel >= %{kf5_version_min}
BuildRequires:  kf5-kidletime-devel >= %{kf5_version_min}
BuildRequires:  kf5-kinit-devel >= %{kf5_version_min}
BuildRequires:  kf5-kjsembed-devel >= %{kf5_version_min}
BuildRequires:  kf5-knewstuff-devel >= %{kf5_version_min}
BuildRequires:  kf5-knotifyconfig-devel >= %{kf5_version_min}
BuildRequires:  kf5-kpeople-devel >= %{kf5_version_min}
BuildRequires:  kf5-krunner-devel >= %{kf5_version_min}
BuildRequires:  kf5-ktexteditor-devel >= %{kf5_version_min}
BuildRequires:  kf5-ktextwidgets-devel >= %{kf5_version_min}
BuildRequires:  kf5-kwallet-devel >= %{kf5_version_min}
BuildRequires:  kf5-kxmlrpcclient-devel >= %{kf5_version_min}
BuildRequires:  kf5-networkmanager-qt-devel >= %{kf5_version_min}
BuildRequires:  kf5-plasma-devel >= %{kf5_version_min}
Requires:       kf5-plasma%{?_isa} >= %{kf5_version}
BuildRequires:  kf5-threadweaver-devel >= %{kf5_version_min}
BuildRequires:  kf5-kded-devel >= %{kf5_version_min}

BuildRequires:  kf5-ksysguard-devel >= %{majmin_ver}
BuildRequires:  kf5-kwayland-devel >= %{kf5_version_min}
BuildRequires:  wayland-devel >= 1.3.0
BuildRequires:  libkscreen-qt5-devel >= %{majmin_ver}
BuildRequires:  kscreenlocker-devel >= %{majmin_ver}

BuildRequires:  kwin-devel >= %{majmin_ver}

BuildRequires:  chrpath
BuildRequires:  desktop-file-utils

# Optional
BuildRequires:  kf5-kactivities-devel
%if 0%{?fedora}
BuildRequires:  cmake(AppStreamQt) >= 0.10.4
%endif

# when kded_desktopnotifier.so moved here
Conflicts:      kio-extras < 5.4.0

%if 0%{?fedora} || 0%{?rhel} > 7
Recommends:     %{name}-geolocation = %{version}-%{release}
Suggests:       imsettings-qt
%else
Requires:       %{name}-geolocation = %{version}-%{release}
%endif

Requires:       %{name}-common = %{version}-%{release}
Requires:       %{name}-libs%{?_isa} = %{version}-%{release}
Requires:       libkworkspace5%{?_isa} = %{version}-%{release}

# for libkdeinit5_*
%{?kf5_kinit_requires}
Requires:       kactivitymanagerd >= %{majmin_ver}
Requires:       khotkeys >= %{majmin_ver}
Requires:       kf5-kded
Requires:       kf5-kdoctools
Requires:       qt5-qtquickcontrols
Requires:       qt5-qtgraphicaleffects
Requires:       kf5-filesystem
Requires:       kf5-baloo
Requires:       kf5-kglobalaccel >= 5.7
Requires:       kf5-kxmlrpcclient
#Requires:       kf5-kquickcharts

# systemmonitor dataengine
Requires:       ksysguardd >= %{majmin_ver}

# The new volume control for PulseAudio
%if 0%{?fedora} || 0%{?rhel} > 7
Requires:       plasma-pa
%endif

# Without the platformtheme plugins we get broken fonts
Requires:       kf5-frameworkintegration

# For krunner
Requires:       plasma-milou >= %{majmin_ver}

# powerdevil has a versioned dep on libkworkspace5, so (may?)
# need to avoid this dep when bootstrapping
%if ! 0%{?bootstrap}
# Power management
Requires:       powerdevil >= %{majmin_ver}
%endif

# startkde
Requires:       coreutils
Requires:       dbus-x11
Requires:       socat
Requires:       xmessage
Requires:       qt5-qttools

Requires:       xorg-x11-utils
Requires:       xorg-x11-server-utils

Requires:       kde-settings-plasma


Requires:       systemd

# Oxygen
# TODO: review if oxygen-fonts, oxygen-icon-theme are still needed (I suspect not) -- rex
#Requires:       oxygen-icon-theme
Requires:       oxygen-sound-theme >= %{majmin_ver}
#Requires:       oxygen-fonts

# PolicyKit authentication agent
Requires:        polkit-kde >= %{majmin_ver}

# Require any plasmashell (plasma-desktop provides plasmashell(desktop))
%if 0%{?bootstrap}
#Provides:       plasmashell = %%{version}
%else
# Note: We should require >= %%{version}, but that creates a circular dependency
# at build time of plasma-desktop, because it provides the needed dependency, but
# also needs plasma-workspace to build. So for now the dependency is unversioned.
#Requires:       plasmashell >= %%{majmin_ver}
%endif

# when -common, libkworkspace5 was split out
Obsoletes:      plasma-workspace < 5.4.2-2

# plasmashell provides dbus service org.freedesktop.Notifications
Provides: desktop-notification-daemon

# upgrade path, when sddm-breeze was split out
Obsoletes: plasma-workspace < 5.3.2-8

# digitalclock applet
%if ! 0%{?bootstrap}
BuildRequires: pkgconfig(iso-codes)
%endif
Requires: iso-codes

%description
Plasma 5 libraries and runtime components

%package common
Summary: Common files for %{name}
%description common
%{name}.

%package -n libkworkspace5
Summary: Runtime libkworkspace5 library
# when spilt occurred
Obsoletes: plasma-workspace < 5.4.2-2
Requires:  %{name}-common = %{version}-%{release}
%description -n libkworkspace5
%{summary}.

%package libs
Summary: Runtime libraries for %{name}
# when split out
Obsoletes: plasma-workspace < 5.4.2-2
## omit dep on main pkg for now, means we can avoid pulling in a
## huge amount of deps (including kde4) into buildroot -- rex
#Requires:  %%{name}%%{?_isa} = %%{version}-%%{release}
Requires:  %{name}-common = %{version}-%{release}
# consider splitting out plasma_packagestructure content later
Provides: plasma-packagestructure = %{version}-%{release}
%description libs
%{summary}.

%package        devel
Summary:        Development files for %{name}
Requires:       %{name}-libs%{?_isa} = %{version}-%{release}
Requires:       libkworkspace5%{?_isa} = %{version}-%{release}
%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.

%package        doc
Summary:        Documentation and user manuals for %{name}
License:        GFDL
# switch to noarch
Obsoletes:      plasma-workspace-doc < 5.3.1-2
Requires:       %{name}-common = %{version}-%{release}
BuildArch: noarch
%description    doc
Documentation and user manuals for %{name}.

%package geolocation
Summary: Plasma5 geolocation components
# when split out
Obsoletes: plasma-workspace < 5.4.2-2
Requires: %{name}-geolocation-libs%{?_isa} = %{version}-%{release}
%description geolocation
%{summary}.

%package geolocation-libs
Summary: Plasma5 geolocation runtime libraries
Requires: %{name}-common = %{version}-%{release}
Requires: %{name}-geolocation = %{version}-%{release}
%description geolocation-libs
%{summary}.

%package -n sddm-breeze
Summary:        SDDM breeze theme
# upgrade path, when sddm-breeze was split out
Obsoletes: plasma-workspace < 5.3.2-8
Requires:       kf5-plasma >= %{kf5_version}
# Background.qml:import QtQuick
Requires:       qt5-qtquickcontrols
# on-screen keyboard
Recommends:     qt5-qtvirtualkeyboard
# QML imports:
# org.kde.plasma.workspace.components
# org.kde.plasma.workspace.keyboardlayout
Requires:       %{name} = %{version}-%{release}
# /usr/share/backgrounds/default.png
%if 0%{?fedora}
BuildRequires:  desktop-backgrounds-compat
Requires:       desktop-backgrounds-compat
%endif
%if 0%{?rhel}
Requires:       system-logos
%endif
BuildArch: noarch
%description -n sddm-breeze
%{summary}.

%package wayland
Summary:        Wayland support for Plasma
Requires:       %{name} = %{version}-%{release}
Requires:       kwin-wayland >= %{majmin_ver}
Requires:       kwayland-integration%{?_isa} >= %{majmin_ver}
Requires:       xorg-x11-server-Xwayland
Requires:       qt5-qtwayland%{?_isa}
# startplasmacompositor deps
Requires:       qt5-qttools
%description wayland
%{summary}.

%package -n plasma-lookandfeel-fedora
Summary:  Fedora look-and-feel for Plasma
Requires: %{name} = %{version}-%{release}
# when switched to noarch
Obsoletes: plasma-lookandfeel-fedora < 5.8.0-5
# https://bugzilla.redhat.com/show_bug.cgi?id=1356890
Obsoletes: f22-kde-theme < 22.4
Obsoletes: f23-kde-theme < 23.1
Obsoletes: f24-kde-theme < 24.6
Obsoletes: f24-kde-theme-core < 5.10.5-2
BuildArch: noarch
%description -n plasma-lookandfeel-fedora
%{summary}.


%prep
#%%setup -q -a 20
%setup -q -n %{name}-%{version}

## upstream patches

#%%patch100 -p1 -b .konsole-in-contextmenu
# FIXME/TODO:  it is unclear whether this is needed or even a good idea anymore -- rex
%if 0%{?default_lookandfeel:1}
sed -i -e "s|@DEFAULT_LOOKANDFEEL@|%{?default_lookandfeel}%{!?default_lookandfeel:org.kde.breeze.desktop}|g" \
  shell/packageplugins/lookandfeel/lookandfeel.cpp
%endif





# highlight the use of wayland
#sed -i.plasmawayland -e "s|Plasma|Plasma (Wayland)|g" login-sessions/plasmawayland.desktop.cmake


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}

%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

chrpath --delete %{buildroot}%{_kf5_qtplugindir}/phonon_platform/kde.so

# compat symlink
#ln -s startplasma-x11 %{buildroot}%{_kf5_bindir}/startkde

# make fedora-breeze sddm theme variant.

# Make kcheckpass work
install -m644 -p -D %{SOURCE10} %{buildroot}%{_sysconfdir}/pam.d/kde

%find_lang all --with-html --all-name

grep "%{_kf5_docdir}" all.lang > %{name}-doc.lang
grep libkworkspace.mo all.lang > libkworkspace5.lang
# any translations not used elsewhere, include in main pkg
cat *.lang | sort | uniq -u > %{name}.lang


%check
desktop-file-validate %{buildroot}%{_kf5_datadir}/applications/plasma-windowed.desktop
desktop-file-validate %{buildroot}%{_kf5_datadir}/applications/org.kde.{klipper,plasmashell,systemmonitor}.desktop


%files common
%license COPYING
%license COPYING.DOC
%license COPYING.LIB

%files -f %{name}.lang
%{_kf5_bindir}/gmenudbusmenuproxy
%{_kf5_bindir}/kcminit
%{_kf5_bindir}/kcminit_startup
%{_kf5_bindir}/klipper
%{_kf5_bindir}/krunner
%{_kf5_bindir}/ksmserver
%{_kf5_bindir}/ksplashqml
%{_kf5_bindir}/plasmashell
%{_kf5_bindir}/plasmawindowed
#%%{_kf5_bindir}/plasma_session
%{_kf5_bindir}/plasma_waitforname
%{_kf5_bindir}/startkde
#%%{_kf5_bindir}/startplasma-x11
%{_kf5_bindir}/systemmonitor
%{_kf5_bindir}/xembedsniproxy

%{_kf5_bindir}/kcheckrunning
%{_kf5_bindir}/kdostartupconfig5
%{_kf5_bindir}/kstartupconfig5
%{_kf5_bindir}/kuiserver5
%{_kf5_bindir}/startplasmacompositor

%{_kf5_libdir}/libkdeinit5_*.so




%{_kf5_qmldir}/org/kde/*
%{_libexecdir}/baloorunner
%{_libexecdir}/ksmserver-logout-greeter
%{_libexecdir}/ksyncdbusenv
%{_kf5_datadir}/ksplash/
%{_kf5_datadir}/plasma/plasmoids/
%{_kf5_datadir}/plasma/services/
%{_kf5_datadir}/plasma/wallpapers/
%dir %{_kf5_datadir}/plasma/look-and-feel/
%{_kf5_datadir}/plasma/look-and-feel/org.kde.breeze.desktop/
%{_kf5_datadir}/solid/
%{_kf5_datadir}/kstyle/
%{_sysconfdir}/xdg/autostart/*.desktop
%{_sysconfdir}/xdg/
%{_datadir}/desktop-directories/*.directory
%{_datadir}/dbus-1/services/*.service
#%%{_datadir}/knsrcfiles/*.knsrc
%{_kf5_datadir}/kservices5/*.desktop
%{_kf5_datadir}/kservices5/*.protocol
%{_kf5_datadir}/kservicetypes5/*.desktop
%{_kf5_datadir}/knotifications5/*.notifyrc
%{_kf5_datadir}/config.kcfg/*
%{_kf5_datadir}/kio_desktop/
%{_kf5_datadir}/kconf_update/krunnerplugins.upd
%{_kf5_libdir}/kconf_update_bin/krunnerplugins
%{_kf5_metainfodir}/*.xml
%{_kf5_datadir}/applications/org.kde.klipper.desktop
%{_kf5_datadir}/applications/org.kde.plasmashell.desktop
%{_kf5_datadir}/applications/plasma-windowed.desktop
%{_kf5_datadir}/applications/org.kde.systemmonitor.desktop
%{_datadir}/xsessions/plasma.desktop
%{_kf5_bindir}/plasma_waitforname
#%%{_kf5_datadir}/qlogging-categories5/*.categories
#%%{_sysconfdir}/xdg/plasmanotifyrc
%{_kf5_datadir}/kpackage/kcms/kcm_translations/*
# PAM
%config(noreplace) %{_sysconfdir}/pam.d/kde
%exclude %{_kf5_datadir}/kservices5/plasma-dataengine-geolocation.desktop
%exclude %{_kf5_datadir}/kservices5/plasma-geolocation-gps.desktop
%exclude %{_kf5_datadir}/kservices5/plasma-geolocation-ip.desktop
%exclude %{_kf5_datadir}/kservicetypes5/plasma-geolocationprovider.desktop

%files doc -f %{name}-doc.lang

%ldconfig_scriptlets -n libkworkspace5

%files -n libkworkspace5 -f libkworkspace5.lang
%{_libdir}/libkworkspace5.so.5*

%ldconfig_scriptlets libs

%files libs
%{_sysconfdir}/xdg/taskmanagerrulesrc
%{_libdir}/libcolorcorrect.so.*
%{_libdir}/libtaskmanager.so.*
%{_libdir}/libweather_ion.so.*
#%%{_libdir}/libnotificationmanager.*
# multilib'able plugins
%{_kf5_qtplugindir}/plasma/applets/
%{_kf5_qtplugindir}/plasma/dataengine/
%if 0%{?kf5_pim}
%{_kf5_qtplugindir}/plasmacalendarplugins/
%endif
%{_kf5_qtplugindir}/*.so
%exclude %{_kf5_qtplugindir}/plasma-geolocation-gps.so
%exclude %{_kf5_qtplugindir}/plasma-geolocation-ip.so
%exclude %{_kf5_qtplugindir}/plasma/dataengine/plasma_engine_geolocation.so
%dir %{_kf5_qtplugindir}/phonon_platform/
%{_kf5_qtplugindir}/phonon_platform/kde.so
%{_kf5_qtplugindir}/kpackage/packagestructure/*.so
%{_kf5_qtplugindir}/plasma/packagestructure/plasma_packagestructure_share.so
%{_kf5_plugindir}/kio/*.so
%{_kf5_plugindir}/kded/*.so
%{_qt5_plugindir}/kcms/kcm_translations.so
%{_libexecdir}/startplasma
%{_datadir}/plasma/shareprovider/
#%%{_libdir}/kconf_update_bin/krunnerglobalshortcuts
#%%{_kf5_qtplugindir}/plasma/containmentactions/plasma_containmentactions_applauncher.so
#%%{_kf5_qtplugindir}/plasma/containmentactions/plasma_containmentactions_contextmenu.so
#%%{_kf5_qtplugindir}/plasma/containmentactions/plasma_containmentactions_paste.so
#%%{_kf5_qtplugindir}/plasma/containmentactions/plasma_containmentactions_switchdesktop.so
#%%{_kf5_qtplugindir}/plasma/containmentactions/plasma_containmentactions_switchwindow.so
#%%{_libexecdir}/plasma-sourceenv.sh
#%%{_libexecdir}/startplasma-waylandsession
#%%{_datadir}/kconf_update/krunnerglobalshortcuts.upd
#%%{_datadir}/kglobalaccel/krunner.desktop


%files geolocation
%{_kf5_qtplugindir}/plasma-geolocation-gps.so
%{_kf5_qtplugindir}/plasma-geolocation-ip.so
%{_kf5_qtplugindir}/plasma/dataengine/plasma_engine_geolocation.so
%{_kf5_datadir}/kservices5/plasma-dataengine-geolocation.desktop
%{_kf5_datadir}/kservices5/plasma-geolocation-gps.desktop
%{_kf5_datadir}/kservices5/plasma-geolocation-ip.desktop
%{_kf5_datadir}/kservicetypes5/plasma-geolocationprovider.desktop

%ldconfig_scriptlets geolocation-libs

%files geolocation-libs
%{_libdir}/libplasma-geolocation-interface.so.5*

%files devel
%{_libdir}/libcolorcorrect.so
%{_libdir}/libweather_ion.so
%{_libdir}/libtaskmanager.so
%{_libdir}/libplasma-geolocation-interface.so
%{_libdir}/libkworkspace5.so
%dir %{_includedir}/plasma/
%{_includedir}/colorcorrect/
%{_includedir}/plasma/weather/
%{_includedir}/kworkspace5/
%{_includedir}/plasma/geolocation/
%{_includedir}/taskmanager/
#%%{_includedir}/notificationmanager/
%{_libdir}/cmake/KRunnerAppDBusInterface/
%{_libdir}/cmake/KSMServerDBusInterface/
%{_libdir}/cmake/LibColorCorrect
%{_libdir}/cmake/LibKWorkspace/
%{_libdir}/cmake/LibTaskManager/
#%%{_libdir}/cmake/LibNotificationManager/
%{_datadir}/dbus-1/interfaces/*.xml
%{_datadir}/kdevappwizard/templates/ion-dataengine.tar.bz2

%files -n sddm-breeze
%{_datadir}/sddm/themes/breeze/
#%%{_datadir}/sddm/themes/01-breeze-fedora/
#%%config(noreplace) %%{_datadir}/sddm/themes/01-breeze-fedora/theme.conf.user

%files wayland
#%%{_kf5_bindir}/startplasma-wayland
%{_datadir}/wayland-sessions/plasmawayland.desktop



%changelog

