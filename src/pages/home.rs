use crate::components::counter_btn::Button;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to Leptos"</h1>

                <div class="buttons">
                    <Button />
                    <Button increment=5 />
                </div>

                <div style="margin-top: 30px; text-align: center;">
                    <a
                        href="/complex-props-demo"
                        style="display: inline-block; padding: 12px 24px; background: #007bff; color: white; text-decoration: none; border-radius: 8px; font-weight: 500; transition: background-color 0.2s;"
                    >
                        "🚀 查看复杂 Props 演示"
                    </a>
                    <p style="margin-top: 12px; color: #666; font-size: 14px;">
                        "探索结构体、枚举、回调函数等复杂类型的 Props 用法"
                    </p>
                </div>

            </div>
        </ErrorBoundary>
    }
}
