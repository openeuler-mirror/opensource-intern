Name:    nx-systemtray-applet
Version: 3.9.6
Release: 1%{?dist}
Summary: System tray for NX Desktop

License: GPL-3.0+
URL:     https://github.com/nx-desktop/nx-systemtray-applet.git
Source0: https://github.com/nx-desktop/%{name}.tar.gz


BuildRequires:  extra-cmake-modules
BuildRequires:  cmake
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-rpm-macros >= 5.25.0-2
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-kdeclarative-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-kxmlgui-devel
BuildRequires:  kf5-knotifications-devel
BuildRequires:  kf5-kdelibs4support-devel
Requires: plasma-workspace

%description
System tray replacemen for Plasma 5 used
in NX Desktop.


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
%{_kf5_qtplugindir}/plasma/applets/org.nx.private.systemtray.so
%{_kf5_qtplugindir}/plasma/applets/org.nx.systemtray.so
%{_kf5_datadir}/kservices5/plasma-applet-org.nx.private.systemtray.desktop
%{_kf5_datadir}/kservices5/plasma-applet-org.nx.systemtray.desktop
%{_kf5_metainfodir}/org.nx.systemtray.appdata.xml
%{_kf5_datadir}/plasma/plasmoids/org.nx.private.systemtray/
%{_kf5_datadir}/plasma/plasmoids/org.nx.systemtray/


%post
sed -i -- 's/X-KDE-PluginInfo-EnabledByDefault=true/X-KDE-PluginInfo-EnabledByDefault=false/g' /usr/share/kservices5/plasma-applet-org.kde.plasma.systemtray.desktop
%postun
sed -i -- 's/X-KDE-PluginInfo-EnabledByDefault=false/X-KDE-PluginInfo-EnabledByDefault=true/g' /usr/share/kservices5/plasma-applet-org.kde.plasma.systemtray.desktop
%changelog



