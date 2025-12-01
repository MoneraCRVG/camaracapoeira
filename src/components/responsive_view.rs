use leptos::prelude::{signal, Effect, Get}; // Removed unused imports
use leptos::*;

/// A component that renders different views based on screen width.
#[component]
pub fn ResponsiveView<F, V>(
    /// A `Vec` of `(u32, F)` tuples, sorted by max_width (the `u32`).
    breakpoints: Vec<(u32, F)>,
    /// The view to render if no breakpoints match (i.e., width is > largest max_width).
    fallback: F,
) -> impl IntoView
where
    F: Fn() -> V + 'static + Send,
    V: IntoView + 'static,
{
    // Initialize signal with 0.
    let (width, _set_width) = signal(0_u32);

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::Closure;
            use wasm_bindgen::JsCast;
            use leptos::prelude::{StoredValue, on_cleanup, WithValue, Set};

            let window = leptos::leptos_dom::helpers::window();
            let window_clone = window.clone();

            let on_resize = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                let w = window_clone
                    .inner_width()
                    .expect("Failed to get inner width")
                    .as_f64()
                    .expect("Inner width is not a number") as u32;
                _set_width.set(w);
            }) as Box<dyn FnMut(_)>);

            let w = window
                .inner_width()
                .expect("Failed to get inner width")
                .as_f64()
                .expect("Inner width is not a number") as u32;
            _set_width.set(w);

            window
                .add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())
                .expect("Failed to add resize event listener");

            let stored_on_resize = StoredValue::new_local(on_resize);

            on_cleanup(move || {
                let window = leptos::leptos_dom::helpers::window();
                stored_on_resize.with_value(|on_resize_closure| {
                    window
                        .remove_event_listener_with_callback("resize", on_resize_closure.as_ref().unchecked_ref())
                        .expect("Failed to remove resize event listener");
                });
            });
        }
    });

    let view_to_render = move || {
        let current_width = width.get();
        let matching_view = breakpoints
            .iter()
            .find(|(max_width, _)| current_width < *max_width);

        if let Some((_, view_fn)) = matching_view {
            view_fn().into_view()
        } else {
            fallback().into_view()
        }
    };

    view! {
        {view_to_render}
    }
}