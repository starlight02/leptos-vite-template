use crate::components::{
    callback_test::CallbackTest,
    user_card::{CardConfig, User, UserCard},
};
use leptos::prelude::*;

#[component]
pub fn ComplexPropsDemo() -> impl IntoView {
    // 定义用户数据 - 演示结构体类型的 Props
    let user1 = User {
        id: 1,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        avatar: Some("https://avatars.githubusercontent.com/u/1?v=4".to_string()),
        role: "开发者".to_string(),
    };

    let user2 = User {
        id: 2,
        name: "李四".to_string(),
        email: "lisi@example.com".to_string(),
        avatar: None,
        role: "设计师".to_string(),
    };

    let user3 = User {
        id: 3,
        name: "王五".to_string(),
        email: "wangwu@example.com".to_string(),
        avatar: Some("https://avatars.githubusercontent.com/u/2?v=4".to_string()),
        role: "产品经理".to_string(),
    };

    view! {
        <div style="min-height: 100vh; background: #f5f5f5; padding: 20px;">
            <div style="max-width: 1200px; margin: 0 auto;">

                <div style="text-align: center; margin-bottom: 40px;">
                    <h1 style="color: #333; margin-bottom: 8px;">"Leptos 复杂 Props 演示"</h1>
                    <p style="color: #666; font-size: 18px;">
                        "展示结构体类型的 Props 用法。测试文件监听功能。"
                    </p>
                </div>

                // 用户卡片部分
                <section style="margin-bottom: 50px;">
                    <h2 style="color: #333; margin-bottom: 20px; border-bottom: 2px solid #007bff; padding-bottom: 8px;">
                        "结构体 Props - 用户卡片组件"
                    </h2>
                    <div style="display: flex; flex-wrap: wrap; gap: 16px; justify-content: center;">
                        // 用户1 - 显示所有信息
                        <UserCard
                            user=user1
                            config=CardConfig {
                                theme: "light".to_string(),
                                show_email: true,
                                show_role: true,
                            }
                        />

                        // 用户2 - 隐藏邮箱
                        <UserCard
                            user=user2
                            config=CardConfig {
                                theme: "light".to_string(),
                                show_email: false,
                                show_role: true,
                            }
                        />

                        // 用户3 - 隐藏角色
                        <UserCard
                            user=user3
                            config=CardConfig {
                                theme: "light".to_string(),
                                show_email: true,
                                show_role: false,
                            }
                        />
                    </div>
                </section>

                // 回调函数测试部分
                <section style="margin-bottom: 50px;">
                    <h2 style="color: #333; margin-bottom: 20px; border-bottom: 2px solid #28a745; padding-bottom: 8px;">
                        "回调函数 Props - Callback 用法演示"
                    </h2>
                    <div style="background: white; padding: 30px; border-radius: 12px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                        <CallbackTest />
                    </div>
                </section>

                // 代码示例部分
                <section style="margin-bottom: 50px;">
                    <h2 style="color: #333; margin-bottom: 20px; border-bottom: 2px solid #28a745; padding-bottom: 8px;">
                        "代码示例"
                    </h2>
                    <div style="background: white; padding: 30px; border-radius: 12px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                        <h3 style="color: #333; margin-bottom: 16px;">
                            "1. 定义结构体类型"
                        </h3>
                        <pre style="background: #f8f9fa; padding: 16px; border-radius: 6px; overflow-x: auto; font-family: 'Courier New', monospace; font-size: 14px;">
                            {r#"#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
                            pub struct User {
                            pub id: u32,
                            pub name: String,
                            pub email: String,
                            pub avatar: Option<String>,
                            pub role: String,
                            }
                            
                            #[derive(Debug, Clone, PartialEq)]
                            pub struct CardConfig {
                            pub theme: String,
                            pub show_email: bool,
                            pub show_role: bool,
                            }"#}
                        </pre>

                        <h3 style="color: #333; margin: 24px 0 16px 0;">
                            "2. 组件 Props 定义"
                        </h3>
                        <pre style="background: #f8f9fa; padding: 16px; border-radius: 6px; overflow-x: auto; font-family: 'Courier New', monospace; font-size: 14px;">
                            {r#"#[component]
                            pub fn UserCard(
                            user: User,                                                  // 必需的结构体参数
                            #[prop(default = CardConfig::default())] config: CardConfig, // 带默认值的结构体参数
                            ) -> impl IntoView {"#}
                        </pre>

                        <h3 style="color: #333; margin: 24px 0 16px 0;">"3. Action 定义"</h3>
                        <pre style="background: #f8f9fa; padding: 16px; border-radius: 6px; overflow-x: auto; font-family: 'Courier New', monospace; font-size: 14px;">
                            {r#"// 创建 Action - 用于处理异步操作
                            let increment_action = Action::new({
                            let set_count = set_count.clone();
                            move |amount: &i32| {
                            let amount = *amount;
                            let set_count = set_count.clone();
                            async move {
                                set_count.update(|c| *c += amount);
                            }
                            }
                            });
                            
                            // 在组件中使用 Action
                            #[component]
                            pub fn MyComponent(
                            increment_action: Action<i32, ()>,  // 接收 Action 作为 prop
                            ) -> impl IntoView {
                            view! {
                            <button
                                disabled=move || increment_action.pending().get()
                                on:click=move |_| { increment_action.dispatch(5); }
                            >
                                "点击调用 Action"
                            </button>
                            }
                            }"#}
                        </pre>

                        <h3 style="color: #333; margin: 24px 0 16px 0;">"4. Action 的优势"</h3>
                        <pre style="background: #f8f9fa; padding: 16px; border-radius: 6px; overflow-x: auto; font-family: 'Courier New', monospace; font-size: 14px;">
                            {r#"// Action 提供了丰富的状态信息：
                            // 1. pending() - 是否正在执行
                            // 2. value() - 最后的返回值
                            // 3. version() - 执行次数
                            // 4. error() - 错误信息（如果有）
                            
                            view! {
                            <button
                            disabled=move || action.pending().get()
                            on:click=move |_| { action.dispatch(data); }
                            >
                            {move || if action.pending().get() {
                                "处理中..."
                            } else {
                                "点击执行"
                            }}
                            </button>
                            
                            <p>"执行次数: " {action.version()}</p>
                            }"#}
                        </pre>

                        <h3 style="color: #333; margin: 24px 0 16px 0;">"5. 使用组件"</h3>
                        <pre style="background: #f8f9fa; padding: 16px; border-radius: 6px; overflow-x: auto; font-family: 'Courier New', monospace; font-size: 14px;">
                            {r#"<UserCard
                            user=User {
                            id: 1,
                            name: "张三".to_string(),
                            email: "zhangsan@example.com".to_string(),
                            avatar: Some("avatar.jpg".to_string()),
                            role: "开发者".to_string(),
                            }
                            config=CardConfig {
                            theme: "light".to_string(),
                            show_email: true,
                            show_role: true,
                            }
                            />"#}
                        </pre>
                    </div>
                </section>

                // 特性说明部分
                <section style="margin-bottom: 30px;">
                    <h2 style="color: #333; margin-bottom: 20px; border-bottom: 2px solid #6f42c1; padding-bottom: 8px;">
                        "💡 关键特性说明"
                    </h2>
                    <div style="background: white; padding: 25px; border-radius: 12px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px;">
                            <div style="padding: 16px; border-left: 4px solid #007bff; background: #f8f9fa;">
                                <h4 style="color: #007bff; margin: 0 0 8px 0;">
                                    "结构体 Props"
                                </h4>
                                <p style="margin: 0; color: #666; font-size: 14px;">
                                    "使用自定义结构体作为 Props，支持复杂数据传递"
                                </p>
                            </div>
                            <div style="padding: 16px; border-left: 4px solid #28a745; background: #f8f9fa;">
                                <h4 style="color: #28a745; margin: 0 0 8px 0;">"Action 系统"</h4>
                                <p style="margin: 0; color: #666; font-size: 14px;">
                                    "使用 Action<Input, Output> 处理异步操作，提供状态跟踪和错误处理"
                                </p>
                            </div>
                            <div style="padding: 16px; border-left: 4px solid #17a2b8; background: #f8f9fa;">
                                <h4 style="color: #17a2b8; margin: 0 0 8px 0;">"可选字段"</h4>
                                <p style="margin: 0; color: #666; font-size: 14px;">
                                    "使用 Option<T> 类型定义可选的结构体字段"
                                </p>
                            </div>
                            <div style="padding: 16px; border-left: 4px solid #ffc107; background: #f8f9fa;">
                                <h4 style="color: #ffc107; margin: 0 0 8px 0;">"类型安全"</h4>
                                <p style="margin: 0; color: #666; font-size: 14px;">
                                    "编译时类型检查，避免运行时错误"
                                </p>
                            </div>
                        </div>
                    </div>
                </section>

            </div>
        </div>
    }
}
