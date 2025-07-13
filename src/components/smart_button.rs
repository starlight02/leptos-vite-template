use leptos::prelude::*;

// 按钮变体枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

// 按钮尺寸枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Medium
    }
}

#[component]
pub fn SmartButton(
    variant: ButtonVariant,
    #[prop(default = ButtonSize::default())] size: ButtonSize,
    #[prop(into)] text: String,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let base_style = "border: none; border-radius: 6px; font-weight: 500; cursor: pointer; display: inline-flex; align-items: center; gap: 8px; transition: all 0.2s;";

    let variant_style = match variant {
        ButtonVariant::Primary => "background: #007bff; color: white;",
        ButtonVariant::Secondary => "background: #6c757d; color: white;",
        ButtonVariant::Success => "background: #28a745; color: white;",
        ButtonVariant::Warning => "background: #ffc107; color: #212529;",
        ButtonVariant::Danger => "background: #dc3545; color: white;",
    };

    let size_style = match size {
        ButtonSize::Small => "padding: 6px 12px; font-size: 12px;",
        ButtonSize::Medium => "padding: 10px 20px; font-size: 14px;",
        ButtonSize::Large => "padding: 14px 28px; font-size: 16px;",
    };

    let full_style = format!("{}{}{}", base_style, variant_style, size_style);

    view! {
        <button
            style=full_style
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback(());
                }
            }
        >
            <span>{text}</span>
            {children()}
        </button>
    }
}
