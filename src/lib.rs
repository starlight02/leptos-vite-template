use leptos::prelude::*;
use leptos_router::{components::*, path};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::complex_props_demo::ComplexPropsDemo;
use crate::pages::home::Home;
use crate::pages::mdui_demo::MduiDemo;
use crate::pages::mdui_js_api_demo::MduiJsApiDemo;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app-container">
            <div class="hero-section">
                // 🖼️ 使用 public 目录中的静态资源（绝对路径）
                <img src="/icons/logo.svg" alt="Leptos Logo" class="leptos-logo" />
                <h1>"Leptos + Vite SPA Demo1"</h1>
                <p>"静态资源演示 - Logo 来自 /public/icons/logo.svg"</p>
            </div>

            <nav>
                <a href="/">"Home"</a>
                " | "
                <a href="/complex-props-demo">"Complex Props Demo"</a>
                " | "
                <a href="/md-demo">"Mdui Demo"</a>
                " | "
                <a href="/md-js-api-demo">"Mdui Js Api Demo"</a>
            </nav>

            <Router>
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

    let element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("leptos-app")
        .expect("Element with id 'leptos-app' not found");
    
    mount_to(element.unchecked_into(), || view! { <App /> }).forget();
}
