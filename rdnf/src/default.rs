pub const RDNF_NAME: &str = "rdnf";
// pub const RDNF_CONF_FILE: &str = "/etc/rdnf/rdnf.conf";
pub const RDNF_CONF_FILE:&str="/etc/dnf/dnf.conf";
// pub const SYSTEM_LIBDIR: &str = "/usr/local/lib64";
pub const DEFAULT_REPO_LOCATION: &str = "/etc/yum.repos.d";
pub const DEFAULT_CACHE_LOCATION: &str = "/var/cache/rdnf";
// pub const DEFAULT_DATA_LOCATION:  &str = "/var/lib/rdnf";
// pub const HISTORY_DB_FILE:&str="history.db";
pub const DEFAULT_DISTROVERPKG: &str = "system-release";
pub const DEFAULT_PLUGIN_CONF_PATH: &str = "/etc/tdnf/pluginconf.d";
pub const DEFAULT_PLUGIN_PATH: &str = "/usr/local/lib64/tdnf-plugins";
pub const VAR_RELEASEVER: &str = "$releasever";
pub const VAR_BASEARCH: &str = "$basearch";
pub const SYSTEM_REPO_NAME: &str = "@System";
pub const CMDLINE_REPO_NAME: &str = "@cmdline";
pub const REPO_METADATA_MARKER: &str = "lastrefresh";
pub const REPODATA_DIR_NAME: &str = "repodata";
pub const SOLVCACHE_DIR_NAME: &str = "solvcache";
pub const RPM_CACHE_DIR_NAME: &str = "rpms";
pub const GPGKEY_CACHE_DIR_NAME:&str="keys";
pub const REPO_METADATA_FILE_NAME: &str = "repomd.xml";
pub const REPO_METADATA_FILE_PATH: &str = "repodata/repomd.xml";
pub const REPO_METALINK_FILE_NAME: &str = "metalink";
pub const REPO_BASEURL_FILE_NAME: &str = "baseurl";
// pub const PROGRESS_BAR_STYLE:&str="{{msg:{}}}{{spinner:.green}}[{{bar:{}.cyan/blue}}] {{bytes}}/{{total_bytes}} ({{bytes_per_sec}},{{eta}})";
pub const SOLV_COOKIE_IDENT: &str = "tdnf";
pub const SOLV_COOKIE_LEN: usize = 32;
pub const RDNF_INSTANCE_LOCK_FILE: &str = "/var/run/.rdnf-instance-lockfile";

// use console::Term;
// use lazy_static::lazy_static;
// lazy_static! {
//     pub static ref TERM_WIDTH:u16= {
//         let term = Term::stdout();
//         let (_,width)=term.size();
//         width
//     };
// }
