%global framework kfilemetadata

# Define to 1 to enable ffmpeg extractor
#global         ffmpeg 1

%if 0%{?fedora}
%global         ebook 1
%global         poppler 1
%global         taglib 1
%endif

Name:   kf5-%{framework}
Summary:    A Tier 2 KDE Framework for extracting file metadata
Version:    5.55.0
Release:    1%{?dist}

# # KDE e.V. may determine that future LGPL versions are accepted
License:    LGPLv2 or LGPLv3
URL:    https://cgit.kde.org/%{framework}

%global majmin %(echo %{version} | cut -d. -f1-2)
%global revision %(echo %{version} | cut -d. -f3)
%if %{revision} >= 50
%global stable unstable
%else
%global stable stable
%endif
Source0:        http://download.kde.org/%{stable}/frameworks/%{majmin}/%{framework}-%{version}.tar.xz

# filter plugin provides
%global __provides_exclude_from ^(%{_kf5_plugindir}/.*\\.so)$

BuildRequires:  extra-cmake-modules >= %{majmin}
BuildRequires:  kf5-karchive-devel >= %{majmin}
BuildRequires:  kf5-ki18n-devel >= %{majmin}
BuildRequires:  kf5-rpm-macros

BuildRequires:  qt5-qtbase-devel

BuildRequires:  libattr-devel
BuildRequires:  exiv2-devel >= 0.20

## optional deps
%if 0%{?ebook}
BuildRequires:  ebook-tools-devel
%endif
%if 0%{?ffmpeg}
BuildRequires:  ffmpeg-devel
%endif
%if 0%{?poppler}
BuildRequires:  pkgconfig(poppler-qt5)
%endif
%if 0%{?taglib}
BuildRequires:  pkgconfig(taglib) >= 1.9
%endif

%description
%{summary}.

%package devel
Summary:        Developer files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}
Requires:       qt5-qtbase-devel
%description devel
%{summary}.


%prep
%autosetup -n %{framework}-%{version} -p1


%build
mkdir build
cd build
%{cmake_kf5} ..
cd ../

make %{?_smp_mflags} -C build


%install
make install DESTDIR=%{buildroot} -C build

%find_lang %{name} --all-name

mkdir -p %{buildroot}%{_kf5_plugindir}/kfilemetadata/writers/

%ldconfig_scriptlets

%files -f %{name}.lang
%license COPYING.LGPL*
%{_sysconfdir}/xdg/%{framework}*
%{_kf5_libdir}/libKF5FileMetaData.so.*

# consider putting these into some subpkg ?
%dir %{_kf5_plugindir}/kfilemetadata/
%{_kf5_plugindir}/kfilemetadata/kfilemetadata_*.so
%dir %{_kf5_plugindir}/kfilemetadata/writers/
%if 0%{?taglib}
%{_kf5_plugindir}/kfilemetadata/writers/kfilemetadata_taglibwriter.so
%endif

%files devel
%{_kf5_libdir}/libKF5FileMetaData.so
%{_kf5_libdir}/cmake/KF5FileMetaData
%{_kf5_includedir}/KFileMetaData/
%{_kf5_archdatadir}/mkspecs/modules/qt_KFileMetaData.pri


%changelog



