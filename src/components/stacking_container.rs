use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos::*;
use leptos_use::{UseScrollReturn, use_scroll};
#[component]
pub fn StackingContainer(
    children: Children,
    node_ref: NodeRef<html::Div>,
    #[prop(optional, into)] class: Option<TextProp>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let scroll_y = RwSignal::new(0.0_f32);

    Effect::new(move |_| {
        let UseScrollReturn { y, .. } = use_scroll(node_ref);
        Effect::new(move |_| {
            scroll_y.set(y.get() as f32);
        });
    });

    provide_context(scroll_y);

    // Combine the base class "stacking-container" with any class passed by the user
    let final_class = move || {
        let mut classes = "stacking-container".to_string();
        if let Some(c) = class.as_ref() {
            let user_class = c.get();
            if !user_class.is_empty() {
                classes.push(' ');
                classes.push_str(&user_class);
            }
        }
        classes
    };

    view! {
        <div
            {..attrs}
            node_ref=node_ref
            class=final_class
        >
            {children()}
        </div>
    }
}

