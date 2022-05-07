Name:           libdmtx
Version:        0.7.5
Release:        4%{?dist}
Summary:        Library for working with Data Matrix 2D bar-codes

License:        BSD
# http://www.libdmtx.org/ doesn't work any more
# outdated info is still at http://libdmtx.sourceforge.net/
URL:            https://github.com/dmtx
Source0:        https://github.com/dmtx/%{name}/archive/v%{version}/%{name}-%{version}.tar.gz
# https://github.com/dmtx/libdmtx/archive/v0.7.5/libdmtx-0.7.5.tar.gz
# https://github.com/dmtx/libdmtx/pull/13
Patch0:         libdmtx-0.7.5-c99.patch
# https://github.com/dmtx/libdmtx/pull/14
Patch1:         libdmtx-0.7.5-size_t.patch
# https://github.com/dmtx/libdmtx/pull/12
Patch2:         libdmtx-0.7.5-math.patch

BuildRequires:  gcc
BuildRequires:  libtool

# obsolete language bindings we can't provide any more
Obsoletes:      php-libdmtx < 0.7.4
Obsoletes:      python-libdmtx < 0.7.4
Obsoletes:      ruby-libdmtx < 0.7.4


%description
libdmtx is open source software for reading and writing Data Matrix 2D
bar-codes on Linux, Unix, OS X, Windows, and mobile devices. At its core
libdmtx is a shared library, allowing C/C++ programs to use its capabilities
without restrictions or overhead.

The included utility programs, dmtxread and dmtxwrite, provide the official
interface to libdmtx from the command line, and also serve as a good reference
for programmers who wish to write their own programs that interact with
libdmtx. All of the software in the libdmtx package is distributed under
the LGPLv2 and can be used freely under these terms.


%package        devel
Summary:        Development files for %{name}
Requires:       %{name} = %{version}-%{release}

%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.


%prep
%autosetup -p1

./autogen.sh


%build
%configure --disable-static
make %{?_smp_mflags}


%install
make install DESTDIR=$RPM_BUILD_ROOT
find $RPM_BUILD_ROOT -name '*.la' -exec rm -f {} ';'


%check
make check
pushd test
for t in simple
do
    ./${t}_test/${t}_test
done
popd


%files
%license LICENSE
%doc AUTHORS ChangeLog KNOWNBUG README README.linux TODO
%{_libdir}/%{name}.so.*

%files devel
%doc
%{_includedir}/*
%{_libdir}/%{name}.so
%{_libdir}/pkgconfig/%{name}.pc
%{_mandir}/man3/%{name}.3*


%changelog

