use leptos::prelude::*;
use leptos::{html, text_prop::TextProp};
use leptos::attr::any_attribute::AnyAttribute;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AlignItems {
    #[default]
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
}

impl AlignItems {
    pub fn to_css_value(self) -> &'static str {
        match self {
            AlignItems::FlexStart => "flex-start",
            AlignItems::Center => "center",
            AlignItems::FlexEnd => "flex-end",
            AlignItems::Stretch => "stretch",
            AlignItems::Baseline => "baseline",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum JustifyContent {
    #[default]
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl JustifyContent {
    pub fn to_css_value(self) -> &'static str {
        match self {
            JustifyContent::FlexStart => "flex-start",
            JustifyContent::Center => "center",
            JustifyContent::FlexEnd => "flex-end",
            JustifyContent::SpaceBetween => "space-between",
            JustifyContent::SpaceAround => "space-around",
            JustifyContent::SpaceEvenly => "space-evenly",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

impl FlexWrap {
    pub fn to_css_value(self) -> &'static str {
        match self {
            FlexWrap::NoWrap => "nowrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
        }
    }
}

#[component]
pub fn HStack(
    children: Children,
    #[prop(optional)] spacing: Option<String>,
    #[prop(optional, default = AlignItems::default())] align: AlignItems,
    #[prop(optional, default = JustifyContent::default())] justify: JustifyContent,
    #[prop(optional, default = FlexWrap::default())] wrap: FlexWrap,
    #[prop(optional, default = NodeRef::new())] node_ref: NodeRef<html::Div>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    #[prop(optional, into)] class: Option<TextProp>,
    #[prop(optional, into)] style: Option<TextProp>,
) -> impl IntoView {
    let base_style = move || {
        let mut styles = String::with_capacity(128);
        styles.push_str("display:flex;flex-direction:row;");

        if let Some(spacing_val) = &spacing {
            if !spacing_val.is_empty() {
                styles.push_str(&format!("gap:{};", spacing_val));
            }
        }

        styles.push_str(&format!("align-items:{};", align.to_css_value()));
        styles.push_str(&format!("justify-content:{};", justify.to_css_value()));
        styles.push_str(&format!("flex-wrap:{};", wrap.to_css_value()));

        styles
    };

    let final_class = move || {
        let mut classes = "h-stack".to_string();
        if let Some(c) = class.as_ref() {
            let user_class = c.get();
            if !user_class.is_empty() {
                classes.push(' ');
                classes.push_str(&user_class);
            }
        }
        classes
    };

    let final_style = move || {
        let mut styles = base_style();
        if let Some(s) = style.as_ref() {
            let user_style = s.get();
            if !user_style.is_empty() {
                if !styles.is_empty() && !styles.ends_with(';') {
                    styles.push(';');
                }
                styles.push_str(&user_style);
            }
        }
        styles
    };

    view! {
        <div
            {..attrs}
            node_ref=node_ref
            class=final_class
            style=final_style
        >
            {children()}
        </div>
    }
}

#[component]
pub fn HStackItem(
    children: Children,
    #[prop(optional)] grow: Option<bool>,
    #[prop(optional)] shrink: Option<bool>,
    #[prop(optional)] basis: Option<String>,
    #[prop(optional)] align: Option<String>,
    #[prop(optional)] order: Option<i32>,
    #[prop(optional, into)] class: Option<TextProp>,
    #[prop(optional, into)] style: Option<TextProp>,
) -> impl IntoView {
    let item_style = move || {
        let mut styles = String::with_capacity(64);

        if let Some(grow_val) = grow {
            styles.push_str(&format!("flex-grow:{};", if grow_val { "1" } else { "0" }));
        }

        if let Some(shrink_val) = shrink {
            styles.push_str(&format!("flex-shrink:{};", if shrink_val { "1" } else { "0" }));
        }

        if let Some(basis_val) = &basis {
            if !basis_val.is_empty() {
                styles.push_str(&format!("flex-basis:{};", basis_val));
            }
        }

        if let Some(align_val) = &align {
            if !align_val.is_empty() {
                styles.push_str(&format!("align-self:{};", align_val));
            }
        }

        if let Some(order_val) = order {
            styles.push_str(&format!("order:{};", order_val));
        }

        styles
    };

    let final_class = move || {
        let mut classes = "h-stack-item".to_string();
        if let Some(c) = class.as_ref() {
            let user_class = c.get();
            if !user_class.is_empty() {
                classes.push(' ');
                classes.push_str(&user_class);
            }
        }
        classes
    };

    let final_style = move || {
        let mut styles = item_style();
        if let Some(s) = style.as_ref() {
            let user_style = s.get();
            if !user_style.is_empty() {
                if !styles.is_empty() && !styles.ends_with(';') {
                    styles.push(';');
                }
                styles.push_str(&user_style);
            }
        }
        styles
    };

    view! {
        <div
            class=final_class
            style=final_style
        >
            {children()}
        </div>
    }
}

