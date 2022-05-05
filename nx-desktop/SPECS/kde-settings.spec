Summary: Config files for kde
Name:    kde-settings
Version: 30.0
Release: 1%{?dist}

License: MIT
Url:     https://github.com/FedoraKDE/kde-settings
Source0: https://github.com/FedoraKDE/kde-settings/archive/%{version}/%{name}-%{version}.tar.gz
# https://github.com/FedoraKDE/kde-settings/archive/30.0/kde-settings-30.0.tar.gz
# Source1: COPYING

BuildArch: noarch

BuildRequires: kde-filesystem
BuildRequires: systemd

# when kdebugrc was moved here
Conflicts: kf5-kdelibs4support < 5.7.0-3

Obsoletes: kde-settings-ksplash < 24-2
Obsoletes: kde-settings-minimal < 24-3

Requires: kde-filesystem
# /etc/pam.d/ ownership
Requires: pam
Requires: xdg-user-dirs
## add breeze deps here? probably, need more too -- rex
Requires: breeze-icon-theme
#if 0%%{?fedora}
# for 11-fedora-kde-policy.rules
#Requires: polkit-js-engine
#endif

%description
%{summary}.

## FIXME
%package minimal
Summary: Minimal configuration files for KDE
Requires: %{name} = %{version}-%{release}
Requires: xorg-x11-xinit
%description minimal
%{summary}.

%package plasma
Summary: Configuration files for plasma
Requires: %{name} = %{version}-%{release}
#Requires: f30-backgrounds-kde
Requires: system-logos
%description plasma
%{summary}.

# FIXME/TODO: can probably consider dropping this subpkg now that we
# have good comps and soft dependencies support -- rex
%package pulseaudio
Summary: Enable pulseaudio support in KDE
# nothing here to license
License: Public Domain
Requires: %{name} = %{version}-%{release}
Requires: pulseaudio
Requires: pulseaudio-module-x11
## kde3
Requires: alsa-plugins-pulseaudio
## kde4: -pulseaudio plugins are installed for all phonon backends by default
%description pulseaudio
%{summary}.

%package -n qt-settings
Summary: Configuration files for Qt
# qt-graphicssystem.* scripts use lspci
#Requires: pciutils
%description -n qt-settings
%{summary}.


%prep
%autosetup -p1

# omit crud
rm -fv Makefile


%build
# Intentionally left blank.  Nothing to see here.


%install
mkdir -p %{buildroot}{/usr/share/config,/etc/kde/kdm}

tar cpf - . | tar --directory %{buildroot} -xvpf -

if [ %{_prefix} != /usr ] ; then
   pushd %{buildroot}
   mv %{buildroot}/usr %{buildroot}%{_prefix}
   mv %{buildroot}/etc %{buildroot}%{_sysconfdir}
   popd
fi


# cp -p %{SOURCE1} .

# omit kdm stuff
rm -rfv %{buildroot}%{_sysconfdir}/{kde/kdm,logrotate.d/kdm,pam.d/kdm*}
rm -fv %{buildroot}%{_localstatedir}/lib/kdm/backgroundrc
# we don't use %%{_tmpfilesdir} and %%{_unitdir} because they don't follow %{_prefix}
rm -fv %{buildroot}%{_prefix}/lib/tmpfiles.d/kdm.conf
rm -fv %{buildroot}%{_prefix}/lib/systemd/system/kdm.service

## unpackaged files
# formerly known as -minimal
rm -fv %{buildroot}%{_sysconfdir}/X11/xinit/xinitrc.d/20-kdedirs-minimal.sh
rm -fv %{buildroot}%{_sysconfdir}/profile.d/qt-graphicssystem.*

# FIXME/NEEDSWORK, still (mostly?) kde4
# rhel stuf
rm -rf %{buildroot}%{_sysconfdir}/kde/env/fedora-bookmarks.sh \
       %{buildroot}%{_prefix}/lib/rpm \
       %{buildroot}%{_datadir}/polkit-1/
echo "[Theme]" > %{buildroot}%{_datadir}/kde-settings/kde-profile/default/share/config/plasmarc
echo "name=RHEL7" >> %{buildroot}%{_datadir}/kde-settings/kde-profile/default/share/config/plasmarc
echo "[KSplash]" > %{buildroot}%{_datadir}/kde-settings/kde-profile/default/share/config/ksplashrc
echo "Theme=RHEL7" >> %{buildroot}%{_datadir}/kde-settings/kde-profile/default/share/config/ksplashrc
perl -pi -e "s,^Theme=.*,Theme=/usr/share/kde4/apps/kdm/themes/RHEL7," %{buildroot}%{_sysconfdir}/kde/kdm/kdmrc
perl -pi -e "s,^HomeURL=.*,HomeURL=file:///usr/share/doc/HTML/index.html," %{buildroot}%{_datadir}/kde-settings/kde-profile/default/share/config/konquerorrc
perl -pi -e "s,^View0_URL=.*,View0_URL=file:///usr/share/doc/HTML/index.html," %{buildroot}%{_datadir}/kde-settings/kde-profile/default/share/apps/konqueror/profiles/webbrowsing


%files
#%%license COPYING
%config(noreplace) %{_sysconfdir}/profile.d/kde.*
%{_sysconfdir}/kde/env/env.sh
%{_sysconfdir}/kde/env/gpg-agent-startup.sh
%{_sysconfdir}/kde/shutdown/gpg-agent-shutdown.sh
%{_sysconfdir}/kde/env/gtk2_rc_files.sh
%if 0%{?fedora} || 0%{?rhel} > 7
%{_sysconfdir}/kde/env/fedora-bookmarks.sh
%{_datadir}/kde-settings/
# these can probably go now -- rex
%{_prefix}/lib/rpm/plasma4.prov
%{_prefix}/lib/rpm/plasma4.req
%{_prefix}/lib/rpm/fileattrs/plasma4.attr
%{_datadir}/polkit-1/rules.d/11-fedora-kde-policy.rules
%endif
%config(noreplace) %{_sysconfdir}/xdg/kcm-about-distrorc
%config(noreplace) %{_sysconfdir}/xdg/kdebugrc
%config(noreplace) %{_sysconfdir}/pam.d/kcheckpass
%config(noreplace) %{_sysconfdir}/pam.d/kscreensaver
# drop noreplace, so we can be sure to get the new kiosk bits
%config %{_sysconfdir}/kderc
%config %{_sysconfdir}/kde4rc
%{_datadir}/applications/kde-mimeapps.list
%if 0%{?rhel} && 0%{?rhel} <= 7
%exclude %{_datadir}/kde-settings/kde-profile/default/share/apps/plasma-desktop/init/00-defaultLayout.js
%endif
%{_datadir}/kde-settings/kde-profile/

%files plasma
%{_datadir}/plasma/shells/org.kde.plasma.desktop/contents/updates/00-start-here-2.js
%{_sysconfdir}/xdg/plasma-workspace/env/env.sh
%{_sysconfdir}/xdg/plasma-workspace/env/gtk2_rc_files.sh
%{_sysconfdir}/xdg/plasma-workspace/env/gtk3_scrolling.sh
%{_sysconfdir}/xdg/plasma-workspace/shutdown/kuiserver5.sh
%{_datadir}/plasma/look-and-feel/org.fedoraproject.fedora.desktop/contents/plasmoidsetupscripts/org.kde.plasma.kicker.js
%{_datadir}/plasma/look-and-feel/org.fedoraproject.fedora.desktop/contents/plasmoidsetupscripts/org.kde.plasma.kickerdash.js
%{_datadir}/plasma/look-and-feel/org.fedoraproject.fedora.desktop/contents/plasmoidsetupscripts/org.kde.plasma.kickoff.js

%files pulseaudio
# nothing, this is a metapackage

%files -n qt-settings
# %license COPYING
%config(noreplace) %{_sysconfdir}/Trolltech.conf
#config(noreplace) %%{_sysconfdir}/profile.d/qt-graphicssystem.*


%changelog

