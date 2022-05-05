%global         framework baloo

# uncomment to enable bootstrap mode
#global bootstrap 1

%if !0%{?bootstrap}
%global tests 1
%endif

Name:    kf5-%{framework}
Summary: A Tier 3 KDE Frameworks 5 module that provides indexing and search functionality
Version: 5.55.0
Release: 1%{?dist}

# libs are LGPL, tools are GPL
# KDE e.V. may determine that future LGPL/GPL versions are accepted
License: (LGPLv2 or LGPLv3) and (GPLv2 or GPLv3)
URL:     https://community.kde.org/Baloo
#URL:     https://cgit.kde.org/%{framework}.git

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0:        http://download.kde.org/%{stable}/frameworks/%{majmin}/%{framework}-%{version}.tar.xz

Source1:        97-kde-baloo-filewatch-inotify.conf

## upstreamable patches
# http://bugzilla.redhat.com/1235026
# Patch100: baloo-5.45.0-baloofile_config.patch

## upstream patches

BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-kconfig-devel >= %{majmin}
BuildRequires:  kf5-kcoreaddons-devel >= %{majmin}
BuildRequires:  kf5-kcrash-devel >= %{majmin}
BuildRequires:  kf5-kdbusaddons-devel >= %{majmin}
BuildRequires:  kf5-kfilemetadata-devel >= %{majmin}
BuildRequires:  kf5-ki18n-devel >= %{majmin}
BuildRequires:  kf5-kidletime-devel >= %{majmin}
BuildRequires:  kf5-kio-devel >= %{majmin}
BuildRequires:  kf5-rpm-macros >= %{majmin}
BuildRequires:  kf5-solid-devel >= %{majmin}

BuildRequires:  lmdb-devel
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtdeclarative-devel

%if 0%{?tests}
BuildRequires: dbus-x11
BuildRequires: time
BuildRequires: xorg-x11-server-Xvfb
%endif

Obsoletes:      kf5-baloo-tools < 5.5.95-1
Provides:       kf5-baloo-tools = %{version}-%{release}

%if 0%{?fedora}
Obsoletes:      baloo < 5
Provides:       baloo = %{version}-%{release}
%else
Conflicts:      baloo < 5
%endif

# main pkg accidentally multilib'd prior to 5.21.0-4
Obsoletes:      kf5-baloo < 5.21.0-4

%description
%{summary}.

%package        devel
Summary:        Development files for %{name}
# KDE e.V. may determine that future LGPL versions are accepted
License:        LGPLv2 or LGPLv3
Requires:       %{name}-libs%{?_isa} = %{version}-%{release}
Requires:       kf5-kcoreaddons-devel >= %{majmin}
Requires:       kf5-kfilemetadata-devel >= %{majmin}
Requires:       qt5-qtbase-devel

%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.

%package        file
Summary:        File indexing and search for Baloo
# KDE e.V. may determine that future LGPL versions are accepted
License:        LGPLv2 or LGPLv3
%if 0%{?fedora}
Obsoletes:      baloo-file < 5.0.1-2
Provides:       baloo-file = %{version}-%{release}
%else
Conflicts:      baloo-file < 5
%endif
Requires:       %{name}-libs%{?_isa} = %{version}-%{release}
%description    file
%{summary}.

%package        libs
Summary:        Runtime libraries for %{name}
# KDE e.V. may determine that future LGPL versions are accepted
License:        LGPLv2 or LGPLv3
%description    libs
%{summary}.


%prep
%autosetup -n %{framework}-%{version} -p1


%build
mkdir build
cd build
%{cmake_kf5} .. \
  -DBUILD_TESTING:BOOL=%{?tests:ON}%{!?tests:OFF}
cd ../

make %{?_smp_mflags} -C build


%install
make install DESTDIR=%{buildroot} -C build

# baloodb not installed unless BUILD_EXPERIMENTAL is enabled, so omit translations
rm -fv %{buildroot}%{_datadir}/locale/*/LC_MESSAGES/baloodb5.*

install -p -m644 -D %{SOURCE1} %{buildroot}%{_prefix}/lib/sysctl.d/97-kde-baloo-filewatch-inotify.conf

%find_lang kio5_baloosearch
%find_lang kio5_tags
%find_lang kio5_timeline
%find_lang balooctl5
#find_lang baloodb5
%find_lang balooengine5
%find_lang baloosearch5
%find_lang balooshow5
%find_lang baloo_file5
%find_lang baloo_file_extractor5
%find_lang baloomonitorplugin

cat kio5_tags.lang kio5_baloosearch.lang kio5_timeline.lang \
    balooctl5.lang balooengine5.lang baloosearch5.lang \
    balooshow5.lang baloomonitorplugin.lang \
    > %{name}.lang

cat baloo_file5.lang baloo_file_extractor5.lang \
    > %{name}-file.lang


%check
%if 0%{?tests}
export CTEST_OUTPUT_ON_FAILURE=1
xvfb-run -a \
dbus-launch --exit-with-session \
time \
make test ARGS="--output-on-failure --timeout 300" -C %{_target_platform} ||:
%endif


%files -f %{name}.lang
%license COPYING
#{_kf5_bindir}/baloodb
%{_kf5_bindir}/baloosearch
%{_kf5_bindir}/balooshow
%{_kf5_bindir}/balooctl
%{_kf5_sysconfdir}/xdg/baloo.categories

%files file -f %{name}-file.lang
%{_prefix}/lib/sysctl.d/97-kde-baloo-filewatch-inotify.conf
%{_kf5_bindir}/baloo_file
%{_kf5_bindir}/baloo_file_extractor
%{_kf5_sysconfdir}/xdg/autostart/baloo_file.desktop

%ldconfig_scriptlets libs

%files libs
%license COPYING.LIB
%{_kf5_libdir}/libKF5Baloo.so.*
%{_kf5_libdir}/libKF5BalooEngine.so.*
# multilib'd plugins and friends
%{_kf5_plugindir}/kio/baloosearch.so
%{_kf5_plugindir}/kio/tags.so
%{_kf5_plugindir}/kio/timeline.so
%{_kf5_plugindir}/kded/baloosearchmodule.so
%{_kf5_qmldir}/org/kde/baloo
%{_kf5_datadir}/kservices5/baloosearch.protocol
%{_kf5_datadir}/kservices5/tags.protocol
%{_kf5_datadir}/kservices5/timeline.protocol
# track icon size too, since it may conflict with baloo-4.x
%{_kf5_datadir}/icons/hicolor/128x128/apps/baloo.png

%files devel
%{_kf5_libdir}/libKF5Baloo.so
%{_kf5_libdir}/cmake/KF5Baloo/
%{_kf5_libdir}/pkgconfig/Baloo.pc
%{_kf5_includedir}/Baloo/
%{_kf5_includedir}/baloo_version.h
%{_kf5_archdatadir}/mkspecs/modules/qt_Baloo.pri
%{_kf5_datadir}/dbus-1/interfaces/org.kde.baloo.*.xml
%{_kf5_datadir}/dbus-1/interfaces/org.kde.Baloo*.xml



