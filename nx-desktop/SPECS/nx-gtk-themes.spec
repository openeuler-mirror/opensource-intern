Name:   nx-gtk-themes
Version:    3.9.6
Release: 1%{?dist}
Summary: GTK Themes used in Nitrux
License: GPLv3
URL:     https://github.com/nx-desktop/nx-gtk-themes.git
Source0: https://github.com/nx-desktop/nx-gtk-themes.tar.gz
BuildArch: noarch


%description
GTK themes to integrate GTK software with
NX Desktop.


%prep
%setup -q -n %{name}

%install
install -m 0644 -p -D nitrux/gtk-2.0/gtkrc %{buildroot}%{_datadir}/themes/nitrux/gtk-2.0/gtkrc
install -m 0755 -d %{buildroot}%{_datadir}/themes/nitrux/scss/apps
install -m 0644 -p nitrux/scss/apps/* %{buildroot}%{_datadir}/themes/nitrux/scss/apps
install -m 0755 -d %{buildroot}%{_datadir}/themes/nitrux/scss/widgets
install -m 0644 -p nitrux/scss/widgets/* %{buildroot}%{_datadir}/themes/nitrux/scss/widgets
install -m 0644 -p nitrux/scss/*.scss %{buildroot}%{_datadir}/themes/nitrux/scss
install -m 0644 -p nitrux/img.svg %{buildroot}%{_datadir}/themes/nitrux/img.svg
install -m 0644 -p nitrux/index %{buildroot}%{_datadir}/themes/nitrux/index
install -m 0644 -p nitrux/index.theme %{buildroot}%{_datadir}/themes/nitrux/index.theme

install -m 0644 -p -D nitrux-dark/gtk-2.0/gtkrc %{buildroot}%{_datadir}/themes/nitrux-dark/gtk-2.0/gtkrc
install -m 0755 -d %{buildroot}%{_datadir}/themes/nitrux-dark/scss/apps
install -m 0644 -p nitrux-dark/scss/apps/* %{buildroot}%{_datadir}/themes/nitrux-dark/scss/apps
install -m 0755 -d %{buildroot}%{_datadir}/themes/nitrux-dark/scss/widgets
install -m 0644 -p nitrux-dark/scss/widgets/* %{buildroot}%{_datadir}/themes/nitrux-dark/scss/widgets
install -m 0644 -p nitrux-dark/scss/*.scss %{buildroot}%{_datadir}/themes/nitrux-dark/scss
install -m 0644 -p nitrux-dark/img.svg %{buildroot}%{_datadir}/themes/nitrux-dark/img.svg
install -m 0644 -p nitrux-dark/index %{buildroot}%{_datadir}/themes/nitrux-dark/index
install -m 0644 -p nitrux-dark/index.theme %{buildroot}%{_datadir}/themes/nitrux-dark/index.theme

%files
%license LICENSE
%{_datadir}/themes/nitrux/
%{_datadir}/themes/nitrux-dark/
