use leptos::{ev, prelude::*};
use wasm_bindgen::prelude::*;
use web_sys::*;

// 2. 直接在 view! 宏中使用 MDUI Web Components
#[component]
pub fn MduiButton(
    #[prop(optional)] variant: Option<String>,
    #[prop(optional)] icon: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or_else(|| "filled".to_string());
    let disabled = disabled.unwrap_or(false);
    let class = class.unwrap_or_default();
    let icon = icon.unwrap_or_default();
    
    view! {
        <mdui-button
            variant=variant
            icon=icon
            disabled=disabled
            class=class
            on:click=move |_: ev::MouseEvent| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </mdui-button>
    }
}

#[component]
pub fn MduiCard(
    #[prop(optional)] variant: Option<String>,
    #[prop(optional)] clickable: Option<bool>,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or_else(|| "elevated".to_string());
    let clickable = clickable.unwrap_or(false);
    let class = class.unwrap_or_default();
    
    view! {
        <mdui-card variant=variant clickable=clickable class=class>
            {children()}
        </mdui-card>
    }
}

#[component]
pub fn MduiTextField(
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String)>>,
) -> impl IntoView {
    let label = label.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_default();
    let class = class.unwrap_or_default();
    
    // 如果没有提供 value signal，创建一个内部的
    let internal_value = RwSignal::new(String::new());
    let value_signal = value.unwrap_or(internal_value);
    
    view! {
        <mdui-text-field
            label=label
            placeholder=placeholder
            value=move || value_signal.get()
            class=class
            on:input=move |ev: ev::Event| {
                let target = ev.target().unwrap();
                let input = target.unchecked_into::<HtmlInputElement>();
                let new_value = input.value();
                value_signal.set(new_value.clone());
                if let Some(handler) = &on_input {
                    handler(new_value);
                }
            }
        >
        </mdui-text-field>
    }
}

#[component]
pub fn MduiDialog(
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional)] headline: Option<String>,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let headline = headline.unwrap_or_default();
    let class = class.unwrap_or_default();
    let open_signal = open.unwrap_or_else(|| RwSignal::new(false));
    
    view! {
        <mdui-dialog
            open=move || open_signal.get()
            headline=headline
            class=class
            on:close=move |_: ev::Event| {
                open_signal.set(false);
            }
        >
            {children()}
        </mdui-dialog>
    }
}

#[component]
pub fn MduiChip(
    #[prop(optional)] variant: Option<String>,
    #[prop(optional)] selectable: Option<bool>,
    #[prop(optional)] selected: Option<RwSignal<bool>>,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or_else(|| "assist".to_string());
    let selectable = selectable.unwrap_or(false);
    let class = class.unwrap_or_default();
    let selected_signal = selected.unwrap_or_else(|| RwSignal::new(false));
    
    view! {
        <mdui-chip
            variant=variant
            selectable=selectable
            selected=move || selected_signal.get()
            class=class
            on:click=move |_: ev::MouseEvent| {
                if selectable {
                    selected_signal.update(|s| *s = !*s);
                }
            }
        >
            {children()}
        </mdui-chip>
    }
}

#[component]
pub fn MduiSnackbar(
    #[prop(optional)] message: Option<String>,
    #[prop(optional)] action: Option<String>,
    #[prop(optional)] closeable: Option<bool>,
    #[prop(optional)] placement: Option<String>,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let message = message.unwrap_or_default();
    let action = action.unwrap_or_default();
    let closeable = closeable.unwrap_or(true);
    let placement = placement.unwrap_or_else(|| "bottom".to_string());
    let class = class.unwrap_or_default();
    let open_signal = open.unwrap_or_else(|| RwSignal::new(false));
    
    view! {
        <mdui-snackbar
            message=message
            action=action
            closeable=closeable
            placement=placement
            open=move || open_signal.get()
            class=class
            on:closed=move |_: ev::Event| {
                open_signal.set(false);
            }
        >
        </mdui-snackbar>
    }
}