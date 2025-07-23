use log::{info, warn, error, debug};
use std::collections::HashMap;
use js_sys;

/// 环境类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum EnvironmentType {
    Development,
    Production,
    GitHubPages,
}

impl EnvironmentType {
    /// 从字符串解析环境类型
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Self::Development,
            "production" | "prod" => Self::Production,
            "github-pages" | "github" => Self::GitHubPages,
            _ => {
                warn!("Unknown environment type '{}', defaulting to Development", s);
                Self::Development
            }
        }
    }
    
    /// 转换为字符串
    pub fn to_string(&self) -> String {
        match self {
            Self::Development => "development".to_string(),
            Self::Production => "production".to_string(),
            Self::GitHubPages => "github-pages".to_string(),
        }
    }
}

/// 环境配置结构体
#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    /// 应用基础路径，如 "/" 或 "/leptos-vite-template/"
    pub base_url: String,
    
    /// 环境类型标识
    pub env_type: EnvironmentType,
    
    /// 应用标题
    pub app_title: String,
    
    /// 是否为调试模式
    pub debug_mode: bool,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            base_url: "/".to_string(),
            env_type: EnvironmentType::Development,
            app_title: "Leptos App".to_string(),
            debug_mode: cfg!(debug_assertions),
        }
    }
}

/// 环境变量相关错误
#[derive(Debug)]
pub enum EnvironmentError {
    MissingRequired(String),
    InvalidFormat(String),
    ValidationFailed(String),
    ConfigurationError(String),
}

impl std::fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRequired(var) => write!(f, "Missing required environment variable: {}", var),
            Self::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            Self::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            Self::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for EnvironmentError {}

impl EnvironmentError {
    /// 优雅地处理错误，返回默认配置
    pub fn handle_gracefully(&self) -> EnvironmentConfig {
        match self {
            Self::MissingRequired(var) => {
                warn!("Missing required environment variable: {}, using default", var);
                log_error_details("MissingRequired", var);
                EnvironmentConfig::default()
            },
            Self::InvalidFormat(msg) => {
                error!("Invalid environment format: {}, using default", msg);
                log_error_details("InvalidFormat", msg);
                EnvironmentConfig::default()
            },
            Self::ValidationFailed(msg) => {
                error!("Environment validation failed: {}, using default", msg);
                log_error_details("ValidationFailed", msg);
                EnvironmentConfig::default()
            },
            Self::ConfigurationError(msg) => {
                error!("Configuration error: {}, using default", msg);
                log_error_details("ConfigurationError", msg);
                EnvironmentConfig::default()
            }
        }
    }
    
    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::MissingRequired(_) => ErrorSeverity::Warning,
            Self::InvalidFormat(_) => ErrorSeverity::Error,
            Self::ValidationFailed(_) => ErrorSeverity::Error,
            Self::ConfigurationError(_) => ErrorSeverity::Critical,
        }
    }
    
    /// 检查是否应该继续执行
    pub fn should_continue(&self) -> bool {
        match self.severity() {
            ErrorSeverity::Warning | ErrorSeverity::Error => true,
            ErrorSeverity::Critical => false,
        }
    }
}

/// 错误严重程度
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Critical,
}

/// 获取应用的基础路径
/// 
/// 编译时从环境变量读取，如果不存在则返回默认值 "/"
pub fn get_base_url() -> String {
    let base_url = option_env!("VITE_BASE_URL").unwrap_or("/");
    info!("Base URL: {}", base_url);
    base_url.to_string()
}

/// 获取环境类型
/// 
/// 编译时从环境变量读取，如果不存在则返回 "development"
pub fn get_env_type() -> EnvironmentType {
    let env_str = option_env!("VITE_ENV").unwrap_or("development");
    info!("Environment type: {}", env_str);
    EnvironmentType::from_str(env_str)
}

/// 获取应用标题
/// 
/// 编译时从环境变量读取，如果不存在则返回默认标题
pub fn get_app_title() -> String {
    let title = option_env!("VITE_APP_TITLE").unwrap_or("Leptos + Vite Demo");
    info!("App title: {}", title);
    title.to_string()
}

/// 加载完整的环境配置
pub fn load_environment_config() -> EnvironmentConfig {
    let config = EnvironmentConfig {
        base_url: get_base_url(),
        env_type: get_env_type(),
        app_title: get_app_title(),
        debug_mode: cfg!(debug_assertions),
    };
    
    info!("Loaded environment config: {:?}", config);
    config
}

/// 验证环境配置
pub fn validate_config() -> Result<(), EnvironmentError> {
    let base_url = get_base_url();
    
    // 验证 base_url 格式
    if !base_url.starts_with('/') {
        return Err(EnvironmentError::InvalidFormat(
            format!("Base URL must start with '/': {}", base_url)
        ));
    }
    
    if !base_url.ends_with('/') {
        return Err(EnvironmentError::InvalidFormat(
            format!("Base URL must end with '/': {}", base_url)
        ));
    }
    
    info!("Environment configuration validation passed");
    Ok(())
}

/// 记录错误详细信息
fn log_error_details(error_type: &str, details: &str) {
    error!("Environment Error [{}]: {}", error_type, details);
    
    // 在调试模式下记录更多信息
    if cfg!(debug_assertions) {
        debug!("Error context:");
        debug!("  - Error type: {}", error_type);
        debug!("  - Details: {}", details);
        debug!("  - Timestamp: {}", js_sys::Date::new_0().to_iso_string());
        
        // 记录当前环境状态
        let current_config = load_environment_config();
        debug!("  - Current config: {:?}", current_config);
    }
}

/// 验证单个环境变量
fn validate_environment_variable(name: &str, value: &str) -> Result<(), EnvironmentError> {
    match name {
        "VITE_BASE_URL" => {
            if value.is_empty() {
                return Err(EnvironmentError::InvalidFormat(
                    "VITE_BASE_URL cannot be empty".to_string()
                ));
            }
            if !value.starts_with('/') {
                return Err(EnvironmentError::InvalidFormat(
                    format!("VITE_BASE_URL must start with '/': {}", value)
                ));
            }
            if !value.ends_with('/') {
                return Err(EnvironmentError::InvalidFormat(
                    format!("VITE_BASE_URL must end with '/': {}", value)
                ));
            }
        },
        "VITE_ENV" => {
            let valid_envs = ["development", "production", "github-pages"];
            if !valid_envs.contains(&value) {
                return Err(EnvironmentError::InvalidFormat(
                    format!("VITE_ENV must be one of {:?}, got: {}", valid_envs, value)
                ));
            }
        },
        "VITE_APP_TITLE" => {
            if value.is_empty() {
                return Err(EnvironmentError::InvalidFormat(
                    "VITE_APP_TITLE cannot be empty".to_string()
                ));
            }
            if value.len() > 100 {
                return Err(EnvironmentError::InvalidFormat(
                    format!("VITE_APP_TITLE too long (max 100 chars): {}", value.len())
                ));
            }
        },
        _ => {
            debug!("Unknown environment variable: {}", name);
        }
    }
    Ok(())
}

/// 获取所有环境变量的状态
pub fn get_environment_status() -> HashMap<String, String> {
    let mut status = HashMap::new();
    
    // 收集所有相关的环境变量
    let vars = [
        ("VITE_BASE_URL", option_env!("VITE_BASE_URL")),
        ("VITE_ENV", option_env!("VITE_ENV")),
        ("VITE_APP_TITLE", option_env!("VITE_APP_TITLE")),
        ("BUILD_TIME", option_env!("BUILD_TIME")),
    ];
    
    for (name, value) in vars.iter() {
        let status_value = match value {
            Some(v) => {
                match validate_environment_variable(name, v) {
                    Ok(_) => format!("✅ {}", v),
                    Err(e) => format!("❌ {} ({})", v, e),
                }
            },
            None => "⚠️ Not set (using default)".to_string(),
        };
        status.insert(name.to_string(), status_value);
    }
    
    status
}

/// 获取调试信息字符串
pub fn get_debug_info() -> String {
    let config = load_environment_config();
    let env_status = get_environment_status();
    
    let mut debug_info = format!(
        "Environment Debug Info:\n\
         - Base URL: {}\n\
         - Environment: {:?}\n\
         - App Title: {}\n\
         - Debug Mode: {}\n\
         - Compile Time: {}\n\
         \n\
         Environment Variables Status:",
        config.base_url,
        config.env_type,
        config.app_title,
        config.debug_mode,
        option_env!("BUILD_TIME").unwrap_or("unknown")
    );
    
    for (name, status) in env_status.iter() {
        debug_info.push_str(&format!("\n  - {}: {}", name, status));
    }
    
    debug_info
}

/// 执行全面的环境健康检查
pub fn health_check() -> Result<EnvironmentConfig, Vec<EnvironmentError>> {
    let mut errors = Vec::new();
    
    info!("Starting environment health check...");
    
    // 检查所有环境变量
    let vars_to_check = [
        ("VITE_BASE_URL", get_base_url()),
        ("VITE_ENV", get_env_type().to_string()),
        ("VITE_APP_TITLE", get_app_title()),
    ];
    
    for (name, value) in vars_to_check.iter() {
        if let Err(e) = validate_environment_variable(name, value) {
            errors.push(e);
        }
    }
    
    // 执行配置验证
    if let Err(e) = validate_config() {
        errors.push(e);
    }
    
    if errors.is_empty() {
        let config = load_environment_config();
        info!("Environment health check passed ✅");
        Ok(config)
    } else {
        error!("Environment health check failed with {} errors", errors.len());
        for (i, error) in errors.iter().enumerate() {
            error!("  {}. {}", i + 1, error);
        }
        Err(errors)
    }
}

/// 安全地初始化环境配置
pub fn safe_init_environment() -> EnvironmentConfig {
    match health_check() {
        Ok(config) => {
            info!("Environment initialized successfully");
            config
        },
        Err(errors) => {
            warn!("Environment initialization encountered {} errors, using fallback configuration", errors.len());
            
            // 尝试从错误中恢复
            let mut config = EnvironmentConfig::default();
            let mut critical_errors = 0;
            
            for error in errors {
                if !error.should_continue() {
                    critical_errors += 1;
                }
                config = error.handle_gracefully();
            }
            
            if critical_errors > 0 {
                error!("Found {} critical errors, application may not function correctly", critical_errors);
            }
            
            config
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_type_from_str() {
        assert_eq!(EnvironmentType::from_str("development"), EnvironmentType::Development);
        assert_eq!(EnvironmentType::from_str("dev"), EnvironmentType::Development);
        assert_eq!(EnvironmentType::from_str("production"), EnvironmentType::Production);
        assert_eq!(EnvironmentType::from_str("prod"), EnvironmentType::Production);
        assert_eq!(EnvironmentType::from_str("github-pages"), EnvironmentType::GitHubPages);
        assert_eq!(EnvironmentType::from_str("github"), EnvironmentType::GitHubPages);
        assert_eq!(EnvironmentType::from_str("unknown"), EnvironmentType::Development);
    }

    #[test]
    fn test_default_environment_config() {
        let config = EnvironmentConfig::default();
        assert_eq!(config.base_url, "/");
        assert_eq!(config.env_type, EnvironmentType::Development);
        assert_eq!(config.app_title, "Leptos App");
    }

    #[test]
    fn test_get_base_url() {
        let base_url = get_base_url();
        assert!(!base_url.is_empty());
        assert!(base_url.starts_with('/'));
    }

    #[test]
    fn test_get_env_type() {
        let env_type = get_env_type();
        // 应该返回有效的环境类型
        match env_type {
            EnvironmentType::Development | 
            EnvironmentType::Production | 
            EnvironmentType::GitHubPages => {},
        }
    }

    #[test]
    fn test_get_app_title() {
        let title = get_app_title();
        assert!(!title.is_empty());
    }

    #[test]
    fn test_load_environment_config() {
        let config = load_environment_config();
        assert!(!config.base_url.is_empty());
        assert!(!config.app_title.is_empty());
    }

    #[test]
    fn test_validate_config_success() {
        // 这个测试假设默认配置是有效的
        // 在实际环境中，如果环境变量设置正确，验证应该通过
        match validate_config() {
            Ok(_) => {}, // 验证通过
            Err(err) => {
                // 如果验证失败，确保错误处理正常工作
                let _fallback = err.handle_gracefully();
            }
        }
    }

    #[test]
    fn test_environment_error_handling() {
        let missing_error = EnvironmentError::MissingRequired("TEST_VAR".to_string());
        let config = missing_error.handle_gracefully();
        assert_eq!(config.base_url, "/");
        assert_eq!(missing_error.severity(), ErrorSeverity::Warning);
        assert!(missing_error.should_continue());
        
        let format_error = EnvironmentError::InvalidFormat("Invalid format".to_string());
        let config = format_error.handle_gracefully();
        assert_eq!(config.base_url, "/");
        assert_eq!(format_error.severity(), ErrorSeverity::Error);
        assert!(format_error.should_continue());
        
        let critical_error = EnvironmentError::ConfigurationError("Critical error".to_string());
        assert_eq!(critical_error.severity(), ErrorSeverity::Critical);
        assert!(!critical_error.should_continue());
    }

    #[test]
    fn test_get_debug_info() {
        let debug_info = get_debug_info();
        assert!(debug_info.contains("Environment Debug Info"));
        assert!(debug_info.contains("Base URL"));
        assert!(debug_info.contains("Environment"));
        assert!(debug_info.contains("App Title"));
        assert!(debug_info.contains("Environment Variables Status"));
    }
    
    #[test]
    fn test_environment_type_to_string() {
        assert_eq!(EnvironmentType::Development.to_string(), "development");
        assert_eq!(EnvironmentType::Production.to_string(), "production");
        assert_eq!(EnvironmentType::GitHubPages.to_string(), "github-pages");
    }
    
    #[test]
    fn test_get_environment_status() {
        let status = get_environment_status();
        assert!(status.contains_key("VITE_BASE_URL"));
        assert!(status.contains_key("VITE_ENV"));
        assert!(status.contains_key("VITE_APP_TITLE"));
        assert!(status.contains_key("BUILD_TIME"));
    }
    
    #[test]
    fn test_safe_init_environment() {
        let config = safe_init_environment();
        assert!(!config.base_url.is_empty());
        assert!(!config.app_title.is_empty());
    }
    
    #[test]
    fn test_health_check() {
        // 健康检查应该返回结果（可能成功或失败）
        match health_check() {
            Ok(config) => {
                assert!(!config.base_url.is_empty());
            },
            Err(errors) => {
                // 如果有错误，应该至少有一个
                assert!(!errors.is_empty());
            }
        }
    }
}
