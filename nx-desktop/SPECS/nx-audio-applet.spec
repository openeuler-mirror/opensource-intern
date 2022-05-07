Name:    nx-audio-applet
Version: 3.9.6
Release: 1%{?dist}
Summary: Audio widget used in Nitrux.

License: GPL-3.0+
URL:     https://github.com/nx-desktop/nx-audio-applet.git
Source0: https://github.com/nx-desktop/%{name}.tar.gz

BuildRequires:  extra-cmake-modules
BuildRequires:  cmake
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
BuildRequires:  kf5-kwindowsystem-devel
Requires: plasma-pa

%description
Audio widget replacement for Plasma 5.


%prep
%setup -q -n %{name}


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5}  -DLIB_INSTALL_DIR=lib  ../
popd

%make_build -C %{_target_platform}


%install
make install/fast  DESTDIR=%{buildroot} -C %{_target_platform}
%files 
%{_kf5_datadir}/kservices5/plasma-applet-org.nx.audio.desktop
%{_kf5_metainfodir}/org.nx.audio.appdata.xml
%{_kf5_datadir}/plasma/plasmoids/org.nx.audio/

%post
sed -i -- 's/X-KDE-PluginInfo-EnabledByDefault=true/X-KDE-PluginInfo-EnabledByDefault=false/g' /usr/share/kservices5/plasma-applet-org.kde.plasma.volume.desktop

%postun
sed -i -- 's/X-KDE-PluginInfo-EnabledByDefault=false/X-KDE-PluginInfo-EnabledByDefault=true/g' /usr/share/kservices5/plasma-applet-org.kde.plasma.volume.desktop

%changelog

