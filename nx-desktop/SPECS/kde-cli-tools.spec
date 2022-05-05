Name:    kde-cli-tools
Version: 5.15.5
Release: 1%{?dist}
Summary: Tools based on KDE Frameworks 5 to better interact with the system

License: GPLv2+
URL:     https://invent.kde.org/plasma/%{name}

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global majmin_ver %(echo %{version} | cut -d. -f1,2).50
%global stable unstable
%else
%global majmin_ver %(echo %{version} | cut -d. -f1,2)
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/kde-cli-tools-5.15.5.tar.xz
## upstream patches

BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtsvg-devel
BuildRequires:  qt5-qtx11extras-devel

BuildRequires:  kf5-rpm-macros

BuildRequires:  extra-cmake-modules
BuildRequires:  kf5-kconfig-devel
BuildRequires:  kf5-kdoctools-devel
BuildRequires:  kf5-kiconthemes-devel
BuildRequires:  kf5-ki18n-devel
BuildRequires:  kf5-kcmutils-devel
BuildRequires:  kf5-kdesu-devel
BuildRequires:  kf5-kio-devel
BuildRequires:  kf5-kwindowsystem-devel
BuildRequires:  kf5-kactivities-devel
BuildRequires:  kf5-kdeclarative-devel
# todo: consider adjusting things to allow majmin
BuildRequires:  plasma-workspace-devel >= %{version}
Requires: libkworkspace5%{?_isa} >= %{version}

# upgrade path, from when this wasn't split out
Requires:       kdesu = 1:%{version}-%{release}

%description
Provides several KDE and Plasma specific command line tools to allow
better interaction with the system.

%package -n kdesu
Summary: Runs a program with elevated privileges
Epoch: 1
Conflicts: kde-runtime < 14.12.3-2
Conflicts: kde-runtime-docs < 14.12.3-2
## added deps below avoidable to due main pkg Requires: kdesu -- rex
# upgrade path, when kdesu was introduced
#Obsoletes: kde-cli-tools < 5.2.1-3
#Requires: %%{name} = %%{version}-%%{release}
%description -n kdesu
%{summary}.


%prep
%autosetup -p1


%build
mkdir build
cd build
%{cmake_kf5} ..
cd ../
make %{?_smp_mflags} -C build


%install
make install DESTDIR=%{buildroot} -C build
%find_lang kdeclitools_qt --with-qt --with-kde --all-name

ln -s %{_kf5_libexecdir}/kdesu %{buildroot}%{_bindir}/kdesu


%files -f kdeclitools_qt.lang
%{_bindir}/kcmshell5
%{_bindir}/kde-open5
%{_bindir}/kdecp5
%{_bindir}/kdemv5
%{_bindir}/keditfiletype5
%{_bindir}/kioclient5
%{_bindir}/kmimetypefinder5
%{_bindir}/kstart5
%{_bindir}/ksvgtopng5
%{_bindir}/ktraderclient5
%{_bindir}/kbroadcastnotification
# %%{_bindir}/kde-inhibit
# %%{_bindir}/plasma-open-settings
%{_kf5_libexecdir}/kdeeject
%{_kf5_qtplugindir}/kcm_filetypes.so
%{_kf5_libdir}/libkdeinit5_kcmshell5.so
%{_kf5_datadir}/kservices5/filetypes.desktop
%{_datadir}/doc/HTML/*/kcontrol5
# %%{_datadir}/applications/org.kde.keditfiletype.desktop
# %%{_datadir}/applications/org.kde.plasma.settings.open.desktop

%files -n kdesu
%{_bindir}/kdesu
%{_kf5_libexecdir}/kdesu
%{_mandir}/man1/kdesu.1.gz
%{_mandir}/*/man1/kdesu.1.gz
## FIXME: %%lang'ify
%{_datadir}/doc/HTML/*/kdesu


%changelog

