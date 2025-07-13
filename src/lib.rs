use leptos::prelude::*;
use leptos_router::{components::*, path};
use wasm_bindgen::prelude::wasm_bindgen;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::complex_props_demo::ComplexPropsDemo;
use crate::pages::home::Home;

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
                <a href="/">"Home"</a> " | "
                <a href="/complex-props-demo">"Complex Props Demo"</a>
            </nav>

            <Router>
                <Routes fallback=|| view! { <p>"Not Found!"</p> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/complex-props-demo") view=ComplexPropsDemo />
                </Routes>
            </Router>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    mount_to_body(|| view! { <App/> });
}
