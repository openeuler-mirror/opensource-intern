Name:    khotkeys
Version: 5.15.5
Release: 1%{?dist}
Summary: Application to configure hotkeys in KDE

License: GPLv2+
URL:     https://cgit.kde.org/%{name}.git


%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0:        http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/khotkeys-5.15.5.tar.xz
## downstream patches
# kcm_hotkeys, use qdbusviewer-qt5, see also http://bugs.kde.org/329094
# Patch100: khotkeys-5.4.2-qdbusviewer-qt5.patch
# use qdbus-qt5 instead of hard-coding (unpathed) qdbus
# FIXME: make upstreamable
# Patch101: khotkeys-5.14.4-qdbus-qt5.patch

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-kcmutils-devel
BuildRequires:  kf5-kdbusaddons-devel
BuildRequires:  kf5-kdelibs4support-devel
BuildRequires:  kf5-kdoctools-devel
BuildRequires:  kf5-kglobalaccel-devel
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-kxmlgui-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-rpm-macros
BuildRequires:  libX11-devel
BuildRequires:  libXtst-devel
BuildRequires:  plasma-workspace-devel
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtx11extras-devel

# expects to call qdbus binary (currently in qt5-qttools):
Requires:       qt5-qttools
# not sure if we want a hard dep on this (yet) -- rex
%if 0%{?fedora} > 21
Recommends:     qt5-qdbusviewer
%endif

# when khotkeys was split out of kde-workspace-4.11.x
Conflicts:      kde-workspace < 4.11.15-3

# upgrade path from khotkeys-libs-4.11.x (skip Provides for now, it was only ever a private library)
Obsoletes:      khotkeys-libs < 5.0.0
#Provides:       khotkeys-libs = %{version}-%{release}

%description
An advanced editor component which is used in numerous KDE applications
requiring a text editing component.

%package        devel
Summary:        Development files for %{name}
# strictly speaking, not required in this case, but still often expected to pull in subpkg -- rex
Requires:       %{name}%{?_isa} = %{version}-%{release}
%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.


%prep
%autosetup -p1


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

# hack around FTBFS:
# make[2]: *** No rule to make target 'app/org.kde.khotkeys.xml', needed by 'kcm_hotkeys/khotkeys_interface.cpp'.  Stop.
%make_build -C %{_target_platform}/app
%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang khotkeys --with-html


%ldconfig_scriptlets

%files -f khotkeys.lang
%license COPYING*
%{_kf5_libdir}/libkhotkeysprivate.so.*
%{_kf5_qtplugindir}/kcm_hotkeys.so
%{_kf5_plugindir}/kded/khotkeys.so
%{_kf5_datadir}/kservices5/khotkeys.desktop
%{_datadir}/khotkeys/

%files devel
%{_datadir}/dbus-1/interfaces/org.kde.khotkeys.xml
%{_libdir}/cmake/KHotKeysDBusInterface/


