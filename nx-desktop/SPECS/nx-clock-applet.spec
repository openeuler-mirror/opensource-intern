Name:    nx-clock-applet
Version: 3.9.6
Release: 1%{?dist}
Summary: Clock, Calendar and Weather used in Nitrux.

License: GPL-2.0+
URL:     https://github.com/nx-desktop/nx-clock-applet.git
Source0: https://github.com/nx-desktop/%{name}.tar.gz

BuildRequires:  extra-cmake-modules
BuildRequires:  cmake
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
BuildRequires:  kf5-kwindowsystem-devel
Requires: plasma-workspace

%description
Clock, Calendar and Weather replacement for Plasma 5.


%prep
%setup -q -n %{name}


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} -DLIB_INSTALL_DIR=lib  ../
popd

%make_build -C %{_target_platform}


%install
make install/fast  DESTDIR=%{buildroot} -C %{_target_platform}

%files


%{_kf5_datadir}/kservices5/plasma-applet-org.nx.clock.desktop
%{_kf5_metainfodir}/org.nx.clock.appdata.xml
%{_kf5_datadir}/plasma/plasmoids/org.nx.clock/

%changelog


