%global framework qqc2-desktop-style

Name:    %{framework}
Version: 5.55.0
Release: 1%{?dist}
Summary: QtQuickControls2 style for consistency between QWidget and QML apps

# kirigami-plasmadesktop-integration: LGPLv2+
# plugin,org.kde.desktop: LGPLv3 or GPLv3
License: (LGPLv3 or GPLv3) and LGPLv2+
URL:     https://cgit.kde.org/%{framework}.git

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0: http://download.kde.org/%{stable}/frameworks/%{majmin}/%{framework}-%{version}.tar.xz
# http://download.kde.org/stable/frameworks/5.55/qqc2-desktop-style-5.55.0.tar.xz
## upstream patches

BuildRequires: extra-cmake-modules >= %{majmin}
BuildRequires: kf5-rpm-macros
BuildRequires: kf5-kirigami2-devel >= %{majmin}
Requires: kf5-kirigami2%{?_isa} >= %{majmin}
BuildRequires: kf5-kconfigwidgets-devel >= %{majmin}
BuildRequires: kf5-kiconthemes-devel >= %{majmin}
# cmake() style
BuildRequires: cmake(KF5Kirigami2)
BuildRequires: cmake(KF5ConfigWidgets)
BuildRequires: cmake(KF5IconThemes)

BuildRequires: pkgconfig(Qt5Gui)
BuildRequires: pkgconfig(Qt5Quick)
BuildRequires: pkgconfig(Qt5Widgets)
# not sure if this is *really* needed or not -- rex
# CMakeLists.txt:# When building as a static plugin, dependencies may add a -lQt5X11Extras
BuildRequires: pkgconfig(Qt5X11Extras)

BuildRequires: qt5-qtquickcontrols2-devel
Requires: qt5-qtquickcontrols2%{?_isa}

# WORKAROUND FTBFS
%if 0%{?rhel}==7
BuildRequires: devtoolset-7-toolchain
BuildRequires: devtoolset-7-gcc-c++
%endif

%description
This is a style for QtQuickControls 2 that uses QWidget's QStyle for
painting, making possible to achieve an higher degree of consistency
between QWidget-based and QML-based apps.


%prep
%autosetup -n %{framework}-%{version} -p1


%build
%if 0%{?rhel}==7
. /opt/rh/devtoolset-7/enable
%endif

mkdir %{_target_platform}
pushd %{_target_platform}
%{cmake_kf5} ..
popd

%make_build -C %{_target_platform}


%install
make install/fast DESTDIR=%{buildroot} -C %{_target_platform}


%files
%doc README.md
%license LICENSE*
%dir %{_kf5_plugindir}/kirigami/
%{_kf5_plugindir}/kirigami/org.kde.desktop.so
%{_qt5_qmldir}/QtQuick/Controls.2/org.kde.desktop/
%{_qt5_qmldir}/org/kde/qqc2desktopstyle/
# yes, here
%{_kf5_libdir}/cmake/KF5QQC2DeskopStyle/
#%%{_kf5_libdir}/cmake/KF5QQC2DesktopStyle/


%changelog

