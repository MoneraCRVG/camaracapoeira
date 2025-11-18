use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::{AddAnyAttr, Children, ClassAttribute, ElementChild, NodeRef, NodeRefAttribute, StyleAttribute};
use leptos::{component, html, IntoView, view};
use std::fmt::Write;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum AlignItems {
    #[default]
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
}

impl AlignItems {
    const fn to_css_value(&self) -> &'static str {
        match self {
            Self::FlexStart => "flex-start",
            Self::Center => "center",
            Self::FlexEnd => "flex-end",
            Self::Stretch => "stretch",
            Self::Baseline => "baseline",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
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
    const fn to_css_value(&self) -> &'static str {
        match self {
            Self::FlexStart => "flex-start",
            Self::Center => "center",
            Self::FlexEnd => "flex-end",
            Self::SpaceBetween => "space-between",
            Self::SpaceAround => "space-around",
            Self::SpaceEvenly => "space-evenly",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

impl FlexWrap {
    const fn to_css_value(&self) -> &'static str {
        match self {
            Self::NoWrap => "nowrap",
            Self::Wrap => "wrap",
            Self::WrapReverse => "wrap-reverse",
        }
    }
}

#[component]
pub fn VStack(
    children: Children,
    #[prop(optional, into)] spacing: Option<String>,
    #[prop(optional)] align: Option<AlignItems>,
    #[prop(optional)] justify: Option<JustifyContent>,
    #[prop(optional)] wrap: Option<FlexWrap>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    #[prop(attrs, optional)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let base_style = move || {
        let mut styles = String::with_capacity(128);

        // These are the base styles for a vertical stack.
        let _ = write!(styles, "display:flex;flex-direction:column;");

        if let Some(val) = spacing.as_ref().filter(|v| !v.is_empty()) {
            let _ = write!(styles, "gap:{};", val);
        }
        if let Some(val) = align.as_ref() {
            let _ = write!(styles, "align-items:{};", val.to_css_value());
        }
        if let Some(val) = justify.as_ref() {
            let _ = write!(styles, "justify-content:{};", val.to_css_value());
        }
        if let Some(val) = wrap.as_ref() {
            let _ = write!(styles, "flex-wrap:{};", val.to_css_value());
        }
        
        if let Some(val) = style.as_ref().filter(|v| !v.is_empty()) {
            if !styles.ends_with(';') {
                styles.push(';');
            }
            styles.push_str(val);
        }

        styles
    };

    let class_list = move || {
        let base = "v-stack";
        class
            .as_ref()
            .map_or_else(|| base.to_string(), |c| format!("{} {}", base, c))
    };

    view! {
        <div
            {..attrs}
            node_ref=node_ref
            class=class_list
            style=base_style
        >
            {children()}
        </div>
    }
}

#[component]
pub fn VStackItem(
    children: Children,
    #[prop(optional)] align_self: Option<AlignItems>,
    #[prop(optional, into)] flex: Option<String>,
    #[prop(optional)] order: Option<i32>,
    #[prop(optional)] grow: Option<bool>,
    #[prop(optional)] shrink: Option<bool>,
    #[prop(optional, into)] basis: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    #[prop(attrs, optional)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let item_style = move || {
        let mut styles = String::with_capacity(128);

        if let Some(val) = align_self.as_ref() {
            let _ = write!(styles, "align-self:{};", val.to_css_value());
        }
        if let Some(val) = flex.as_ref().filter(|v| !v.is_empty()) {
            let _ = write!(styles, "flex:{};", val);
        }
        if let Some(val) = order {
            let _ = write!(styles, "order:{};", val);
        }
        if let Some(val) = grow {
            let _ = write!(styles, "flex-grow:{};", if val { "1" } else { "0" });
        }
        if let Some(val) = shrink {
            let _ = write!(styles, "flex-shrink:{};", if val { "1" } else { "0" });
        }
        if let Some(val) = basis.as_ref().filter(|v| !v.is_empty()) {
            let _ = write!(styles, "flex-basis:{};", val);
        }
        if let Some(val) = style.as_ref().filter(|v| !v.is_empty()) {
            styles.push_str(val);
        }

        styles
    };

    let class_list = move || {
        let base = "v-stack-item";
        class
            .as_ref()
            .map_or_else(|| base.to_string(), |c| format!("{} {}", base, c))
    };

    view! {
        <div
            {..attrs}
            class=class_list
            style=item_style
        >
            {children()}
        </div>
    }
}

