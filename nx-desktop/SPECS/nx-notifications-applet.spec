Name:    nx-notifications-applet
Version: 3.9.6
Release: 1%{?dist}
Summary: Notifications manager for Nitrux

License: GPL-3.0+
URL:     https://github.com/nx-desktop/nx-notifications-applet.git
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
Requires: plasma-workspace

%description
NX Notifications is the notification manager
used in NX Desktop on Nitrux.


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
%{_kf5_datadir}/kservices5/plasma-applet-org.nx.notifications.desktop
%{_kf5_metainfodir}/org.nx.notifications.appdata.xml
%{_kf5_datadir}/plasma/plasmoids/org.nx.notifications/
%{_kf5_qtplugindir}/plasma/applets/nx_applet_notifications.so
%{_kf5_qmldir}/org/nx/private/notifications/libnotificationshelperplugin.so
%{_kf5_qmldir}/org/nx/private/notifications/qmldir

%post
sed -i -- 's/X-KDE-PluginInfo-EnabledByDefault=true/X-KDE-PluginInfo-EnabledByDefault=false/g' /usr/share/kservices5/kcm_notifications.desktop
sed -i -- 's/X-KDE-PluginInfo-EnabledByDefault=true/X-KDE-PluginInfo-EnabledByDefault=false/g' /usr/share/kservices5/plasma-dataengine-devicenotifications.desktop

%postun
sed -i -- 's/X-KDE-PluginInfo-EnabledByDefault=true/X-KDE-PluginInfo-EnabledByDefault=false/g' /usr/share/kservices5/kcm_notifications.desktop
sed -i -- 's/X-KDE-PluginInfo-EnabledByDefault=true/X-KDE-PluginInfo-EnabledByDefault=false/g' /usr/share/kservices5/plasma-dataengine-devicenotifications.desktop




%changelog


