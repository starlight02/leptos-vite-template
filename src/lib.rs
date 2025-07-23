use leptos::prelude::*;
use leptos_router::{components::*, path};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

// Modules
mod bindings;
mod components;
mod env;
mod pages;

// 导入环境变量处理函数
use env::{get_app_title, get_base_url, get_debug_info};

// Top-Level pages
use crate::pages::complex_props_demo::ComplexPropsDemo;
use crate::pages::home::Home;
use crate::pages::mdui_demo::MduiDemo;
use crate::pages::mdui_js_api_demo::MduiJsApiDemo;

#[component]
pub fn App() -> impl IntoView {
    let base_url = get_base_url();
    let app_title = get_app_title();

    view! {
        <div class="app-container">
            <div class="hero-section">
                // 🖼️ 使用 public 目录中的静态资源（相对路径，兼容 GitHub Pages）
                <img src="icons/logo.svg" alt="Leptos Logo" class="leptos-logo" />
                <h1>{app_title}</h1>
                <p>"静态资源演示 - Logo 来自 /public/icons/logo.svg"</p>
                <p>
                    <small>"Base URL: " {base_url.clone()}</small>
                </p>
            </div>

            <Router base=base_url>
                <nav>
                    <A href="/">Home</A>
                    " | "
                    <A href="complex-props-demo">Complex Props Demo</A>
                    " | "
                    <A href="md-demo">Mdui Demo</A>
                    " | "
                    <A href="md-js-api-demo">Mdui Js Api Demo</A>
                </nav>

                <Routes fallback=|| view! { <p>"Not Found!"</p> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/complex-props-demo") view=ComplexPropsDemo />
                    <Route path=path!("/md-demo") view=MduiDemo />
                    <Route path=path!("/md-js-api-demo") view=MduiJsApiDemo />
                </Routes>
            </Router>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // 输出调试信息
    log::info!("{}", get_debug_info());

    let element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("leptos-app")
        .expect("Element with id 'leptos-app' not found");

    mount_to(element.unchecked_into(), || view! { <App /> }).forget();
}
