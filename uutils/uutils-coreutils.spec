%global cargo_min_version 1.51.0
%global libgcc_min_version 8.5.0
%global usr /usr
Name:   uutils-coreutils
Version:    0.0.14
Release:    1
Summary:    Cross-platform Rust rewrite of the GNU coreutils
License:    MIT
URL:    https://github.com/uutils/coreutils
Source0:    https://github.com/uutils/coreutils/archive/refs/tags/0.0.14.tar.gz
# ExclusiveArch:
#BuildRequires:  cargo
BuildRequires:  pkg-config
BuildRequires:  python3-sphinx
BuildRequires:  libselinux-devel
BuildRequires:  gcc-c++
BuildRequires:  clang
BuildRequires:  clang-devel
Requires:   glibc
Requires:   libgcc
%global debug_package %{nil}
%description

%prep
%setup -q -n coreutils-0.0.14

%build
make PROFILE=release

%install
mkdir -p %{buildroot}/${local}
make install PROFILE=release  DESTDIR=%{buildroot} PREFIX=/usr

#%%check
#make test \
      PROFILE=release \
      CARGOFLAGS=--release
%files
%license LICENSE
%{usr}
%doc LICENSE



