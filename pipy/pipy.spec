Name:		pipy
Version: 	0.30.0
Release: 	1%{?dist}
Summary: 	Pipy is a programmable network proxy for the cloud, edge and IoT.
License: 	NEU License
Source0: 	https://github.com/flomesh-io/pipy/archive/refs/tags/0.30.0-23.tar.gz
BuildRequires: 	cmake3
BuildRequires:  clang
BuildRequires:  nodejs >= 12
BuildRequires:  zlib
BuildRequires:   libstdc++-static
BuildRequires:  chrpath
#AutoReqProv: no
%define revision %{release}
%define prefix /usr/local
%define PIPY_GUI ON
%global PIPY_STATIC ON
%global BUILD_TYPE Release
%global debug_package %{nil}

%description
Pipy is a tiny, high performance, highly stable, programmable proxy.

%prep
%setup -q -n %{name}-0.30.0-23

%build
if [ %{PIPY_GUI} == "ON" ] ; then
  npm install
  npm run build
fi
%{__mkdir} build
cd build
CXX=clang++ CC=clang cmake3 -DPIPY_GUI=%{PIPY_GUI} -DPIPY_STATIC=ON -DPIPY_TUTORIAL=%{PIPY_GUI} -DCMAKE_BUILD_TYPE=Release ..
make %{?_smp_mflags} 
cd ..

%preun

systemctl --no-reload disable pipy.service > /dev/null 2>&1 || true
systemctl stop pipy.service > /dev/null 2>&1 || true



%pre
getent group pipy >/dev/null || groupadd -r pipy
getent passwd pipy >/dev/null || useradd -r -g pipy -G pipy -d /etc/pipy -s /sbin/nologin -c "pipy" pipy


%install
mkdir -p %{buildroot}%{prefix}/bin
mkdir -p %{buildroot}/etc/pipy
cp bin/pipy   %{buildroot}%{prefix}/bin
# chrpath --delete %{buildroot}%{prefix}/bin/pipy

%post

%postun
if [ $1 -eq 0 ] ; then
        userdel pipy 2> /dev/null || true
fi


%files
%attr(755, pipy, pipy) %{prefix}/bin/pipy

%changelog

