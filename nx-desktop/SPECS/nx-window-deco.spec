Name:    nx-window-deco
Version: 3.9.6
Release: 1%{?dist}
Summary: A fork of SierraBreezeEnhanced to replace the Nitrux Aurorae window decoration.

License: GPL-3.0+
URL:     https://github.com/nx-desktop/nx-window-deco.git
Source0: https://github.com/nx-desktop/%{name}.tar.gz

BuildRequires:  extra-cmake-modules
BuildRequires:  cmake
BuildRequires:  kdecoration-devel
BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-kguiaddons-devel
BuildRequires:  kf5-kconfigwidgets-devel
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  gettext
BuildRequires:  qt5-qtx11extras-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
Requires: kwin

%description
Window decoration for Nitrux.


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
%{_kf5_libdir}/libmauidecocommon5.so.0
%{_kf5_libdir}/libmauidecocommon5.so.0.9.5
%{_kf5_qtplugindir}/org.kde.kdecoration2/mauideco.so
%{_kf5_datadir}/kservices5/mauidecoconfig.desktop

%changelog



