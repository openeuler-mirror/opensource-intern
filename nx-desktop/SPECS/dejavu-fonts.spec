Name:           dejavu-fonts
Version:        2.37
Release:        1%{?dist}
Summary:        DejaVu Truetype Fonts
License:        SUSE-Permissive
Url:            http://dejavu.sourceforge.net/
Source:         http://downloads.sourceforge.net/project/dejavu/dejavu/%{version}/%{name}-ttf-%{version}.tar.gz
BuildRequires:  fontpackages-devel
BuildArch:      noarch
Requires:       fontpackages-filesystem

%description
The DejaVu fonts are a font family based on the Bitstream Vera Fonts.
Its purpose is to provide a wider range of characters while maintaining
the original look and feel through the process of collaborative
development.

%prep
%setup -n %{name}-ttf-%{version}

%build

%install
install -m 0755 -d %{buildroot}%{_fontdir}
install -m 0644 -p ttf/*.ttf %{buildroot}%{_fontdir}
install -m 0755 -d %{buildroot}%{_fontconfig_templatedir} \
                   %{buildroot}%{_fontconfig_confdir}
install -m 0644 -p fontconfig/*.conf %{buildroot}%{_fontconfig_templatedir}
ln -s %{_fontconfig_templatedir}/20-unhint-small-dejavu-sans.conf \
      %{buildroot}%{_fontconfig_confdir}/20-unhint-small-dejavu-sans.conf

ln -s %{_fontconfig_templatedir}/20-unhint-small-dejavu-sans-mono.conf   \
      %{buildroot}%{_fontconfig_confdir}/20-unhint-small-dejavu-sans-mono.conf

ln -s %{_fontconfig_templatedir}/20-unhint-small-dejavu-serif.conf   \
      %{buildroot}%{_fontconfig_confdir}/20-unhint-small-dejavu-serif.conf

ln -s %{_fontconfig_templatedir}/57-dejavu-sans.conf  \
      %{buildroot}%{_fontconfig_confdir}/57-dejavu-sans.conf

ln -s %{_fontconfig_templatedir}/57-dejavu-sans-mono.conf  \
      %{buildroot}%{_fontconfig_confdir}/57-dejavu-sans-mono.conf

ln -s %{_fontconfig_templatedir}/57-dejavu-serif.conf  \
      %{buildroot}%{_fontconfig_confdir}/57-dejavu-serif.conf

%files 
%{_fontdir}/*.ttf
%{_fontconfig_templatedir}/*.conf
%{_fontconfig_confdir}/*.conf

%doc README.md
%license LICENSE

%changelog




