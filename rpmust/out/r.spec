%define aprver 1

Name: apr
Version: 1.7.0
Release: 2
Summary: Apache Portable Runtime.
License: ASL 2.0 and BSD with advertising and ISC and BSD
URL: http://apr.apache.org
Source0: http://www.apache.org/dist/%{name}/%{name}-%{version}.tar.bz2
Source1: apr-wrapper.h

Patch0: apr-1.2.2-libdir.patch
Patch1: apr-1.2.7-pkgconf.patch
Patch2: Split-apr_pool_check_integrity-into-two-parts.patch
Patch3: Pool-debugging-fixes.patch
Patch4: Fix-pool-debugging-output-so-that-creation-events-ar.patch
Patch5: memory-unix-apr_pools.c-apr_pool_cleanup_register.patch
Patch6: Follow-up-to-r1675967-trunk-resp.-r1863202-1.7.x.patch
Patch7: Register-the-pool-debug-log-cleanup-handler-after-em.patch

BuildRequires: gcc autoconf libtool libuuid-devel python3 lksctp-tools-devel

%description
The mission of the Apache Portable Runtime (APR) project is to create and maintain software libraries
that provide a predictable and consistent interface to underlying platform-specific implementations.
The primary goal is to provide an API to which software developers may code and be assured of
predictable if not identical behaviour regardless of the platform on which their software is built,
relieving them of the need to code special-case conditions to work around or take advantage of
platform-specific deficiencies or features.

%package devel
Summary: Apache Portable Runtime development kit.
Requires: %{name} = %{version}-%{release} pkgconfig

%description devel
Apache Portable Runtime development kit.

%package_help

%prep
%autosetup -n %{name}-%{version} -p1

%build
./buildconf
export ac_cv_search_shm_open=no
%configure \
        --includedir=%{_includedir}/%{name}-%{aprver} --with-installbuilddir=%{_libdir}/%{name}-%{aprver}/build \
        --with-devrandom=/dev/urandom
make

%install
rm -rf %{buildroot}
%make_install
install -D -m 0644 build/find_apr.m4 %{buildroot}/%{_datadir}/aclocal/find_apr.m4
sed -ri '/^dependency_libs/{s,-l(uuid|crypt) ,,g}' \
       %{buildroot}%{_libdir}/libapr*.la
sed -ri '/^LIBS=/{s,-l(uuid|crypt) ,,g;s/  */ /g}' \
       %{buildroot}%{_bindir}/%{name}-%{aprver}-config
sed -ri '/^Libs/{s,-l(uuid|crypt) ,,g}' \
       %{buildroot}%{_libdir}/pkgconfig/%{name}-%{aprver}.pc
%ifarch %{multilib_arches}
install -D -m 0644 %{buildroot}%{_includedir}/%{name}-%{aprver}/apr.h \
   %{buildroot}%{_includedir}/%{name}-%{aprver}/%{name}-%{_arch}.h
install -D -m 0644 %{SOURCE1}  %{buildroot}%{_includedir}/%{name}-%{aprver}/apr.h
%endif
rm -rf %{buildroot}%{_libdir}/apr.exp
rm -rf %{buildroot}%{_libdir}/libapr-*.a

%check
make check
%ldconfig_scriptlets

%pre

%preun

%post

%postun

%files
%doc CHANGES NOTICE
%license LICENSE
%{_libdir}/libapr-%{aprver}.so.*

%files devel
%{_bindir}/%{name}-%{aprver}-config
%{_libdir}/libapr-%{aprver}.*a
%{_libdir}/libapr-%{aprver}.so
%{_libdir}/pkgconfig/*.pc
%dir %{_libdir}/%{name}-%{aprver}
%dir %{_libdir}/%{name}-%{aprver}/build
%{_libdir}/%{name}-%{aprver}/build/*
%dir %{_includedir}/%{name}-%{aprver}
%{_includedir}/%{name}-%{aprver}/*.h
%{_datadir}/aclocal/*.m4

%files help
%doc docs/APRDesign.html docs/canonical_filenames.html
%doc docs/incomplete_types docs/non_apr_programs

%changelog
* Mon Jun 29 2020  linwei<linwei54@huawei.com> - 1.7.0-2
- sync some patches from community

* Mon May 11 2020 openEuler Buildteam <buildteam@openeuler.org> - 1.7.0-1
- update to 1.7.0-1

* Tue Dec 17 2019 openEuler Buildteam <buildteam@openeuler.org> - 1.6.5-4
- quality enhancement synchronization github patch

* Tue Oct 22 2019 openEuler Buildteam <buildteam@openeuler.org> - 1.6.5-3
- optimize spec file.

* Sat Sep 28 2019 openEuler Buildteam <buildteam@openeuler.org> - 1.6.5-2
- Package rebuild.

* Wed Sep 04 2019 openEuler Buildteam <buildteam@openeuler.org> - 1.6.5-1
- Package init.
