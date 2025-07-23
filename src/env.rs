/// 简单的环境变量获取模块

/// 获取应用的基础路径
/// 
/// 编译时从环境变量读取，如果不存在则返回默认值 "/"
pub fn get_base_url() -> String {
    option_env!("VITE_BASE_URL").unwrap_or("/").to_string()
}

/// 获取应用标题
/// 
/// 编译时从环境变量读取，如果不存在则返回默认标题
pub fn get_app_title() -> String {
    option_env!("VITE_APP_TITLE").unwrap_or("Leptos + Vite Demo").to_string()
}

/// 获取环境类型字符串
/// 
/// 编译时从环境变量读取，如果不存在则返回 "development"
pub fn get_env() -> String {
    option_env!("VITE_ENV").unwrap_or("development").to_string()
}

/// 获取调试信息字符串
pub fn get_debug_info() -> String {
    format!(
        "Environment Info:\n\
         - Base URL: {}\n\
         - Environment: {}\n\
         - App Title: {}",
        get_base_url(),
        get_env(),
        get_app_title()
    )
}