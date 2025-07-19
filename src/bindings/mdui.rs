// 现代化的 MDUI JavaScript 绑定
// 支持 Tree Shaking 和类型安全

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::*;
use web_sys::*;

// 使用传统的全局 MDUI API
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "mdui"])]
    fn alert(options: &JsValue) -> js_sys::Promise;
    
    #[wasm_bindgen(js_namespace = ["window", "mdui"])]
    fn confirm(options: &JsValue) -> js_sys::Promise;
    
    #[wasm_bindgen(js_namespace = ["window", "mdui"])]
    fn snackbar(options: &JsValue) -> js_sys::Promise;
}

// 辅助函数：创建选项对象
pub fn create_alert_options(headline: &str, description: &str) -> Result<JsValue, JsValue> {
    let options = Object::new();
    Reflect::set(&options, &"headline".into(), &headline.into())?;
    Reflect::set(&options, &"description".into(), &description.into())?;
    Ok(options.into())
}

pub fn create_confirm_options(
    headline: &str, 
    description: &str,
    confirm_text: Option<&str>,
    cancel_text: Option<&str>
) -> Result<JsValue, JsValue> {
    let options = Object::new();
    Reflect::set(&options, &"headline".into(), &headline.into())?;
    Reflect::set(&options, &"description".into(), &description.into())?;
    
    if let Some(confirm) = confirm_text {
        Reflect::set(&options, &"confirmText".into(), &confirm.into())?;
    }
    
    if let Some(cancel) = cancel_text {
        Reflect::set(&options, &"cancelText".into(), &cancel.into())?;
    }
    
    Ok(options.into())
}

pub fn create_snackbar_options(
    message: &str,
    action: Option<&str>,
    timeout: Option<u32>
) -> Result<JsValue, JsValue> {
    let options = Object::new();
    Reflect::set(&options, &"message".into(), &message.into())?;
    
    if let Some(action_text) = action {
        Reflect::set(&options, &"action".into(), &action_text.into())?;
    }
    
    if let Some(timeout_ms) = timeout {
        Reflect::set(&options, &"timeout".into(), &timeout_ms.into())?;
    }
    
    Ok(options.into())
}

// 高级 API 封装
pub struct MduiService;

impl MduiService {
    pub async fn show_alert(headline: &str, description: &str) -> Result<(), JsValue> {
        let options = create_alert_options(headline, description)?;
        let promise = alert(&options); // alert 现在返回 Promise
        JsFuture::from(promise).await?;
        Ok(())
    }
    
    pub async fn show_confirm(
        headline: &str, 
        description: &str,
        confirm_text: Option<&str>,
        cancel_text: Option<&str>
    ) -> Result<bool, JsValue> {
        let options = create_confirm_options(headline, description, confirm_text, cancel_text)?;
        let promise = confirm(&options);
        let result = JsFuture::from(promise).await?;
        Ok(result.as_bool().unwrap_or(false))
    }
    
    pub async fn show_snackbar(
        message: &str,
        action: Option<&str>,
        timeout: Option<u32>
    ) -> Result<(), JsValue> {
        let options = create_snackbar_options(message, action, timeout)?;
        let promise = snackbar(&options); // snackbar 现在也返回 Promise
        JsFuture::from(promise).await?;
        Ok(())
    }
}
