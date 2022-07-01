#
# spec file for pijul
#
# Copyright (c) 2022. Huawei Technologies Co.,Ltd.ALL rights reserved.
# This program is licensed under Mulan PSL v2.
# You can use it according to the terms and conditions of the Mulan PSL v2.
#     http://license.coscl.org.cn/MulanPSL2
# THIS PROGRAM IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
# EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
# MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
# See the Mulan PSL v2 for more details.
#

%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: pijul
Summary: Distributed version control system based on a theory of patches
Version: 1.0.0~beta.2
Release: 1.0%{?dist}
License: Mulan-PSL-v2
Group: Development/Tools/Version Control
URL: https://pijul.org/
Packager: Lavandejoey(Liu Ziyi)<lavandejoey@outlook.com>
Source0: %{name}-%{version}.tar.gz
BuildRequires: clang-devel
BuildRequires: cmake
BuildRequires: pkg-config
BuildRequires: gcc
BuildRequires: openssl-devel
BuildRequires: libsodium
BuildRequires: libzstd-devel
BuildRequires: zstd-devel

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
Pijul is a distributed version control system. Its distinctive feature is to be
based on a theory of patches, which makes it really distributed.

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
