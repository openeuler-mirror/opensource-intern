 
sudo dnf install ufw-0.35-17.noarch.rpm netstat-nat-1.4.10-1.x86_64.rpm dejavu-fonts-2.37-1.noarch.rpm applet-window-buttons-0.9.0-3.x86_64.rpm
sudo dnf install nx-audio-applet-3.9.6-1.x86_64.rpm \
                 nx-clock-applet-3.9.6-1.x86_64.rpm \
                 nx-desktop-settings-3.9.6-1.x86_64.rpm \
                 nx-firewall-3.9.6-1.x86_64.rpm \
                 nx-gtk-themes-3.9.6-1.noarch.rpm \
                 nx-networkmanagement-applet-3.9.6-1.x86_64.rpm \
                 nx-notifications-applet-3.9.6-1.x86_64.rpm \
                 nx-plasma-look-and-feel-3.9.6-1.x86_64.rpm \
                 nx-simplemenu-applet-3.9.6-1.x86_64.rpm \
                 nx-systemtray-applet-3.9.6-1.x86_64.rpm \
                 nx-window-deco-3.9.6-1.x86_64.rpm
sudo dnf install luv-icon-theme-3.9.6-1.noarch.rpm
sudo dnf install grub2-theme-nitrux-3.9.6-1.noarch.rpm
sudo dnf install google-noto-sans-cjk-sc-fonts.noarch  google-noto-sans-mono-cjk-sc-fonts.noarch google-noto-sans-sc-fonts.noarch  google-noto-serif-cjk-sc-fonts.noarch
sudo systemctl enable sddm
sudo systemctl set-default graphical.target

