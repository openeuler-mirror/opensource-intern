%global _hardened_build 1
%global srcname gpsd

%if 0%{?fedora} || 0%{?rhel} <= 7
%global with_qt 1
%else
%global with_qt 0
%endif

Name: %{srcname}
Version: 3.19
Release: 4%{?dist}.1
Summary: Service daemon for mediating access to a GPS

License: BSD
URL: http://catb.org/gpsd/
Source0: https://download-mirror.savannah.gnu.org/releases/gpsd/%{name}-%{version}.tar.gz
Source11: gpsd.sysconfig

BuildRequires: dbus-devel dbus-glib-devel ncurses-devel xmlto python3-devel
BuildRequires: python3-scons python3-gobject python3-cairo python3-pyserial
BuildRequires: desktop-file-utils bluez-libs-devel pps-tools-devel /usr/bin/c++
BuildRequires: systemd
%if %{with_qt}
BuildRequires: qt-devel
%endif
%ifnarch s390 s390x
BuildRequires: libusb1-devel
%endif

Requires: %{name}-libs%{?_isa} = %{version}-%{release}
Requires: udev
%{?systemd_requires}

%description 
gpsd is a service daemon that mediates access to a GPS sensor
connected to the host computer by serial or USB interface, making its
data on the location/course/velocity of the sensor available to be
queried on TCP port 2947 of the host computer.  With gpsd, multiple
GPS client applications (such as navigational and war-driving software)
can share access to a GPS without contention or loss of data.  Also,
gpsd responds to queries with a format that is substantially easier to
parse than NMEA 0183.  

%package libs
Summary: Client libraries in C for talking to a running gpsd or GPS

%description libs
This package contains the gpsd libraries that manage access
to a GPS for applications.

%package -n python3-%{srcname}
Summary: Python libraries and modules for use with gpsd
Requires: %{name}-libs%{?_isa} = %{version}-%{release}
%{?python_provide:%python_provide python3-%{srcname}}

%description -n python3-%{srcname}
This package contains the python3 modules that manage access to a GPS for
applications, and commonly useful python applications for use with gpsd.

%package devel
Summary: Development files for the gpsd library
Requires: %{name}-libs%{?_isa} = %{version}-%{release}

%description devel
This package provides C header files for the gpsd shared libraries that
manage access to a GPS for applications

%if %{with_qt}
%package qt
Summary: C++/Qt5 bindings for the gpsd library
Requires: %{name}-libs%{?_isa} = %{version}-%{release}

%description qt
This package provide C++ and Qt bindings for use with the libgps library from
gpsd.

%package qt-devel
Summary: Development files for the C++/Qt5 bindings for the gpsd library
Requires: %{name}-libs%{?_isa} = %{version}-%{release}
Requires: %{name}-qt%{?_isa} = %{version}-%{release}

%description qt-devel
This package provides the development files for the C++ and Qt bindings for use
with the libgps library from gpsd.
%endif

%package clients
Summary: Clients for gpsd
Requires: python3-%{srcname} = %{version}-%{release}
Requires: python3-pyserial
Requires: %{srcname}-libs%{?_isa} = %{version}-%{release}

%description clients
xgps is a simple test client for gpsd with an X interface. It displays
current GPS position/time/velocity information and (for GPSes that
support the feature) the locations of accessible satellites.

xgpsspeed is a speedometer that uses position information from the GPS.
It accepts an -h option and optional argument as for gps, or a -v option
to dump the package version and exit. Additionally, it accepts -rv
(reverse video) and -nc (needle color) options.

cgps resembles xgps, but without the pictorial satellite display.  It
can run on a serial terminal or terminal emulator.

gpsfake can feed data from files to simulate data coming from many
different gps devices.


%prep
%setup -q

# fix paths in systemd unit files
sed -i 's|/usr/local/sbin|%{_sbindir}|' systemd/*.service

# set gpsd revision string to include package revision
sed -i 's|^revision=.*REVISION.*$|revision='\'\
'#define REVISION "%{version}-%{release}'\"\'\| SConstruct

# fix systemd path
sed -i 's|systemd_dir =.*|systemd_dir = '\'%{_unitdir}\''|' SConstruct

# don't try reloading systemd when installing in the build root
sed -i 's|systemctl daemon-reload|true|' SConstruct

# don't set RPATH
sed -i 's|env.Prepend.*RPATH.*|pass #\0|' SConstruct

%build
export CCFLAGS="%{optflags}"
export LINKFLAGS="%{__global_ldflags}"

# breaks with %{_smp_mflags}
scons-3 \
	dbus_export=yes \
	systemd=yes \
	%if %{with_qt}
	libQgpsmm=yes \
	%else
	libQgpsmm=no \
	%endif
	debug=yes \
	leapfetch=no \
	prefix="" \
	sysconfdif=%{_sysconfdir} \
	bindir=%{_bindir} \
	includedir=%{_includedir} \
	libdir=%{_libdir} \
	sbindir=%{_sbindir} \
	mandir=%{_mandir} \
	docdir=%{_docdir} \
	pkgconfigdir=%{_libdir}/pkgconfig \
	udevdir=$(dirname %{_udevrulesdir}) \
	target_python=python3 \
	python_libdir=%{python3_sitearch} \
	build

# Fix python interpreter path.
sed -e "s,#!/usr/bin/\(python[23]\?\|env \+python[23]\?\),#!/usr/bin/python3,g" -i \
	gegps gpscat gpsfake xgps xgpsspeed gpsprof gps/*.py ubxtool zerk


%install
# avoid rebuilding
export CCFLAGS="%{optflags}"
export LINKFLAGS="%{__global_ldflags}"

DESTDIR=%{buildroot} scons-3 install systemd_install udev-install

# use the old name for udev rules
mv %{buildroot}%{_udevrulesdir}/{25,99}-gpsd.rules

%{__install} -d -m 0755 %{buildroot}%{_sysconfdir}/sysconfig
%{__install} -p -m 0644 %{SOURCE11} \
    %{buildroot}%{_sysconfdir}/sysconfig/gpsd

# Install the .desktop files
desktop-file-install \
    --dir %{buildroot}%{_datadir}/applications \
    packaging/X11/xgps.desktop
desktop-file-install \
    --dir %{buildroot}%{_datadir}/applications \
    packaging/X11/xgpsspeed.desktop

# Install logo icon for .desktop files
%{__install} -d -m 0755 %{buildroot}%{_datadir}/gpsd
%{__install} -p -m 0644 packaging/X11/gpsd-logo.png %{buildroot}%{_datadir}/gpsd/gpsd-logo.png

# Missed in scons install 
%{__install} -p -m 0755 gpsinit %{buildroot}%{_sbindir}

# If qt build was disabled, clean up the files that may have been installed
# anyway
%if !%{with_qt}
%{__rm} -f %{buildroot}%{_libdir}/libQgpsmm* \
    %{buildroot}%{_libdir}/pkgconfig/Qgpsmm* \
    %{buildroot}%{_mandir}/man3/libQgpsmm.3*
%endif

%post
%systemd_post gpsd.service gpsd.socket

%preun
%systemd_preun gpsd.service gpsd.socket

%postun
# Don't restart the service
%systemd_postun gpsd.service gpsd.socket

%ldconfig_scriptlets libs

%if %{with_qt}
%ldconfig_scriptlets qt
%endif

%files
%doc README COPYING
%config(noreplace) %{_sysconfdir}/sysconfig/%{name}
%{_sbindir}/gpsd
%{_sbindir}/gpsdctl
%{_sbindir}/gpsinit
%{_bindir}/gpsmon
%{_bindir}/gpsctl
%{_bindir}/ntpshmmon
%{_bindir}/ppscheck
%{_unitdir}/gpsd.service
%{_unitdir}/gpsd.socket
%{_unitdir}/gpsdctl@.service
%{_udevrulesdir}/*.rules
%{_mandir}/man8/gpsd.8*
%{_mandir}/man8/gpsdctl.8*
%{_mandir}/man8/gpsinit.8*
%{_mandir}/man8/ppscheck.8*
%{_mandir}/man1/gpsmon.1*
%{_mandir}/man1/gpsctl.1*
%{_mandir}/man1/ntpshmmon.1*

%files libs
%{_libdir}/libgps.so.25*

%files -n python3-%{srcname}
%{_bindir}/gpsprof
%{_mandir}/man1/gpsprof.1*
%{python3_sitearch}/gps*
%exclude %{python3_sitearch}/gps/fake*
%exclude %{python3_sitearch}/gps/__pycache__/fake*

%files devel
%doc TODO
%{_libdir}/libgps.so
%{_libdir}/pkgconfig/libgps.pc
%{_includedir}/gps.h
%{_includedir}/libgpsmm.h
%{_mandir}/man3/libgps.3*
%{_mandir}/man3/libgpsmm.3*
%{_mandir}/man5/gpsd_json.5*
%{_mandir}/man5/srec.5*

%if %{with_qt}
%files qt
%{_libdir}/libQgpsmm.so.25*

%files qt-devel
%{_libdir}/libQgpsmm.so
%{_libdir}/libQgpsmm.prl
%{_libdir}/pkgconfig/Qgpsmm.pc
%{_mandir}/man3/libQgpsmm.3*
%endif

%files clients
%{_bindir}/cgps
%{_bindir}/gegps
%{_bindir}/gps2udp
%{_bindir}/gpscat
%{_bindir}/gpsdecode
%{_bindir}/gpspipe
%{_bindir}/gpsrinex
%{_bindir}/gpxlogger
%{_bindir}/lcdgps
%{_bindir}/xgps
%{_bindir}/xgpsspeed
%{_bindir}/gpsfake
%{_bindir}/ubxtool
%{_bindir}/zerk
%{_mandir}/man1/gegps.1*
%{_mandir}/man1/gps.1*
%{_mandir}/man1/gps2udp.1*
%{_mandir}/man1/gpsdecode.1*
%{_mandir}/man1/gpspipe.1*
%{_mandir}/man1/gpsrinex.1*
%{_mandir}/man1/gpxlogger.1*
%{_mandir}/man1/lcdgps.1*
%{_mandir}/man1/xgps.1*
%{_mandir}/man1/xgpsspeed.1*
%{_mandir}/man1/cgps.1*
%{_mandir}/man1/gpscat.1*
%{_mandir}/man1/gpsfake.1*
%{_mandir}/man1/ubxtool.1*
%{_mandir}/man1/zerk.1*
%{_datadir}/applications/*.desktop
%dir %{_datadir}/gpsd
%{_datadir}/gpsd/gpsd-logo.png
%{python3_sitearch}/gps/fake*
%{python3_sitearch}/gps/__pycache__/fake*


%changelog



