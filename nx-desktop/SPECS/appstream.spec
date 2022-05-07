%global vala 1

Summary: Utilities to generate, maintain and access the AppStream database
Name:    appstream
Version: 0.11.8
Release: 1%{?dist}

# lib LGPLv2+, tools GPLv2+
License: GPLv2+ and LGPLv2+
#URL:     http://www.freedesktop.org/wiki/Distributions/AppStream
URL:     https://github.com/ximion/appstream
Source0: http://www.freedesktop.org/software/appstream/releases/AppStream-%{version}.tar.xz
# http://www.freedesktop.org/software/appstream/releases/AppStream-0.12.10.tar.xz

# needed for cmake auto-provides
BuildRequires: cmake
BuildRequires: meson
BuildRequires: gettext
BuildRequires: gperf
BuildRequires: gtk-doc
BuildRequires: intltool
BuildRequires: itstool
BuildRequires: libstemmer-devel
BuildRequires: pkgconfig(gio-2.0) pkgconfig(gobject-introspection-1.0)
# BuildRequires: pkgconfig(libsoup-2.4)
BuildRequires: libsoup-devel
BuildRequires: pkgconfig(libxml-2.0)
BuildRequires: pkgconfig(lmdb)
# BuildRequires: pkgconfig(packagekit-glib2)
BuildRequires: PackageKit-devel
#BuildRequires: pkgconfig(protobuf-lite)
BuildRequires: protobuf-lite-devel
BuildRequires: pkgconfig(Qt5Core)
# lrelease
BuildRequires: qt5-linguist
# BuildRequires: pkgconfig(yaml-0.1)
BuildRequires: libyaml-devel
BuildRequires: vala
BuildRequires: xmlto

Requires: appstream-data

%description
AppStream makes it easy to access application information from the
AppStream database over a nice GObject-based interface.

%package devel
Summary:  Development files for %{name}
Requires: %{name}%{?_isa} = %{version}-%{release}
# -vala subpackage removed in F30
Obsoletes: appstream-vala < 0.12.4-3
Provides: appstream-vala = %{version}-%{release}
%description devel
%{summary}.

%package qt
Summary: Qt5 bindings for %{name}
Requires: %{name}%{?_isa} = %{version}-%{release}
%description qt
%{summary}.

%package qt-devel
Summary:  Development files for %{name}-qt bindings
Requires: %{name}-qt%{?_isa} = %{version}-%{release}
Requires: pkgconfig(Qt5Core)
%description qt-devel
%{summary}.


%prep
%autosetup -n AppStream-%{version} -p1

sed -i -e "s|0.12.2|%{version}|" meson.build


%build
%{meson} \
 -Dqt=true \
 -Dvapi=%{?vala:true}%{!?vala:false}

%{meson_build}


%install
%{meson_install}

mkdir -p %{buildroot}/var/cache/app-info/{icons,gv,xmls}
touch %{buildroot}/var/cache/app-info/cache.watch

%find_lang appstream

%if "%{?_metainfodir}" != "%{_datadir}/metainfo"
# move metainfo to right/legacy location
mkdir -p %{buildroot}%{_kf5_metainfodir}
mv %{buildroot}%{_datadir}/metainfo/*.xml \
   %{buildroot}%{_metainfodir}
%endif


%check
%{meson_test} ||:


%ldconfig_scriptlets

%posttrans
%{_bindir}/appstreamcli refresh --force >& /dev/null ||:

## use file triggers instead of static pkg names
## other repos can provide appdata too
%if 0%{?fedora} > 25
## not sure how smart appstreamcli is about cache validation
## to judge if --force is really needed here or not -- rex
%transfiletriggerin -- %{_datadir}/app-info/xmls
%{_bindir}/appstreamcli refresh --force >& /dev/null ||:

%transfiletriggerpostun -- %{_datadir}/app-info/xmls
%{_bindir}/appstreamcli refresh >& /dev/null ||:
%else
%triggerun -- appstream-data
%{_bindir}/appstreamcli refresh >& /dev/null ||:
%endif

%files -f appstream.lang
%doc AUTHORS
%license LICENSE.GPLv2
%license LICENSE.LGPLv2.1
%{_bindir}/appstreamcli
%{_mandir}/man1/appstreamcli.1*
%config(noreplace) %{_sysconfdir}/appstream.conf
%dir %{_libdir}/girepository-1.0/
%{_libdir}/girepository-1.0/AppStream-1.0.typelib
%{_libdir}/libappstream.so.4*
%{_libdir}/libappstream.so.%{version}
%{_metainfodir}/org.freedesktop.appstream.cli.*.xml
# put in -devel? -- rex
%{_datadir}/gettext/its/metainfo.*
%ghost /var/cache/app-info/cache.watch
%dir /var/cache/app-info/
%dir /var/cache/app-info/icons/
%dir /var/cache/app-info/gv/
%dir /var/cache/app-info/xmls/

%files devel
%{_includedir}/appstream/
%{_libdir}/libappstream.so
%{_libdir}/pkgconfig/appstream.pc
%dir %{_datadir}/gir-1.0/
%{_datadir}/gir-1.0/AppStream-1.0.gir
%if 0%{?vala}
%dir %{_datadir}/vala
%dir %{_datadir}/vala/vapi
%{_datadir}/vala/vapi/appstream.deps
%{_datadir}/vala/vapi/appstream.vapi
%endif
%{_docdir}/appstream/html/
## symlink pointing to ^^, but need to take care, since rpm has
## trouble replacing dirs with symlinks, omit it for now -- rex
%exclude %{_datadir}/gtk-doc/html/appstream

%ldconfig_scriptlets qt

%files qt
%{_libdir}/libAppStreamQt.so.2*
%{_libdir}/libAppStreamQt.so.%{version}

%files qt-devel
%{_includedir}/AppStreamQt/
%{_libdir}/cmake/AppStreamQt/
%{_libdir}/libAppStreamQt.so


%changelog

