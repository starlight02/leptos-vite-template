use leptos::prelude::*;

// 搜索结果类型
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub score: f32,
}

#[component]
pub fn SearchBox(
    #[prop(into)] placeholder: String,
    on_search: Callback<String>,
    #[prop(optional)] on_clear: Option<Callback<()>>,
    #[prop(default = Vec::new())] results: Vec<SearchResult>,
) -> impl IntoView {
    let (query, set_query) = signal(String::new());

    let handle_input = move |ev| {
        let value = event_target_value(&ev);
        set_query.set(value.clone());

        if !value.trim().is_empty() {
            on_search(value);
        }
    };

    let handle_clear = move |_| {
        set_query.set(String::new());
        if let Some(callback) = on_clear {
            callback(());
        }
    };

    view! {
        <div style="position: relative; width: 100%; max-width: 500px;">
            <div style="position: relative;">
                <input
                    type="text"
                    placeholder=placeholder
                    style="width: 100%; padding: 12px 40px 12px 16px; border: 2px solid #ddd; border-radius: 8px; font-size: 16px; outline: none;"
                    prop:value=query
                    on:input=handle_input
                />

                <div style="position: absolute; right: 12px; top: 50%; transform: translateY(-50%); color: #666;">
                    "🔍"
                </div>

                <Show when=move || !query.get().is_empty()>
                    <button
                        style="position: absolute; right: 35px; top: 50%; transform: translateY(-50%); background: none; border: none; color: #999; cursor: pointer; font-size: 18px;"
                        on:click=handle_clear
                    >
                        "×"
                    </button>
                </Show>
            </div>

            <Show when=move || !results.is_empty() && !query.get().is_empty()>
                <div style="position: absolute; top: 100%; left: 0; right: 0; background: white; border: 1px solid #ddd; border-radius: 0 0 8px 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); z-index: 10; max-height: 300px; overflow-y: auto;">
                    <div style="padding: 8px; color: #666; font-size: 12px; border-bottom: 1px solid #eee;">
                        {move || format!("找到 {} 个结果", results.len())}
                    </div>
                    <For
                        each=move || results.clone()
                        key=|result| result.id
                        children=move |result| {
                            view! {
                                <div style="padding: 12px 16px; border-bottom: 1px solid #f5f5f5; cursor: pointer; hover:background-color: #f8f9fa;">
                                    <div style="font-weight: 500; color: #333; margin-bottom: 4px;">
                                        {result.title}
                                    </div>
                                    <div style="font-size: 14px; color: #666; line-height: 1.4;">
                                        {result.description}
                                    </div>
                                    <div style="font-size: 12px; color: #999; margin-top: 4px;">
                                        {format!("相关度: {:.1}%", result.score * 100.0)}
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}
