Name:    kwrited
Summary: KDE Write Daemon
Version: 5.15.5
Release: 1%{?dist}

License: GPLv2+
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/kwrited-5.15.5.tar.xz
## upstreamable patches
#Patch0:         kwrited-call-setgroups.patch

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-rpm-macros
BuildRequires:  kf5-kpty-devel >= 5.13.0-2
BuildRequires:  kf5-kdelibs4support-devel
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtx11extras-devel

Requires:       kf5-filesystem

# Owns /usr/share/knotifications5
Requires:       kf5-knotifications

# TODO: Remove once kwrited is split from kde-workspace
Conflicts:      kde-workspace < 5.0.0-1

%description
%{summary}.


%prep
%setup -q -n %{name}-%{version}

#%%patch0 -p1 -b .setgroups


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

make %{?_smp_mflags} -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}


%files
%license COPYING
# kpty built with utempter support
%if 1
%{_kf5_qtplugindir}/kf5/kded/kwrited.so
%else
%{_kf5_bindir}/kwrited
%{_sysconfdir}/xdg/autostart/kwrited-autostart.desktop
%endif
%{_kf5_datadir}/knotifications5/kwrited.notifyrc


%changelog

