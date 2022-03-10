%global deno_arches x86_64 i686 aarch64
%global glibc_min_version 2.3.0
%global libgcc_min_version 8.5.0
%global cargo_min_version 1.58.1
Name:   deno
Version:    1.19.1
Release:    1
Summary:    A secure runtime for JavaScript and TypeScript
License:    MIT
URL:    https://deno.land
Source0:    https://github.com/denoland/deno/releases/download/v{version}/deno_src.tar.gz
ExclusiveArch:  %{deno_arches}
# BuildRequires:  cargo >= %%{cargo_min_version}
BuildRequires:  cargo
BuildRequires:  python3
Requires:   glibc >= %{glibc_min_version}
Requires:   libgcc >= %{libgcc_min_version}

%description
Deno is a simple, modern and secure runtime for JavaScript and TypeScript that uses V8 and is built in Rust.
Secure by default. No file, network, or environment access, unless explicitly enabled.
Supports TypeScript out of the box.
Ships only a single executable file.
Has built-in utilities like a dependency inspector (deno info) and a code formatter (deno fmt).
Has a set of reviewed (audited) standard modules that are guaranteed to work with Deno: deno.land/std
Has a number of companies interested in using and exploring Deno

%prep
%setup -q -n deno


%build
cargo build --release --locked

%install
mkdir -p %{buildroot}/%{_bindir}
install -Dm755 target/release/deno  %{buildroot}/%{_bindir}/deno

install -dm755 %{buildroot}/usr/share/bash-completion/completions
./target/release/deno completions bash > %{buildroot}/usr/share/bash-completion/completions/deno

# install -dm755 %%{buildroot}/usr/share/zsh/site-functions
# ./target/release/deno completions zsh > %%{buildroot}/usr/share/zsh/site-functions/_deno

# install -dm755 %%{buildroot}/usr/share/fish/vendor_functions.d
# ./target/release/deno completions fish > %%{buildroot}/usr/share/fish/vendor_functions.d/deno.fish

%check
./target/release/deno run ./cli/tests/testdata/002_hello.ts

%files
%license LICENSE.md
%{_bindir}/deno
%{_datadir}/bash-completion/completions/deno
# %%{share}/zsh/site-functions/_deno
# %%{share}/fish/vendor_functions.d/deno.fish
%changelog
* Thu Mar 3 2022 csmsoledad <2584139809@qq.com> - 1.19.1
- Package init
