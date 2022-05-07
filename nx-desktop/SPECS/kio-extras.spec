# uncomment to enable bootstrap mode
%global bootstrap 1

%if !0%{?bootstrap}
#global tests 1
%endif

Name:    kio-extras
Version: 19.04.3
Release: 1%{?dist}
Summary: Additional components to increase the functionality of KIO Framework

License: GPLv2+
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/applications/%{version}/src/%{name}-%{version}.tar.xz
#
## upstramable patches
# patch to use libtirpc for RPC, from Cygwin Ports
# should be upstreamable, considering that glibc's builtin RPC is obsolete
# https://github.com/cygwinports/kf5-kio-extras/blob/master/16.08.3-nfs-libtirpc.patch
Patch1000: kio-extras-19.04-nfs-libtirpc.patch

## upstream patches

# filter plugin provides
%global __provides_exclude_from ^(%{_kf5_qtplugindir}/.*\\.so)$

BuildRequires:  bzip2-devel
BuildRequires:  exiv2-devel
BuildRequires:  gperf

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-kactivities-devel
BuildRequires:  kf5-karchive-devel
BuildRequires:  kf5-kconfig-devel
BuildRequires:  kf5-kconfigwidgets-devel
BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-kdbusaddons-devel
BuildRequires:  kf5-kdelibs4support-devel
BuildRequires:  kf5-kdnssd-devel
BuildRequires:  kf5-kdoctools-devel
BuildRequires:  kf5-khtml-devel
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-kpty-devel
BuildRequires:  kf5-rpm-macros
BuildRequires:  kf5-solid-devel
BuildRequires:  cmake(KF5SyntaxHighlighting)
BuildRequires:  cmake(KF5ActivitiesStats)

BuildRequires:  libjpeg-devel
BuildRequires:  libmtp-devel
BuildRequires:  libsmbclient-devel
BuildRequires:  libssh-devel
BuildRequires:  OpenEXR-devel
BuildRequires:  openslp-devel
BuildRequires:  perl-generators
BuildRequires:  phonon-qt5-devel
BuildRequires:  libtirpc-devel
BuildRequires:  pkgconfig(shared-mime-info)
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtsvg-devel
BuildRequires:  taglib-devel > 1.11

%if 0%{?tests}
BuildRequires: dbus-x11
BuildRequires: time
BuildRequires: xorg-x11-server-Xvfb
%endif

# translations moved here
Conflicts: kde-l10n < 17.03

# short-lived subpkg, locale conflicts fixed in kio_mtp instead
Obsoletes:      kio-extras-mtp-common < 5.2.2-3

Obsoletes: kde-runtime-docs < 5.0.0-1
# when went noarch
Obsoletes: kio-extras-doc < 5.8.0-2
# moved to main pkg
Obsoletes: kio-extras-docs < 17.03
Provides:  kio-extras-docs = %{version}-%{release}

# -htmlthumbnail removed
Obsoletes: kio-extras-htmlthumbnail < 18.08.3

# helpful for  imagethumbnail plugin
Recommends:     qt5-qtimageformats %{?_isa}

# when -info was split out
Obsoletes: kio-extras < 19.04.1-1

%description
%{summary}.

%package info
Summary: Info kioslave
# when -info was split out
Obsoletes: kio-extras < 19.04.1-1
%description info
Kioslave for reading info pages.

%package        devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
%description devel
%{summary}.


%prep
%autosetup -p1
#setup -q
#patch1000 -p1 -b .nfs-libtirpc


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} .. \
  -DLIBSSH_LIBRARIES="$(pkg-config --libs libssh)" \
  %{?tests:-DBUILD_TESTING:BOOL=ON}
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang %{name} --all-name --with-html


%check
%if 0%{?tests}
export CTEST_OUTPUT_ON_FAILURE=1
xvfb-run -a dbus-launch --exit-with-session \
time make test -C %{_target_platform} ARGS="--output-on-failure --timeout 10" ||:
%endif


%ldconfig_scriptlets

%files -f %{name}.lang
# include *a* copy, others are in mtp/
%license fish/COPYING
# %%{_kf5_datadir}/qlogging-categories5/
%{_sysconfdir}/xdg/kio-extras.categories
%{_kf5_libdir}/libkioarchive.so.5*
%{_kf5_libdir}/libmolletnetwork5.so.*
%dir %{_kf5_plugindir}/kded
%{_kf5_plugindir}/kded/filenamesearchmodule.so
%{_kf5_plugindir}/kded/networkwatcher.so
%{_kf5_plugindir}/kded/recentdocumentsnotifier.so
%dir %{_kf5_plugindir}/kio/
%dir %{_kf5_plugindir}/kiod/
%{_kf5_plugindir}/kio/about.so
%{_kf5_plugindir}/kio/activities.so
%{_kf5_plugindir}/kio/archive.so
%{_kf5_plugindir}/kio/bookmarks.so
%{_kf5_plugindir}/kio/filenamesearch.so
%{_kf5_plugindir}/kio/filter.so
%{_kf5_plugindir}/kio/fish.so
%{_kf5_plugindir}/kio/man.so
%{_kf5_plugindir}/kiod/kmtpd.so
%{_kf5_plugindir}/kio/mtp.so
%{_kf5_plugindir}/kio/network.so
%{_kf5_plugindir}/kio/nfs.so
%{_kf5_plugindir}/kio/recentdocuments.so
# %%{_kf5_plugindir}/kio/recentlyused.so
%{_kf5_plugindir}/kio/settings.so
%{_kf5_plugindir}/kio/sftp.so
%{_kf5_plugindir}/kio/smb.so
%{_kf5_plugindir}/kio/thumbnail.so
%{_kf5_plugindir}/parts/kmanpart.so
%{_kf5_qtplugindir}/audiothumbnail.so
%{_kf5_qtplugindir}/comicbookthumbnail.so
%{_kf5_qtplugindir}/djvuthumbnail.so
%{_kf5_qtplugindir}/exrthumbnail.so
%{_kf5_qtplugindir}/imagethumbnail.so
%{_kf5_qtplugindir}/jpegthumbnail.so
%{_kf5_qtplugindir}/kactivitymanagerd_fileitem_linking_plugin.so
%{_kf5_qtplugindir}/kfileaudiopreview.so
%{_kf5_qtplugindir}/ebookthumbnail.so
%{_kf5_qtplugindir}/kritathumbnail.so
%{_kf5_qtplugindir}/opendocumentthumbnail.so
%{_kf5_qtplugindir}/svgthumbnail.so
%{_kf5_qtplugindir}/textthumbnail.so
%{_kf5_qtplugindir}/windowsexethumbnail.so
%{_kf5_qtplugindir}/windowsimagethumbnail.so
%{_datadir}/kio_docfilter/
%{_datadir}/kio_bookmarks/
%dir %{_datadir}/konqsidebartng/
%dir %{_datadir}/konqsidebartng/virtual_folders/
%dir %{_datadir}/konqsidebartng/virtual_folders/remote/
%{_datadir}/konqsidebartng/virtual_folders/remote/virtualfolder_network.desktop
%dir %{_datadir}/konqueror/
%dir %{_datadir}/konqueror/dirtree/
%dir %{_datadir}/konqueror/dirtree/remote/
%{_datadir}/konqueror/dirtree/remote/mtp-network.desktop
%{_datadir}/konqueror/dirtree/remote/smb-network.desktop
%{_datadir}/remoteview/
%{_kf5_datadir}/solid/actions/solid_mtp.desktop
%{_kf5_datadir}/kservices5/*.protocol
%{_kf5_datadir}/kservices5/*.desktop
%{_kf5_datadir}/kservicetypes5/thumbcreator.desktop
%{_datadir}/dbus-1/interfaces/kf5_org.kde.network.kioslavenotifier.xml
%{_datadir}/dbus-1/services/org.kde.kmtp.daemon.service
%{_datadir}/mime/packages/kf5_network.xml
%{_datadir}/config.kcfg/jpegcreatorsettings5.kcfg

%files info
%{_kf5_plugindir}/kio/info.so
# perl deps, but required at runtime for the info kioslave to actually work:
%dir %{_datadir}/kio_info/
%{_datadir}/kio_info/kde-info2html*

%files devel
%{_kf5_includedir}/*.h
# no soname symlink? --rex
#{_kf5_libdir}/libkioarchive.so
%{_kf5_libdir}/cmake/KioArchive/


%changelog
