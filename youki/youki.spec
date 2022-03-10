%global cargo_min_version 1.58.1
%global libgcc_min_version 8.5.0
%global glibc_min_version 2.3.0
%global share /usr/share
Name:   youki
Version:    0.0.2
Release:    1
Summary:    A container runtime written in Rust
License:    Apache
URL:    https://containers.github.io/youki
Source0:    https://github.com/containers/youki/archive/refs/tags/v%{version}/%{name}-%{version}.tar.gz
# https://github.com/containers/youki/archive/refs/tags/v0.0.2/youki-0.0.2.tar.gz
# https://github.com/containers/youki.git
# 注意从Source0 源下载的编译会出错 建议用 git clone 并且将其 tar zcvf youki-0.0.2.tar.gz youki 添加进SOURCES目录
# 注意官方明确表示cargo不得低于1.58.1
# ExclusiveArch:
# BuildRequires:  cargo >= %%{cargo_min_version}
BuildRequires:  cargo
BuildRequires:  pkg-config
BuildRequires:  systemd-devel
BuildRequires:  dbus-devel
BuildRequires:  dbus-glib
BuildRequires:  elfutils-libelf-devel
BuildRequires:  libseccomp-devel
Requires:   libseccomp-devel
Requires:   docker
Requires:   libgcc >= %{libgcc_min_version}
Requires:   glibc >= %{glibc_min_version}
Requires:   dbus

%description

%prep
%setup -q -n youki
cargo fetch --locked

%build
cargo build --release

%install
mkdir -p %{buildroot}/%{_bindir}
install -Dm755 target/release/youki  %{buildroot}/%{_bindir}/youki

install -dm755 %{buildroot}/usr/share/bash-completion/completions
./target/release/youki completion --shell bash > %{buildroot}/usr/share/bash-completion/completions/youki

# install -dm755 %%{buildroot}/usr/share/zsh/site-functions
# ./target/release/youki completions zsh > %%{buildroot}/usr/share/zsh/site-functions/_youki

# install -dm755 %%{buildroot}/usr/share/fish/vendor_functions.d
# ./target/release/youki completions fish > %%{buildroot}/usr/share/fish/vendor_functions.d/youki.fish

%check
# cargo test
./target/release/youki -h

%files
%license LICENSE
%{_bindir}/youki
%doc README.md
%{share}/bash-completion/completions/youki
