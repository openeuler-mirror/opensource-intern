Name:           libbsd
Version:        0.9.1
Release:        4%{?dist}
Summary:        Library providing BSD-compatible functions for portability
URL:            http://libbsd.freedesktop.org/
License:        BSD and ISC and Copyright only and Public Domain

Source0:        http://libbsd.freedesktop.org/releases/libbsd-%{version}.tar.xz
# Patch1:         %%{name}-0.8.3-deprecated.patch
# Patch2:         %%{name}-0.8.6-compat.patch

BuildRequires:  gcc
%description
libbsd provides useful functions commonly found on BSD systems, and
lacking on others like GNU systems, thus making it easier to port
projects with strong BSD origins, without needing to embed the same
code over and over again on each project.

%package devel
Summary:        Development files for libbsd
Requires:       %{name}%{?_isa} = %{version}-%{release}

%description devel
Development files for the libbsd library.

%package ctor-static
Summary:        Development files for libbsd
Requires:       %{name}%{?_isa} = %{version}-%{release}

%description ctor-static
The libbsd-ctor static library is required if setproctitle() is to be used
when libbsd is loaded via dlopen() from a threaded program.  This can be
configured using "pkg-config --libs libbsd-ctor".
# See the libbsd mailing list message by Guillem Jover on Jul 14 2013:
#     http://lists.freedesktop.org/archives/libbsd/2013-July/000091.html

%prep
%setup -q
%if 0%{?rhel} && 0%{?rhel} < 7
%patch1 -p1 -b .deprecated
%patch2 -p1 -b .compat
%endif

%build
%configure
%make_build V=1

%check
%make_build check V=1

%install
%make_install V=1

# don't want static library or libtool archive
rm %{buildroot}%{_libdir}/%{name}.a
rm %{buildroot}%{_libdir}/%{name}.la

# remove manual pages that conflict with man-pages package
rm %{buildroot}%{_mandir}/man3/explicit_bzero.3bsd


%ldconfig_scriptlets

%files
%license COPYING
%doc README TODO ChangeLog
%{_libdir}/%{name}.so.*

%files devel
%{_mandir}/man3/*.3bsd.*
%{_mandir}/man7/%{name}.7.*
%{_includedir}/bsd
%{_libdir}/%{name}.so
%{_libdir}/pkgconfig/%{name}.pc
%{_libdir}/pkgconfig/%{name}-overlay.pc

%files ctor-static
%{_libdir}/%{name}-ctor.a
%{_libdir}/pkgconfig/%{name}-ctor.pc

%changelog

