#[cfg(feature="en_US")]
pub mod repo_list{
    pub const REPOLIST_REPO_ID:&str="repo id";
    pub const REPOLIST_REPO_NAME:&str="repo name";
    pub const REPOLIST_REPO_STATUS:&str="status";
    pub const REPOLIST_REPO_STATUS_ENABLED:&str="enabled";
    pub const REPOLIST_REPO_STATUS_DISABLED:&str="disabled";
    
}
#[cfg(feature="zh_CN")]
pub mod repo_list{
    pub const REPOLIST_REPO_ID:&str="仓库 id";
    pub const REPOLIST_REPO_NAME:&str="仓库名称";
    pub const REPOLIST_REPO_STATUS:&str="状态";
    pub const REPOLIST_REPO_STATUS_ENABLED:&str="启用";
    pub const REPOLIST_REPO_STATUS_DISABLED:&str="禁用";
}

#[cfg(feature="en_US")]
pub mod action_alter{
    pub const ACTION_ALTER_INTALL:&str="Installing";
    pub const ACTION_ALTER_UPGRADE:&str="Upgrading";
    pub const ACTION_ALTER_ERASE:&str="Removing";
    pub const ACTION_ALTER_DOWNGRADE:&str="Downgrading";
    pub const ACTION_ALTER_REINSTALL:&str="Reinstalling";
    pub const ACTION_ALTER_OBSOLETED:&str="Obsoleting";
}
#[cfg(feature="zh_CN")]
pub mod action_alter{
    pub const ACTION_ALTER_INTALL:&str="安装";
    pub const ACTION_ALTER_UPGRADE:&str="升级";
    pub const ACTION_ALTER_ERASE:&str="卸载";
    pub const ACTION_ALTER_DOWNGRADE:&str="降级";
    pub const ACTION_ALTER_REINSTALL:&str="重新安装";
    pub const ACTION_ALTER_OBSOLETED:&str="废弃";
}


#[cfg(feature="en_US")]
pub mod pkg_info{
    pub const PKG_INFO_NAME___:&str="Name       ";
    pub const PKG_INFO_ARCH___:&str="Arch       ";
    pub const PKG_INFO_EPOCH__:&str="Epoch      ";
    pub const PKG_INFO_VERSION:&str="Version    ";
    pub const PKG_INFO_RELEASE:&str="Release    ";
    pub const PKG_INFO_SIZE___:&str="Size       ";
    pub const PKG_INFO_REPO___:&str="Repo       ";
    pub const PKG_INFO_SUMMARY:&str="Summary    ";
    pub const PKG_INFO_URL____:&str="URL        ";
    pub const PKG_INFO_LICENSE:&str="License    ";
    pub const PKG_INFO_DESC___:&str="Description";
}
#[cfg(feature="zh_CN")]
pub mod pkg_info{
    pub const PKG_INFO_NAME___:&str="名称        ";
    pub const PKG_INFO_ARCH___:&str="架构        ";
    pub const PKG_INFO_EPOCH__:&str="时期        ";
    pub const PKG_INFO_VERSION:&str="版本        ";
    pub const PKG_INFO_RELEASE:&str="发布        ";
    pub const PKG_INFO_SIZE___:&str="大小        ";
    pub const PKG_INFO_REPO___:&str="仓库        ";
    pub const PKG_INFO_SUMMARY:&str="概况        ";
    pub const PKG_INFO_URL____:&str="URL        ";
    pub const PKG_INFO_LICENSE:&str="协议        ";
    pub const PKG_INFO_DESC___:&str="描述        ";
}







