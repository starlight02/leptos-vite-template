use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::*;

use crate::components::mdui_components::MduiButton;

// 4. 高级用法：直接使用 MDUI JavaScript API
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "mdui"])]
    fn alert(options: &JsValue);

    #[wasm_bindgen(js_namespace = ["window", "mdui"])]
    fn confirm(options: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "mdui"])]
    fn snackbar(options: &JsValue);
}

#[component]
pub fn MduiJsApiDemo() -> impl IntoView {
    let show_alert = Box::new(|| {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"headline".into(), &"提示".into()).unwrap();
        js_sys::Reflect::set(
            &options,
            &"description".into(),
            &"这是通过 JS API 调用的对话框".into(),
        )
        .unwrap();
        alert(&options.into());
    });

    let show_confirm = Box::new(|| {
        spawn_local(async move {
            let options = js_sys::Object::new();
            js_sys::Reflect::set(&options, &"headline".into(), &"确认".into()).unwrap();
            js_sys::Reflect::set(&options, &"description".into(), &"你确定要继续吗？".into())
                .unwrap();

            let result = confirm(&options.into());
            let _confirmed = wasm_bindgen_futures::JsFuture::from(result).await;
            // 处理确认结果
        });
    });

    view! {
        <div class="js-api-demo">
            <h3>"MDUI JavaScript API 演示"</h3>

            <MduiButton variant="filled".to_string() on_click=show_alert>
                "显示警告"
            </MduiButton>

            <MduiButton variant="outlined".to_string() on_click=show_confirm>
                "显示确认"
            </MduiButton>
        </div>
    }
}
