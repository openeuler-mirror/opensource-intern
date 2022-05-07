Name:           xsettingsd
Version:        1.0.2
Release:        1%{?dist}
Summary:        Provides settings to X11 clients via the XSETTINGS specification

License:        BSD
URL:            https://github.com/derat/xsettingsd

Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz
# https://github.com/derat/xsettingsd/archive/v1.0.2/xsettingsd-1.0.2.tar.gz
BuildRequires:  cmake
BuildRequires:  gcc-c++
BuildRequires:  libstdc++-devel
BuildRequires:  libX11-devel
# BuildRequires:  systemd-rpm-macros

%description
xsettingsd is a daemon that implements the XSETTINGS specification.

It is intended to be small, fast, and minimally dependent on other libraries.
It can serve as an alternative to gnome-settings-daemon for users who are not
using the GNOME desktop environment but who still run GTK+ applications and
want to configure things such as themes, font anti-aliasing/hinting, and UI
sound effects.

%prep
%setup -q

%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake} ..
popd
%make_build -C %{_target_platform}

%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}
#%%check
#%%ctest

%post
%systemd_post %{name}.service

%preun
%systemd_preun %{name}.service

%files
%license COPYING
%doc README.md
%{_bindir}/dump_xsettings
%{_bindir}/xsettingsd
%{_mandir}/man1/dump_xsettings.1*
%{_mandir}/man1/xsettingsd.1*
%{_userunitdir}/xsettingsd.service

%changelog

