use leptos::prelude::*;

#[component]
pub fn CallbackTest() -> impl IntoView {
    let (count, set_count) = signal(0);

    let increment_action = Action::new(move |amount| {
        let amount = *amount; // 需要解引用
        async move {
            set_count.update(|c| *c += amount);
        }
    });

    let reset_action = Action::new(move |_| {
        async move {
            // 使用 _ 表示我们接收但不使用这个参数
            set_count.set(0);
        }
    });

    view! {
        <div style="padding: 20px; border: 1px solid #ccc; margin: 10px;">
            <h3>"Action 测试组件"</h3>
            <p>"当前计数: " {count}</p>

            <div style="margin: 10px 0;">
                <button
                    style="margin: 5px; padding: 8px 16px;"
                    on:click=move |_| {
                        increment_action.dispatch(1);
                    }
                >
                    "+1"
                </button>

                <button
                    style="margin: 5px; padding: 8px 16px;"
                    on:click=move |_| {
                        increment_action.dispatch(5);
                    }
                >
                    "+5"
                </button>

                <button
                    style="margin: 5px; padding: 8px 16px;"
                    on:click=move |_| {
                        reset_action.dispatch(());
                    }
                >
                    "重置"
                </button>
            </div>

            // 显示 Action 状态
            <div style="margin: 10px 0; font-size: 14px; color: #666;">
                <p>
                    "增量操作状态: "
                    <span style=move || {
                        if increment_action.pending().get() {
                            "color: orange;"
                        } else {
                            "color: green;"
                        }
                    }>
                        {move || {
                            if increment_action.pending().get() { "进行中..." } else { "就绪" }
                        }}
                    </span>
                </p>
                <p>
                    "重置操作状态: "
                    <span style=move || {
                        if reset_action.pending().get() {
                            "color: orange;"
                        } else {
                            "color: green;"
                        }
                    }>
                        {move || {
                            if reset_action.pending().get() { "进行中..." } else { "就绪" }
                        }}
                    </span>
                </p>
            </div>

            // 使用作为 prop 的组件
            <ActionChild increment_action=increment_action reset_action=reset_action />
        </div>
    }
}

#[component]
pub fn ActionChild(
    increment_action: Action<i32, ()>,
    reset_action: Action<(), ()>,
) -> impl IntoView {
    view! {
        <div style="margin-top: 20px; padding: 15px; background: #f0f0f0; border-radius: 5px;">
            <h4>"子组件 - 使用 Action"</h4>
            <p>"这些按钮调用从父组件传来的 Action："</p>

            <button
                style="margin: 5px; padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 3px;"
                disabled=move || increment_action.pending().get()
                on:click=move |_| {
                    increment_action.dispatch(10);
                }
            >
                {move || {
                    if increment_action.pending().get() {
                        "+10 (处理中...)"
                    } else {
                        "+10 (从子组件)"
                    }
                }}
            </button>

            <button
                style="margin: 5px; padding: 8px 16px; background: #dc3545; color: white; border: none; border-radius: 3px;"
                disabled=move || reset_action.pending().get()
                on:click=move |_| {
                    reset_action.dispatch(());
                }
            >
                {move || {
                    if reset_action.pending().get() {
                        "重置 (处理中...)"
                    } else {
                        "重置 (从子组件)"
                    }
                }}
            </button>

            // 显示 Action 的执行次数
            <div style="margin-top: 15px; font-size: 12px; color: #888;">
                <p>"增量 Action 执行次数: " {move || increment_action.version().get()}</p>
                <p>"重置 Action 执行次数: " {move || reset_action.version().get()}</p>
            </div>
        </div>
    }
}
