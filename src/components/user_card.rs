use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// 定义用户数据结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub role: String,
}

// 定义主题配置结构
#[derive(Debug, Clone, PartialEq)]
pub struct CardConfig {
    pub theme: String,
    pub show_email: bool,
    pub show_role: bool,
}

impl Default for CardConfig {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            show_email: true,
            show_role: true,
        }
    }
}

#[component]
pub fn UserCard(
    user: User,
    #[prop(default = CardConfig::default())] config: CardConfig,
) -> impl IntoView {
    let card_class = format!("user-card theme-{}", config.theme);
    let user_name = user.name.clone();
    let user_email = user.email.clone();
    let user_role = user.role.clone();
    let has_avatar = user.avatar.is_some();
    let avatar_url = user.avatar.unwrap_or_default();
    let name_initial = user_name
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();

    view! {
        <div
            class=card_class
            style="border: 1px solid #ddd; border-radius: 8px; padding: 16px; margin: 8px; cursor: pointer; max-width: 300px;"
        >
            <div style="display: flex; align-items: center; gap: 12px;">
                <Show
                    when=move || has_avatar
                    fallback=move || {
                        view! {
                            <div style="width: 50px; height: 50px; border-radius: 50%; background: #f0f0f0; display: flex; align-items: center; justify-content: center; font-weight: bold;">
                                {name_initial.clone()}
                            </div>
                        }
                    }
                >
                    <img
                        src=avatar_url.clone()
                        alt="Avatar"
                        style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;"
                    />
                </Show>

                <div>
                    <h3 style="margin: 0; color: #333;">{user_name.clone()}</h3>

                    <Show when=move || config.show_email>
                        <p style="margin: 4px 0; color: #666; font-size: 14px;">
                            {user_email.clone()}
                        </p>
                    </Show>

                    <Show when=move || config.show_role>
                        <span style="background: #e7f3ff; color: #0066cc; padding: 2px 8px; border-radius: 12px; font-size: 12px;">
                            {user_role.clone()}
                        </span>
                    </Show>
                </div>
            </div>
        </div>
    }
}
