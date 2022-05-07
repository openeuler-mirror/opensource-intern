%global kf5_version 5.55.0

Name:    plasma-nm
Summary: Plasma for managing network connections
Version: 5.15.5
Release: 1%{?dist}

License: LGPLv2+ and GPLv2+
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/plasma-nm-5.15.5.tar.xz
## upstream patches

# master branch

# filter plugin provides
%global __provides_exclude_from ^(%{_kf5_qtplugindir}/.*\\.so)$

BuildRequires:  gettext

BuildRequires:  kf5-rpm-macros
BuildRequires:  extra-cmake-modules >= %{kf5_version}
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtdeclarative-devel
BuildRequires:  qt5-qttools-devel
BuildRequires:  qt5-qttools-static

BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  kf5-kservice-devel
BuildRequires:  kf5-kcompletion-devel
BuildRequires:  kf5-kwidgetsaddons-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-kwallet-devel
BuildRequires:  kf5-kitemviews-devel
BuildRequires:  kf5-kxmlgui-devel
BuildRequires:  kf5-kconfigwidgets-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-solid-devel
BuildRequires:  kf5-kdbusaddons-devel
BuildRequires:  kf5-knotifications-devel
BuildRequires:  kf5-plasma-devel
BuildRequires:  kf5-kdeclarative-devel
BuildRequires:  kf5-kinit-devel
BuildRequires:  kf5-kdelibs4support-devel
BuildRequires:  kf5-networkmanager-qt-devel >= %{kf5_version}
BuildRequires:  kf5-modemmanager-qt-devel >= %{kf5_version}

%if ! 0%{?bootstrap}
BuildRequires:  pkgconfig(ModemManager) >= 1.0.0
%endif
BuildRequires:  pkgconfig(libnm) >= 1.0.0
%if 0%{?fedora} || 0%{?epel}
BuildRequires:  pkgconfig(openconnect) >= 4.00
%endif

BuildRequires:  qca-qt5-devel

Requires:       NetworkManager >= 1.0.0

Obsoletes:      kde-plasma-networkmanagement < 1:0.9.1.0
Obsoletes:      kde-plasma-networkmanagement-libs < 1:0.9.1.0
Obsoletes:      kde-plasma-nm < 5.0.0-1
Provides:       kde-plasma-nm = %{version}-%{release}

%description
Plasma applet and editor for managing your network connections in KDE 4 using
the default NetworkManager service.

# Required for properly working GMS/CDMA connections
%package        mobile
Summary:        Mobile support for %{name}
Requires:       ModemManager
%if ! 0%{?bootstrap}
BuildRequires:  pkgconfig(mobile-broadband-provider-info)
%endif
Requires:       mobile-broadband-provider-info
Requires:       kf5-modemmanager-qt >= 5.0.0-1
Obsoletes:      kde-plasma-networkmanagement-mobile < 1:0.9.1.0
Obsoletes:      kde-plasma-nm-mobile < 5.0.0-1
Provides:       kde-plasma-nm-mobile = %{version}-%{release}
%description    mobile
%{summary}.


%package        openvpn
Summary:        OpenVPN support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       NetworkManager-openvpn
Obsoletes:      kde-plasma-networkmanagement-openvpn < 1:0.9.1.0
Obsoletes:      kde-plasma-nm-openvpn < 5.0.0-1
Provides:       kde-plasma-nm-openvpn = %{version}-%{release}
%description    openvpn
%{summary}.

%package        vpnc
Summary:        Vpnc support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       NetworkManager-vpnc
Obsoletes:      kde-plasma-networkmanagement-vpnc < 1:0.9.1.0
Obsoletes:      kde-plasma-nm-vpnc < 5.0.0-1
Provides:       kde-plasma-nm-vpnc = %{version}-%{release}
%description    vpnc
%{summary}.

%package        openconnect
Summary:        OpenConnect support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       NetworkManager-openconnect
Obsoletes:      kde-plasma-networkmanagement-openconnect < 1:0.9.1.0
Obsoletes:      kde-plasma-nm-openconnect < 5.0.0-1
Provides:       kde-plasma-nm-openconnect = %{version}-%{release}
%description    openconnect
%{summary}.

%package        wireguard
Summary:        wireguard support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       NetworkManager
Obsoletes:      kde-plasma-networkmanagement-wireguard < 1:0.9.1.0
Obsoletes:      kde-plasma-nm-wireguard < 5.0.0-1
Provides:       kde-plasma-nm-wireguard = %{version}-%{release}
%description    wireguard
%{summary}.


%package        openswan
Summary:        Openswan support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       NetworkManager-openswan
Obsoletes:      kde-plasma-nm-openswan < 5.0.0-1
Provides:       kde-plasma-nm-openswan = %{version}-%{release}
%description    openswan
%{summary}.

%package        strongswan
Summary:        Strongswan support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       strongswan
Obsoletes:      kde-plasma-nm-strongswan < 5.0.0-1
Provides:       kde-plasma-nm-strongswan = %{version}-%{release}
%description    strongswan
%{summary}.

%package        l2tp
Summary:        L2TP support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       NetworkManager-l2tp
Obsoletes:      kde-plasma-nm-l2tp < 5.0.0-1
Provides:       kde-plasma-nm-l2tp = %{version}-%{release}
%description    l2tp
%{summary}.

%package        pptp
Summary:        PPTP support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       NetworkManager-pptp
Obsoletes:      kde-plasma-networkmanagement-pptp < 1:0.9.1.0
Obsoletes:      kde-plasma-nm-pptp < 5.0.0-1
Provides:       kde-plasma-nm-pptp = %{version}-%{release}
%description    pptp
%{summary}.

%package        ssh
Summary:        SSH suppor for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       NetworkManager-ssh
%description    ssh
%{summary}.

%package        sstp
Summary:        SSTP support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
%description    sstp
%{summary}.

%package        iodine
Summary:        Iodine support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       NetworkManager-iodine
%description    iodine
%{summary}.

%package        fortisslvpn
Summary:        Fortigate SSL VPN support for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       NetworkManager-fortisslvpn
%description    fortisslvpn
%{summary}.


%prep
%autosetup -p1 -n %{name}-%{version}


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast  DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang plasma_applet_org.kde.plasma.networkmanagement
%find_lang plasmanetworkmanagement-kded
%find_lang plasmanetworkmanagement-kcm
%find_lang plasmanetworkmanagement-libs
%find_lang plasmanetworkmanagement_vpncui
%find_lang plasmanetworkmanagement_openvpnui
%find_lang plasmanetworkmanagement_openconnectui
%find_lang plasmanetworkmanagement_wireguardui
%find_lang plasmanetworkmanagement_openswanui
%find_lang plasmanetworkmanagement_strongswanui
%find_lang plasmanetworkmanagement_l2tpui
%find_lang plasmanetworkmanagement_pptpui
%find_lang plasmanetworkmanagement_sshui
%find_lang plasmanetworkmanagement_sstpui
%find_lang plasmanetworkmanagement_iodineui
%find_lang plasmanetworkmanagement_fortisslvpnui


%ldconfig_scriptlets

%files -f plasma_applet_org.kde.plasma.networkmanagement.lang -f plasmanetworkmanagement-kded.lang -f plasmanetworkmanagement-libs.lang -f plasmanetworkmanagement-kcm.lang
%{_libdir}/libplasmanm_internal.so
%{_libdir}/libplasmanm_editor.so
# plasma-nm applet
%{_qt5_qmldir}/org/kde/plasma/networkmanagement/
%{_kf5_datadir}/plasma/plasmoids/org.kde.plasma.networkmanagement/
%{_kf5_datadir}/kservices5/plasma-applet-org.kde.plasma.networkmanagement.desktop
#{_datadir}/plasma/updates/*.js
# plasma-nm notifications
%{_kf5_datadir}/knotifications5/networkmanagement.notifyrc
# plasma-nm kded
%{_kf5_plugindir}/kded/networkmanagement.so
# plasma-nm other
%{_kf5_datadir}/kservicetypes5/plasma-networkmanagement-vpnuiplugin.desktop
# appdata
%{_kf5_metainfodir}/org.kde.plasma.networkmanagement.appdata.xml

# kcm
%{_qt5_plugindir}/kcm_networkmanagement.so
%{_datadir}/kcm_networkmanagement/qml/
%{_kf5_datadir}/kservices5/kcm_networkmanagement.desktop
%{_datadir}/locale/*/LC_MESSAGES/kcm_mobile_broadband.mo
%{_datadir}/locale/*/LC_MESSAGES/kcm_mobile_wifi.mo

%files mobile


%files openvpn -f plasmanetworkmanagement_openvpnui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_openvpnui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_openvpnui.desktop

%files vpnc -f plasmanetworkmanagement_vpncui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_vpncui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_vpncui.desktop

%files wireguard -f plasmanetworkmanagement_wireguardui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_wireguardui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_wireguardui.desktop

%files openconnect -f plasmanetworkmanagement_openconnectui.lang
# %%{_kf5_qtplugindir}/libplasmanetworkmanagement_openconnectui.so
# %%{_kf5_datadir}/kservices5/plasmanetworkmanagement_openconnectui.desktop
# %%{_kf5_datadir}/kservices5/plasmanetworkmanagement_openconnect_juniperui.desktop
# %%{_kf5_datadir}/kservices5/plasmanetworkmanagement_openconnect_globalprotectui.desktop

%files openswan -f plasmanetworkmanagement_openswanui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_openswanui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_openswanui.desktop

%files strongswan -f plasmanetworkmanagement_strongswanui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_strongswanui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_strongswanui.desktop

%files l2tp -f plasmanetworkmanagement_l2tpui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_l2tpui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_l2tpui.desktop

%files pptp -f plasmanetworkmanagement_pptpui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_pptpui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_pptpui.desktop

%files ssh -f plasmanetworkmanagement_sshui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_sshui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_sshui.desktop

%files sstp -f plasmanetworkmanagement_sstpui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_sstpui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_sstpui.desktop

%files iodine -f plasmanetworkmanagement_iodineui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_iodineui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_iodineui.desktop

%files fortisslvpn -f plasmanetworkmanagement_fortisslvpnui.lang
%{_kf5_qtplugindir}/libplasmanetworkmanagement_fortisslvpnui.so
%{_kf5_datadir}/kservices5/plasmanetworkmanagement_fortisslvpnui.desktop



%changelog

