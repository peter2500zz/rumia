use serde::de::DeserializeOwned;
use std::fs;
use windows_wrapper::mb;

pub fn load_config<T: DeserializeOwned>(path: &str) -> T {
    match serde_yaml::from_str::<T>(&fs::read_to_string(path).unwrap_or_default())
    {
        Ok(cfg) => cfg,
        Err(e) => {
            mb!("读取配置文件时出现错误\n{}: {}", path, e);
            panic!("{}", e)
        }
    }
}
