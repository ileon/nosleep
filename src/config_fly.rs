use std::sync::LazyLock;

use config::Config;
use serde::Deserialize;

/// Global config
pub static CONFIG: LazyLock<MyConfig> = LazyLock::new(|| config_init());

/// Initialize config
fn config_init() -> MyConfig {
    //settings builder
    let settings = Config::builder()
        // 添加配置文件
        .add_source(config::File::with_name("./config.toml"))
        .build()
        .expect("settings build failed");
    //反序列化设置，生成设置的结构体
    let config: MyConfig = settings
        .try_deserialize()
        .expect("Deserialize config failed");
    config
}

#[derive(Debug, Deserialize)]
pub struct MyConfig {
    pub move_interval: u64,
    pub exit_after:u64,
    pub x_move:i32,
    pub y_move:i32,
}
