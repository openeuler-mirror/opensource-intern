Name:    nx-kstyle-theme
Version: 3.9.6
Release: 1%{?dist}
Summary: A fork of Lightly to replace the Nitrux Kvantum themes.

License: GPL-3.0+
URL:     https://github.com/nx-desktop/nx-audio-applet.git
Source0: https://github.com/nx-desktop/%{name}.tar.gz

BuildRequires:  extra-cmake-modules
BuildRequires:  cmake
BuildRequires:  kf5-kconfig-devel
BuildRequires:  kdecoration-devel
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  kf5-kguiaddons-devel
BuildRequires:  kf5-kconfigwidgets-devel
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-frameworkintegration-devel
BuildRequires:  kf5-kcmutils-devel
BuildRequires:  kf5-frameworkintegration-libs
BuildRequires:  kf5-kwayland-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
Requires: plasma-pa

%description
Kstyle theme for Nitrux.


%prep
%setup -q -n %{name}


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} -DLIB_INSTALL_DIR=lib  ../
popd

make -C %{_target_platform}


%install
make install/fast  DESTDIR=%{buildroot} -C %{_target_platform}

%files

#%%{_kf5_datadir}/kservices5/plasma-applet-org.nx.audio.desktop
#%%{_kf5_metainfodir}/org.nx.audio.appdata.xml
#%%{_kf5_datadir}/plasma/plasmoids/org.nx.audio/
