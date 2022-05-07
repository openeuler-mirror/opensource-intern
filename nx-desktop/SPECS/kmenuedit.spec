%global kf5_version 5.55.0

Name:    kmenuedit
Summary: KDE menu editor
Version: 5.15.5
Release: 1%{?dist}

License: GPLv2+
URL:     https://cgit.kde.org/%{name}.git

%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global majmin_ver %(echo %{version} | cut -d. -f1,2).50
%global stable unstable
%else
%global majmin_ver %(echo %{version} | cut -d. -f1,2)
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/plasma/%{version}/%{name}-%{version}.tar.xz
# http://download.kde.org/stable/plasma/5.15.5/kmenuedit-5.15.5.tar.xz
BuildRequires:  qt5-qtbase-devel
BuildRequires:  qt5-qtscript-devel

BuildRequires:  desktop-file-utils
BuildRequires:  extra-cmake-modules >= %{kf5_version}
BuildRequires:  kf5-rpm-macros >= %{kf5_version}
BuildRequires:  kf5-kdbusaddons-devel >= %{kf5_version}
BuildRequires:  kf5-kdelibs4support-devel >= %{kf5_version}
BuildRequires:  kf5-kdoctools-devel >= %{kf5_version}
BuildRequires:  kf5-ki18n-devel >= %{kf5_version}
BuildRequires:  kf5-kiconthemes-devel >= %{kf5_version}
BuildRequires:  kf5-kinit-devel >= %{kf5_version}
BuildRequires:  kf5-kio-devel >= %{kf5_version}
BuildRequires:  kf5-kxmlgui-devel >= %{kf5_version}
BuildRequires:  kf5-sonnet-devel >= %{kf5_version}
BuildRequires:  kf5-kglobalaccel-devel >= %{kf5_version}
BuildRequires:  khotkeys-devel >= %{majmin_ver}

# libkdeinit5_*
%{?kf5_kinit_requires}

# when split out from kde-workspace-4.11.x
Conflicts:      kde-workspace < 4.11.15-3

%description
%{summary}.


%prep
%autosetup -p1


%build
mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}

%find_lang kmenuedit5 --with-html --all-name


%check
desktop-file-validate %{buildroot}%{_datadir}/applications/org.kde.kmenuedit.desktop

%files -f kmenuedit5.lang
%license COPYING*
%{_bindir}/kmenuedit
%{_kf5_libdir}/libkdeinit5_kmenuedit.so
%{_datadir}/kmenuedit/
%{_datadir}/applications/org.kde.kmenuedit.desktop
%{_datadir}/icons/hicolor/*/apps/kmenuedit.*
%{_kf5_datadir}/kxmlgui5/kmenuedit/
%{_sysconfdir}/xdg/kmenuedit.categories
#%%{_kf5_datadir}/qlogging-categories5//kmenuedit.categories
#%%{_libdir}/kconf_update_bin/kmenuedit_globalaccel

%changelog

