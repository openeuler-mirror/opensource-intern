%global base_name nitrux-grub-theme
%global grub2dir /boot/grub2
Name:   grub2-theme-nitrux
Version:    3.9.6
Release: 1%{?dist}
Summary: GRand Unified Bootloader, version 2 (Nitrux theme)
License: GPLv3
URL:     https://github.com/Nitrux/nitrux-grub-theme.git
Source0: https://github.com/Nitrux/%{base_name}.tar.gz
Source1: grub
Requires:  grub2-common
Requires:  dejavu-fonts
BuildArch: noarch


%description
GRand Unified Bootloader, version 2 (Nitrux theme)
This is the default theme for GRUB's graphical menu.

GRUB_TERMINAL_INPUT=console
GRUB_GFXMODE=auto
GRUB_THEME="/usr/share/grub/themes/nitrux/theme.txt"


%prep
%setup -q -n nitrux



%install
install -m 0755 -d %{buildroot}%{_datadir}/grub/themes/nitrux
install -m 0644 -p ./* %{buildroot}%{_datadir}/grub/themes/nitrux

install -m 0755 -d %{buildroot}%{_sysconfdir}/default
install -m 0644 -p %{SOURCE1} %{buildroot}%{_sysconfdir}/default
%files
%{_datadir}/grub/themes/
%{_sysconfdir}/default/grub
%pre

%post
/usr/sbin/grub2-mkconfig -o %{grub2dir}/grub.cfg

%preun
mv /etc/default/grub.rpmsave /etc/default/grub

%postun
/usr/sbin/grub2-mkconfig -o %{grub2dir}/grub.cfg



