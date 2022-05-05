Name:           sddm
Version:        0.18.1
Release:        1%{?dist}
License:        GPLv2+
Summary:        QML based X11 desktop manager

Url:            https://github.com/sddm/sddm
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz
# 注意目前最新的0.19 对qt版本要求 >5.15

# Shamelessly stolen from gdm
Source1:       sddm.pam
# Shamelessly stolen from gdm
Source2:       sddm-autologin.pam
# systemd tmpfiles support for /var/run/sddm
Source3:       tmpfiles-sddm.conf
# sddm.conf 可通过如下方式生成 sddm --example-config > sddm.conf
Source4: sddm.conf
# README.scripts
Source5: README.scripts
# sysconfig snippet
Source6: sddm.sysconfig
# systemd sysusers config
Source7:  sddm-systemd-sysusers.conf

Provides: service(graphical-login) = sddm

BuildRequires:  cmake >= 2.8.8
BuildRequires:  extra-cmake-modules
BuildRequires:  libxcb-devel
BuildRequires:  pam-devel
BuildRequires:  pkgconfig
BuildRequires:  systemd-devel
BuildRequires:  python3-docutils
# sometimes python-docutils, sometimes python2-docutils, sometimes python3-docutils.
# use path then for sanity
BuildRequires:  /usr/bin/rst2man
BuildRequires:  qt5-qtbase-devel >= 5.6
BuildRequires:  qt5-qtdeclarative-devel >= 5.6
BuildRequires:  qt5-qttools-devel >= 5.6
# verify presence to pull defaults from /etc/login.defs
BuildRequires:  shadow-utils
BuildRequires:  systemd
#BuildRequires:  systemd-rpm-macros

Requires: systemd
Requires: xorg-x11-xinit
Requires: xorg-x11-server
Suggests: qt5-qtvirtualkeyboard%{?_isa}
%{?systemd_requires}

Requires(pre): shadow-utils

%description
SDDM is a modern display manager for X11 aiming to be fast, simple and
beautiful. It uses modern technologies like QtQuick, which in turn gives the
designer the ability to create smooth, animated user interfaces.

%package themes
Summary: SDDM Themes
Requires: %{name} = %{version}-%{release}
BuildArch: noarch
%description themes
A collection of sddm themes, including: elarun, maldives, maya


%prep
%setup -q


%build
mkdir build
cd build
%{cmake} .. \
  -DBUILD_MAN_PAGES:BOOL=ON \
  -DCMAKE_BUILD_TYPE:STRING="Release" \
  -DENABLE_JOURNALD:BOOL=ON \
  -DSESSION_COMMAND:PATH=/etc/X11/xinit/Xsession \
  -DWAYLAND_SESSION_COMMAND:PATH=/etc/sddm/wayland-session
cd ../

make %{?_smp_mflags} -C build

%install
make install DESTDIR=%{buildroot} -C build
mkdir -p %{buildroot}%{_sysconfdir}/sddm.conf.d
install -Dpm 644 %{SOURCE1} %{buildroot}%{_sysconfdir}/pam.d/sddm
install -Dpm 644 %{SOURCE2} %{buildroot}%{_sysconfdir}/pam.d/sddm-autologin
install -Dpm 644 %{SOURCE3} %{buildroot}%{_tmpfilesdir}/sddm.conf
install -Dpm 644 %{SOURCE4} %{buildroot}%{_sysconfdir}/sddm.conf
install -Dpm 644 %{SOURCE5} %{buildroot}%{_datadir}/sddm/scripts/README.scripts
install -Dpm 644 %{SOURCE6} %{buildroot}%{_sysconfdir}/sysconfig/sddm
install -Dpm 644 %{SOURCE7} %{buildroot}%{_sysusersdir}/sddm.conf
mkdir -p %{buildroot}/run/sddm
mkdir -p %{buildroot}%{_localstatedir}/lib/sddm
mkdir -p %{buildroot}%{_sysconfdir}/sddm/
cp -a %{buildroot}%{_datadir}/sddm/scripts/* \
      %{buildroot}%{_sysconfdir}/sddm/
# we're using /etc/X11/xinit/Xsession (by default) instead
rm -fv %{buildroot}%{_sysconfdir}/sddm/Xsession

%pre
# 安装软件包前执行的脚本，创建sddm用户组
getent group sddm >/dev/null || groupadd -r sddm
getent passwd sddm >/dev/null || \
    useradd -r -g sddm -d %{_localstatedir}/lib/sddm -s /sbin/nologin \
    -c "Simple Desktop Display Manager" sddm
exit 0

%post
# 安装软件包后执行的脚本
# systemctl preset
%systemd_post sddm.service

%preun
# 卸载软件包前执行脚本
# systemctl --no-reload disable systemctl stop
%systemd_preun sddm.service

%postun
# 卸载软件包后执行脚本
# systemctl daemon-reload
%systemd_postun sddm.service

%files
%license LICENSE
%doc README.md CONTRIBUTORS

#由此rpm拥有目录，用于卸载时清理
%dir %{_sysconfdir}/sddm/
%dir %{_sysconfdir}/sddm.conf.d

#指定以下为配置文件
%config(noreplace)   %{_sysconfdir}/sddm/*
%config(noreplace)   %{_sysconfdir}/sddm.conf
%config(noreplace)   %{_sysconfdir}/pam.d/sddm
%config(noreplace)   %{_sysconfdir}/pam.d/sddm-autologin
%config(noreplace)   %{_sysconfdir}/pam.d/sddm-greeter
%config(noreplace) %{_sysconfdir}/sysconfig/sddm

%{_sysconfdir}/dbus-1/system.d/org.freedesktop.DisplayManager.conf
%{_bindir}/sddm
%{_bindir}/sddm-greeter
%{_libexecdir}/sddm-helper
%{_tmpfilesdir}/sddm.conf
%{_sysusersdir}/sddm.conf
%attr(0711, root, sddm) %dir /run/sddm
%attr(1770, sddm, sddm) %dir %{_localstatedir}/lib/sddm
%{_unitdir}/sddm.service
%{_qt5_archdatadir}/qml/SddmComponents/
%dir %{_datadir}/sddm
%{_datadir}/sddm/faces/
%{_datadir}/sddm/flags/
%{_datadir}/sddm/scripts/
%dir %{_datadir}/sddm/themes/

%{_datadir}/sddm/translations/
%{_mandir}/man1/sddm.1*
%{_mandir}/man1/sddm-greeter.1*
%{_mandir}/man5/sddm.conf.5*
%{_mandir}/man5/sddm-state.conf.5*

%files themes
%{_datadir}/sddm/themes/elarun/
%{_datadir}/sddm/themes/maldives/
%{_datadir}/sddm/themes/maya/


%changelog

