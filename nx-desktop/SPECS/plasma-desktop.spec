
%global kf5_version_min 5.42
%global kf5_version 5.55
%global synaptics 1
%global scim 1
%if 0%{?rhel} && 0%{?rhel} > 7
%global synaptics 0
%global scim 0
%endif

Name:    plasma-desktop
Summary: Plasma Desktop shell
Version: 5.15.5
Release: 1%{?dist}

License: GPLv2+ and (GPLv2 or GPLv3)
URL:     https://cgit.kde.org/%{name}.git

%global majmin_ver %(echo %{version} | cut -d. -f1,2)
Source0: http://download.kde.org/stable/plasma/%{version}/%{name}-%{version}.tar.xz



# filter qmk/plugins provides
%global __provides_exclude_from ^(%{_kf5_qmldir}/.*\\.so|%{_kf5_qtplugindir}/.*\\.so)$

BuildRequires:  libusb-devel
BuildRequires:  fontconfig-devel
BuildRequires:  libX11-devel
BuildRequires:  libxkbfile-devel
BuildRequires:  libxcb-devel
BuildRequires:  xcb-util-keysyms-devel
BuildRequires:  xcb-util-image-devel
BuildRequires:  xcb-util-renderutil-devel
BuildRequires:  xcb-util-devel
BuildRequires:  libxkbcommon-devel
BuildRequires:  pkgconfig(xkeyboard-config)

BuildRequires:  qt5-qtbase-devel >= 5.9
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  qt5-qtsvg-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  phonon-qt5-devel

BuildRequires:  ibus-devel
%if 0%{?scim}
BuildRequires:  scim-devel
%endif

BuildRequires:  kf5-rpm-macros >= %{kf5_version_min}
BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-plasma-devel >= %{kf5_version_min}
Requires:       kf5-plasma%{?_isa} >= %{kf5_version}
BuildRequires:  kf5-kdoctools-devel >= %{kf5_version_min}
BuildRequires:  kf5-ki18n-devel >= %{kf5_version_min}
BuildRequires:  kf5-kcmutils-devel >= %{kf5_version_min}
BuildRequires:  kf5-kglobalaccel-devel >= %{kf5_version_min}
BuildRequires:  kf5-knewstuff-devel >= %{kf5_version_min}
BuildRequires:  kf5-kdelibs4support-devel >= %{kf5_version_min}
BuildRequires:  kf5-knotifyconfig-devel >= %{kf5_version_min}
BuildRequires:  kf5-kdesu-devel >= %{kf5_version_min}
BuildRequires:  kf5-attica-devel >= %{kf5_version_min}
BuildRequires:  kf5-kwallet-devel >= %{kf5_version_min}
BuildRequires:  kf5-krunner-devel >= %{kf5_version_min}
BuildRequires:  kf5-baloo-devel >= %{kf5_version_min}
BuildRequires:  kf5-kdeclarative-devel >= %{kf5_version_min}
BuildRequires:  kf5-kpeople-devel >= %{kf5_version_min}
BuildRequires:  kf5-kded-devel >= %{kf5_version_min}
BuildRequires:  kf5-kinit-devel >= %{kf5_version_min}
# libkdeinit5_*
%{?kf5_kinit_requires}

BuildRequires:  kf5-ksysguard-devel >= %{majmin_ver}
BuildRequires:  kscreenlocker-devel >= %{majmin_ver}
BuildRequires:  kwin-devel >= %{majmin_ver}
# see %%prep below -- rex
BuildRequires:  plasma-breeze >= %{majmin_ver}
BuildRequires:  plasma-workspace-devel >= %{majmin_ver}

# Optional
%if 0%{?rhel} && 0%{?rhel} > 7
BuildRequires:  cmake(AppStreamQt)
%endif
BuildRequires:  kf5-kactivities-devel >= %{kf5_version_min}
BuildRequires:  kf5-kactivities-stats-devel >= %{kf5_version_min}
BuildRequires:  libcanberra-devel
BuildRequires:  boost-devel
BuildRequires:  pulseaudio-libs-devel

BuildRequires:  chrpath
BuildRequires:  desktop-file-utils

# xorg-x11 doesn't have hw_server and disable for s390/s390x
%ifnarch s390 s390x
# KCM touchpad has been merged to plasma-desktop in 5.3
Provides:       kcm_touchpad = %{version}-%{release}
Obsoletes:      kcm_touchpad < 5.3.0
# for xserver-properties
BuildRequires:  xorg-x11-server-devel
Requires:       kf5-kded

# for kcm_keyboard
BuildRequires:  pkgconfig(libudev)
Requires:       iso-codes

# for kcm_input, kcm_touchpad
BuildRequires:  pkgconfig(xorg-evdev)
BuildRequires:  pkgconfig(xorg-libinput)
%if 0%{?synaptics}
BuildRequires:  pkgconfig(xorg-synaptics)
%endif
%endif

# Desktop
Requires:       plasma-workspace >= %{majmin_ver}

# Qt Integration (brings in Breeze)
Requires:       plasma-integration >= %{majmin_ver}

# Install systemsettings, full set of KIO slaves and write() notifications
Requires:       plasma-systemsettings >= %{majmin_ver}
Requires:       kio-extras
Requires:       kwrited >= %{majmin_ver}

# Install KWin
Requires:       kwin >= %{majmin_ver}

# kickoff -> edit applications (#1229393)
Requires:       kmenuedit >= %{majmin_ver}

Requires:       qqc2-desktop-style

# Virtual provides for plasma-workspace
Provides:       plasmashell(desktop) = %{version}-%{release}
Provides:       plasmashell = %{version}-%{release}

Obsoletes:      kde-workspace < 5.0.0-1

Obsoletes:      kactivities-workspace < 5.6.0
Provides:       kactivities-workspace = %{version}-%{release}

# kimpanel moved here from kdeplasma-addons-5.5.x
Conflicts:      kdeplasma-addons < 5.6.0

# kcm_activities.mo moved here (#1325724)
Conflicts:      kde-l10n < 15.12.3-4

%description
%{summary}.

%package        kimpanel-scim
Summary:        SCIM backend for kimpanel
Requires:       %{name} = %{version}-%{release}
%description    kimpanel-scim
A backend for the kimpanel panel icon for input methods using the SCIM input
method framework.

%package        doc
Summary:        Documentation and user manuals for %{name}
# when conflicting HTML docs were removed
Conflicts:      kcm_colors < 1:4.11.16-10
# when conflicting HTML docs were removed
Conflicts:      kde-runtime-docs < 17.08.3-6
# when made noarch
Obsoletes: plasma-desktop-doc < 5.3.1-2
BuildArch: noarch
%description    doc
%{summary}.


%prep
%setup -q


%build
%ifarch s390 %{arm}
# Decrease debuginfo verbosity to reduce memory consumption even more
%global optflags %(echo %{optflags} | sed 's/-g /-g1 /')
%endif

mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}


%find_lang %{name} --with-html --all-name

grep "%{_kf5_docdir}" %{name}.lang > %{name}-doc.lang
cat  %{name}.lang %{name}-doc.lang | sort | uniq -u > plasmadesktop5.lang




# rename script to force it to run again (initial 5.5.0 version was buggy)
mv %{buildroot}%{_datadir}/plasma/shells/org.kde.plasma.desktop/contents/updates/obsolete_kickoffrc.js \
   %{buildroot}%{_datadir}/plasma/shells/org.kde.plasma.desktop/contents/updates/obsolete_kickoffrc-1.js




%check
desktop-file-validate %{buildroot}/%{_datadir}/applications/org.kde.{kfontview,knetattach}.desktop


%ldconfig_scriptlets

%files -f plasmadesktop5.lang
%license COPYING*
%{_bindir}/kaccess
%{_bindir}/kcolorschemeeditor
%{_bindir}/kfontinst
%{_bindir}/kfontview
%{_bindir}/krdb
%{_bindir}/knetattach
%{_bindir}/solid-action-desktop-gen
%{_bindir}/lookandfeeltool

%{_kf5_libexecdir}/kauth/kcmdatetimehelper
%{_kf5_libexecdir}/kauth/fontinst
%{_kf5_libexecdir}/kauth/fontinst_helper
%{_kf5_libexecdir}/kauth/fontinst_x11
%{_libexecdir}/kimpanel-ibus-panel
%{_libexecdir}/kimpanel-ibus-panel-launcher
%{_libexecdir}/plasma-changeicons
%{_libexecdir}/kfontprint
%{_kf5_qmldir}/org/kde/plasma/private
%{_kf5_libdir}/libkdeinit5_kaccess.so
%{_kf5_libdir}/kconf_update_bin/*
# TODO: -libs subpkg -- rex
%{_kf5_libdir}/libkfontinst.so.*
%{_kf5_libdir}/libkfontinstui.so.*
%{_kf5_qtplugindir}/*.so
%{_kf5_qtplugindir}/kcms/*.so
%{_kf5_plugindir}/kded/*.so
%{_kf5_qmldir}/org/kde/plasma/activityswitcher
%{_kf5_qmldir}/org/kde/private/desktopcontainment/*
%{_kf5_qmldir}/org/kde/activities/settings/
%{_kf5_datadir}/plasma/*
%ifnarch s390 s390x
%if 0%{?synaptics}
# touchpad
%{_kf5_datadir}/kservices5/kded/touchpad.desktop
%{_bindir}/kcm-touchpad-list-devices
%{_kf5_qtplugindir}/plasma/dataengine/plasma_engine_touchpad.so
%{_datadir}/config.kcfg/touchpad.kcfg
%{_datadir}/config.kcfg/touchpaddaemon.kcfg
%{_datadir}/dbus-1/interfaces/org.kde.touchpad.xml
%endif
# kcminput
%{_kf5_bindir}/kapplymousetheme
%{_kf5_datadir}/kcmmouse/
%endif


%{_sysconfdir}/dbus-1/system.d/
%{_sysconfdir}/xdg/*.knsrc
%{_sysconfdir}/xdg/plasma-desktop.categories

%{_kf5_libdir}/libkfontinst.so
%{_kf5_libdir}/libkfontinstui.so
%{_kf5_datadir}/color-schemes/
%{_kf5_datadir}/kcm_phonon/
%{_kf5_datadir}/locale/sr/LC_SCRIPTS/kfontinst/kfontinst.js
%{_kf5_datadir}/locale/sr@ijekavian/LC_SCRIPTS/kfontinst/kfontinst.js
%{_kf5_datadir}/locale/sr@ijekavianlatin/LC_SCRIPTS/kfontinst/kfontinst.js
%{_kf5_datadir}/locale/sr@latin/LC_SCRIPTS/kfontinst/kfontinst.js


%{_kf5_qtplugindir}/plasma/dataengine/plasma_engine_kimpanel.so
%{_kf5_datadir}/kconf_update/*
%{_kf5_datadir}/kdisplay
%{_kf5_datadir}/kcontrol
%{_kf5_datadir}/kcmkeys
%{_kf5_datadir}/kcm_componentchooser
%{_kf5_datadir}/kfontinst
%{_kf5_datadir}/kcmkeyboard
%{_kf5_datadir}/kpackage/kcms/*

%{_datadir}/konqsidebartng/virtual_folders/services/fonts.desktop
%{_kf5_datadir}/kf5/kactivitymanagerd/workspace/
%{_kf5_datadir}/kcmsolidactions/
%{_kf5_datadir}/solid/devices/*.desktop

%{_kf5_datadir}/kservices5/*.desktop
%{_kf5_datadir}/kservices5/ServiceMenus/installfont.desktop
%{_kf5_datadir}/kservices5/fonts.protocol
%{_kf5_datadir}/kservicetypes5/*.desktop
%{_kf5_datadir}/kxmlgui5/kfontview
%{_kf5_datadir}/kxmlgui5/kfontinst
%{_kf5_datadir}/knotifications5/*.notifyrc
%{_datadir}/icons/hicolor/*/*/*
%{_kf5_metainfodir}/*.xml
%{_datadir}/applications/*.desktop
%{_datadir}/dbus-1/services/*.service
%{_datadir}/dbus-1/system-services/*.service
%{_datadir}/polkit-1/actions/org.kde.fontinst.policy
%{_datadir}/polkit-1/actions/org.kde.kcontrol.kcmclock.policy

%if 0%{?scim}
%files kimpanel-scim
%{_libexecdir}/kimpanel-scim-panel
%endif

%files doc -f %{name}-doc.lang


%changelog

