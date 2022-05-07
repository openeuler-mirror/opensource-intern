Name: netstat-nat
Version: 1.4.10
Release: 1%{?dist}
License: GPL
Summary : netstat-nat displays NAT connections
URL: http://www.tweegy.nl/projects/netstat-nat/
Source: http://www.tweegy.nl/download/%{name}-%{version}.tar.gz
Packager	: Danny Wijsman <danny@tweegy.nl>
%description
Netstat-nat is a small program written in C. It displays NAT connections,
managed by netfilter/iptables which comes with the > 2.4.x linux kernels. The
program reads its information from '/proc/net/ip_conntrack' or
'/proc/net/nf_conntrack', which is the temporary conntrack-storage of
netfilter. (http://www.netfilter.org/)
Netstat-nat takes several arguments (but not needed).

%prep
%setup

%build
rm -f aclocal.m4
autoreconf --force --install
./configure
make all

%install
[ "$RPM_BUILD_ROOT" != "/" ] && rm -rf $RPM_BUILD_ROOT
install -D -s -m 755 %{name} %{buildroot}%{_bindir}/%{name}
install -D -m 444 netstat-nat.1 %{buildroot}%{_mandir}/man1/netstat-nat.1

%clean
[ "$RPM_BUILD_ROOT" != "/" ] && rm -rf $RPM_BUILD_ROOT

%files
%defattr(-,root,root)
%doc COPYING README AUTHORS INSTALL ChangeLog NEWS
%{_bindir}/%{name}
%{_mandir}/man*/*

%changelog
