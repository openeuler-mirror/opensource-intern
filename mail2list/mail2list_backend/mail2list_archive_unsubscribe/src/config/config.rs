use std::env;
use regex::Regex;
use regex::Captures;
use std::borrow::Cow;
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerConfig{
        ///当前服务地址
    pub host: String,
    pub port: String,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MailConfig{
    pub mine_email : Vec<String>,
    pub password : Vec<String>,
    pub smtp_server : Vec<String>,
    pub imap_server : Vec<String>,
    pub archive_mine_email: Vec<String>,
    pub archive_password: Vec<String>,
    pub archive_smtp_server: Vec<String>,
    pub archive_imap_server: Vec<String>,
    pub archive_name: Vec<String>,
    pub leave_email : Vec<String>,
    pub leave_email_password : Vec<String>,
    pub leave_smtp_server : Vec<String>,
    pub leave_imap_server : Vec<String>,
    pub leave_name : Vec<String>,
}

///服务启动配置
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    /// 数据库地址
    pub database_url: String,
    /// 逻辑删除字段
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,
    ///日志目录 "target/logs/"
    pub log_dir: String,
    /// "100MB" 日志分割尺寸-单位KB,MB,GB
    pub log_temp_size: String,
    /// 日志打包格式可选“”（空-不压缩）“gzip”（gz压缩包）“zip”（zip压缩包）“lz4”（lz4压缩包（非常快））
    pub log_pack_compress: String,
    ///日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    pub log_rolling_type: String,
    ///日志等级
    pub log_level: String,
    //server 配置
    pub server:ServerConfig,
    //邮箱收发配置
    pub email:MailConfig,
}

///默认配置
impl Default for ApplicationConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../../mail2list_web/application.yml");
        let yml_config = expand_var(&yml_data).into_owned();
        //读取配置
        let result: ApplicationConfig =
            serde_yaml::from_str(&yml_config).expect("配置文件加载失败");

        result
    }
}

fn expand_var(raw_config: &str) -> Cow<str> {

    let re = Regex::new(r"\$\{([a-zA-Z_][0-9a-zA-Z_]*)\}").unwrap();
    re.replace_all(&raw_config, |caps: &Captures| {
        match env::var(&caps[1]) {
            Ok(val) => val,
            Err(_) => (&caps[0]).to_string(),
        }
    })
}
