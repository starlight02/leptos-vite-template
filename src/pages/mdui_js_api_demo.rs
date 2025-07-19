use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::mdui_components::MduiButton;
use crate::bindings::mdui::MduiService;

#[component]
pub fn MduiJsApiDemo() -> impl IntoView {
    // 使用现代化的异步 Tree Shaking 支持的 API
    let show_alert = move || {
        spawn_local(async move {
            if let Err(e) = MduiService::show_alert("提示", "这是通过现代化 Tree Shaking API 调用的对话框！仅加载所需模块！").await {
                log::error!("Alert error: {:?}", e);
            }
        });
    };

    let show_confirm = move || {
        spawn_local(async move {
            match MduiService::show_confirm(
                "确认操作",
                "你确定要继续吗？此操作使用了 Tree Shaking 优化。",
                Some("确定"),
                Some("取消")
            ).await {
                Ok(confirmed) => {
                    let message = if confirmed {
                        "你点击了确定按钮！"
                    } else {
                        "你点击了取消按钮。"
                    };
                    
                    spawn_local(async move {
                        if let Err(e) = MduiService::show_snackbar(message, Some("知道了"), Some(3000)).await {
                            log::error!("Snackbar error: {:?}", e);
                        }
                    });
                },
                Err(e) => log::error!("Confirm error: {:?}", e),
            }
        });
    };

    let show_snackbar = move || {
        spawn_local(async move {
            if let Err(e) = MduiService::show_snackbar(
                "这是一个 Snackbar 消息！支持 Tree Shaking 优化。",
                Some("关闭"),
                Some(4000)
            ).await {
                log::error!("Snackbar error: {:?}", e);
            }
        });
    };

    view! {
        <div class="js-api-demo">
            <h3>"现代化 MDUI API 演示（Tree Shaking 优化）"</h3>
            
            <div style="margin-bottom: 16px;">
                <p style="color: #666; font-size: 14px; margin: 0 0 16px 0;">
                    "🚀 此演示使用按需导入的 MDUI 功能，支持 Tree Shaking 优化！"
                </p>
            </div>

            <div style="display: flex; gap: 12px; flex-wrap: wrap;">
                <MduiButton variant="filled".to_string() on_click=Box::new(show_alert)>
                    "显示提示框"
                </MduiButton>

                <MduiButton variant="outlined".to_string() on_click=Box::new(show_confirm)>
                    "显示确认框"
                </MduiButton>
                
                <MduiButton variant="tonal".to_string() on_click=Box::new(show_snackbar)>
                    "显示消息条"
                </MduiButton>
            </div>
            
            <div style="margin-top: 24px; padding: 16px; background: #f5f5f5; border-radius: 8px;">
                <h4 style="margin: 0 0 8px 0; color: #333;">"✨ 技术优势"</h4>
                <ul style="margin: 0; padding-left: 20px; color: #666; font-size: 14px;">
                    <li>"仅打包实际使用的 MDUI 功能"</li>
                    <li>"减少最终 bundle 大小"</li>
                    <li>"类型安全的 Rust-JavaScript 互操作"</li>
                    <li>"现代化的 ES 模块导入方式"</li>
                    <li>"与 Vite 完美集成"</li>
                </ul>
            </div>
        </div>
    }
}
