%global major_version 2

Name:           botan2
Version:        2.13.0
Release:        2%{?dist}
Summary:        Crypto and TLS for C++11

License:        BSD
URL:            https://botan.randombit.net/
Source0:        https://botan.randombit.net/releases/Botan-%{version}.tar.xz
#https://botan.randombit.net/releases/Botan-2.13.0.tar.xz
Patch0:         01-remove-rpath-gcc.patch

BuildRequires:  gcc-c++
BuildRequires:  python3
BuildRequires:  python3-devel
#BuildRequires:  %%{_bindir}/sphinx-build
BuildRequires:  python3-sphinx
BuildRequires:  %{_bindir}/rst2man
BuildRequires:  bzip2-devel
BuildRequires:  zlib-devel
BuildRequires:  openssl-devel

%description
Botan is a BSD-licensed crypto library written in C++. It provides a
wide variety of basic cryptographic algorithms, X.509 certificates and
CRLs, PKCS \#10 certificate requests, a filter/pipe message processing
system, and a wide variety of other features, all written in portable
C++. The API reference, tutorial, and examples may help impart the
flavor of the library. This is the current stable release branch 2.x
of Botan.


%package        devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}

%description    devel
The %{name}-devel package contains libraries and header files for
developing applications that use %{name}.


%package        doc
Summary:        Documentation for %{name}
BuildArch:      noarch

%description    doc
%{summary}

This package contains HTML documentation for %{name}.


%package -n python3-%{name}
Summary:        Python3 bindings for %{name}
%{?python_provide:%python_provide python3-%{name}}

%description -n python3-%{name}
%{summary}

This package contains the Python3 binding for %{name}.


%prep
%setup -q -n Botan-%{version}
#%patch0 -p0


%build
export CXXFLAGS="${CXXFLAGS:-%{optflags}}"

# we have the necessary prerequisites, so enable optional modules
%global enable_modules bzip2,zlib,openssl

%{__python3} ./configure.py \
        --prefix=%{_prefix} \
        --libdir=%{_lib} \
        --docdir=%{_docdir} \
        --cc=gcc \
        --os=linux \
        --cpu=%{_arch} \
        --enable-modules=%{enable_modules} \
        --with-python-version=%{python3_version} \
        --with-sphinx \
        --with-rst2man \
        --distribution-info="$(source /etc/os-release ; echo "$NAME")" \
        --disable-static-library \
        --with-debug-info

# work around https://github.com/randombit/botan/issues/2130
%make_build PYTHON_EXE=%{__python3}

%install
make install PYTHON_EXE=%{__python3} DESTDIR=%{buildroot}

sed -e '1{/^#!/d}' -i %{buildroot}%{python3_sitearch}/botan2.py

# doc installation fixups
mv %{buildroot}%{_docdir}/botan-%{version} %{buildroot}%{_pkgdocdir}
rm -r %{buildroot}%{_pkgdocdir}/handbook/{.doctrees,.buildinfo}


#%%ldconfig_scriptlets
LD_LIBRARY_PATH=/home/csmsoledad/rpmbuild/BUILDROOT/botan2-2.13.0-2.x86_64/usr/lib64


%files
%license license.txt
%dir %{_pkgdocdir}
%{_pkgdocdir}/*.txt
%{_libdir}/libbotan-%{major_version}.so.13*
%{_bindir}/botan
%{_mandir}/man1/botan.1.gz


%files devel
%license license.txt
%{_includedir}/*
%{_libdir}/libbotan-%{major_version}.so
%{_libdir}/pkgconfig/botan-%{major_version}.pc


%files doc
%license license.txt
%dir %{_pkgdocdir}
%{_pkgdocdir}/handbook


%files -n python3-%{name}
%license license.txt
%{python3_sitearch}/%{name}.py
%{python3_sitearch}/__pycache__/*


%check
LD_LIBRARY_PATH=%{buildroot}%{_libdir} ./botan-test


%changelog

