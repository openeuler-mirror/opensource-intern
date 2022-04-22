//! 配置文件

use serde::Deserialize;

/// Web配置
#[derive(Deserialize)]
pub struct WebConfig {
    /// Web服务监听地址
    pub addr: String,
}

#[derive(Deserialize)]
pub struct Config {
    /// Web配置
    pub web: WebConfig,
}

impl Config {
    /// 从环境变量中初始化配置
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        // 尝试合并环境变量设置
        cfg.merge(config::Environment::new())?;
        // 转换成我们自己的Config对象
        cfg.try_into()
    }
}
