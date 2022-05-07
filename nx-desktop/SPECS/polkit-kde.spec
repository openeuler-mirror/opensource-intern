%global         base_name polkit-kde-agent-1

Name:    polkit-kde
Summary: PolicyKit integration for KDE Desktop
Version: 5.15.5
Release: 1%{?dist}

License: GPLv2+
URL:     https://cgit.kde.org/%{base_name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{base_name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/polkit-kde-agent-1-5.15.5.tar.xz
BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-rpm-macros
BuildRequires:  qt5-qtbase-devel

BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  kf5-kdbusaddons-devel
BuildRequires:  kf5-kwidgetsaddons-devel
BuildRequires:  kf5-kcoreaddons-devel
BuildRequires:  kf5-kcrash-devel
BuildRequires:  kf5-kconfig-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-knotifications-devel
BuildRequires:  kf5-kdeclarative-devel

BuildRequires:  polkit-qt5-1-devel

Provides: PolicyKit-authentication-agent = %{version}-%{release}
Provides: polkit-kde-1 = %{version}-%{release}
Provides: polkit-kde-agent-1 = %{version}-%{release}

Obsoletes: PolicyKit-kde < 4.5

# Add explicit dependency on polkit, since polkit-libs were split out
Requires: polkit

%description
Provides Policy Kit Authentication Agent that nicely fits to KDE.


%prep
%autosetup -n %{base_name}-%{version}


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang polkit-kde-authentication-agent-1

# Move the agent from libexec to libexec/kf5
sed -i "s/Exec=\/usr\/libexec\//Exec=\/usr\/libexec\/kf5\//" %{buildroot}%{_sysconfdir}/xdg/autostart/polkit-kde-authentication-agent-1.desktop
mkdir -p %{buildroot}/%{_kf5_libexecdir}/
mv %{buildroot}/%{_libexecdir}/polkit-kde-authentication-agent-1 \
   %{buildroot}/%{_kf5_libexecdir}


%files -f polkit-kde-authentication-agent-1.lang
%license COPYING
%{_kf5_libexecdir}/polkit-kde-authentication-agent-1
%{_sysconfdir}/xdg/autostart/polkit-kde-authentication-agent-1.desktop
%{_kf5_datadir}/knotifications5/policykit1-kde.notifyrc
%{_kf5_datadir}/applications/org.kde.polkit-kde-authentication-agent-1.desktop


%changelog
