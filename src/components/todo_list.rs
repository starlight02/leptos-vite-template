use leptos::prelude::*;

// 待办事项结构
#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub text: String,
    pub completed: bool,
    pub priority: Priority,
    pub created_at: String,
}

// 优先级枚举
#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    pub fn to_color(&self) -> &'static str {
        match self {
            Priority::Low => "#28a745",
            Priority::Medium => "#ffc107",
            Priority::High => "#fd7e14",
            Priority::Urgent => "#dc3545",
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Priority::Low => "低",
            Priority::Medium => "中",
            Priority::High => "高",
            Priority::Urgent => "紧急",
        }
    }
}

#[component]
pub fn TodoList(
    items: Vec<TodoItem>,
    on_toggle: Callback<u32>,
    on_delete: Callback<u32>,
    #[prop(optional)] on_select: Option<Callback<u32>>,
) -> impl IntoView {
    let stats = move || {
        let total = items.len();
        let completed = items.iter().filter(|item| item.completed).count();
        let active = total - completed;
        (total, active, completed)
    };

    view! {
        <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
            <div style="background: #f8f9fa; padding: 16px; border-radius: 8px; margin-bottom: 20px;">
                <h2 style="margin: 0 0 12px 0; color: #333;">"待办事项列表"</h2>
                <div style="display: flex; gap: 20px; font-size: 14px; color: #666;">
                    <span>{move || format!("总计: {}", stats().0)}</span>
                    <span>{move || format!("未完成: {}", stats().1)}</span>
                    <span>{move || format!("已完成: {}", stats().2)}</span>
                </div>
            </div>

            <div style="space-y: 8px;">
                <Show
                    when=move || !items.is_empty()
                    fallback=move || {
                        view! {
                            <div style="text-align: center; padding: 40px; color: #666; font-style: italic;">
                                "暂无待办事项"
                            </div>
                        }
                    }
                >
                    <For
                        each=move || items.clone()
                        key=|item| item.id
                        children=move |item| {
                            let item_id = item.id;
                            let priority_color = item.priority.to_color();

                            view! {
                                <div
                                    style=format!(
                                        "border: 1px solid #ddd; border-radius: 8px; padding: 16px; margin-bottom: 8px; background: {}; transition: all 0.2s; cursor: pointer;",
                                        if item.completed { "#f8f9fa" } else { "white" },
                                    )
                                    on:click=move |_| {
                                        if let Some(select_callback) = on_select {
                                            select_callback(item_id);
                                        }
                                    }
                                >
                                    <div style="display: flex; align-items: center; gap: 12px;">
                                        <input
                                            type="checkbox"
                                            checked=item.completed
                                            style="width: 18px; height: 18px; cursor: pointer;"
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                on_toggle(item_id);
                                            }
                                        />

                                        <div
                                            style=format!(
                                                "width: 12px; height: 12px; border-radius: 50%; background: {}; flex-shrink: 0;",
                                                priority_color,
                                            )
                                            title=format!("优先级: {}", item.priority.to_string())
                                        ></div>

                                        <div style="flex: 1;">
                                            <div style=format!(
                                                "font-size: 16px; {};",
                                                if item.completed {
                                                    "text-decoration: line-through; color: #999;"
                                                } else {
                                                    "color: #333;"
                                                },
                                            )>{item.text}</div>
                                            <div style="font-size: 12px; color: #666; margin-top: 4px;">
                                                {format!(
                                                    "创建于: {} | 优先级: {}",
                                                    item.created_at,
                                                    item.priority.to_string(),
                                                )}
                                            </div>
                                        </div>

                                        <button
                                            style="background: #dc3545; color: white; border: none; border-radius: 4px; width: 24px; height: 24px; cursor: pointer; font-size: 12px; display: flex; align-items: center; justify-content: center;"
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                on_delete(item_id);
                                            }
                                            title="删除"
                                        >
                                            "×"
                                        </button>
                                    </div>
                                </div>
                            }
                        }
                    />
                </Show>
            </div>

            <Show when=move || {
                stats().0
            }>
                0>
                <div style="margin-top: 20px; padding: 16px; background: #f8f9fa; border-radius: 8px; text-align: center;">
                    <div style="font-size: 14px; color: #666;">
                        {move || {
                            let (_, active, _) = stats();
                            if active > 0 {
                                format!("还有 {} 个任务待完成！", active)
                            } else {
                                "🎉 所有任务都完成了！".to_string()
                            }
                        }}
                    </div>
                </div>
            </Show>
        </div>
    }
}
