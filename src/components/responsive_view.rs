// 1. Import `StoredValue` and `on_cleanup`. Remove `store_value`.
use leptos::prelude::{signal, Effect, Get, Set, StoredValue, on_cleanup, WithValue};
use leptos::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

/// A component that renders different views based on screen width.
///
/// It takes a `Vec` of (max_width, view_function) tuples and a `fallback` view.
/// The views are evaluated in order, and the first one where
/// `current_width < max_width` is rendered.
/// If no breakpoint matches, the `fallback` is rendered.
///
/// The breakpoints should be sorted by `max_width` in ascending order.
#[component]
pub fn ResponsiveView<F, V>(
    /// A `Vec` of `(u32, F)` tuples, sorted by max_width (the `u32`).
    /// `F` is a closure that returns a view.
    breakpoints: Vec<(u32, F)>,
    /// The view to render if no breakpoints match (i.e., width is > largest max_width).
    fallback: F,
) -> impl IntoView
where
    // 2. Added `+ Send` to the trait bound for F
    F: Fn() -> V + 'static + Send,
    V: IntoView + 'static,
{
    // Create a signal to store the window width.
    // We initialize it to 0. It will be updated by the effect on the client.
    let (width, set_width) = signal(0_u32);

    // 3. Replaced deprecated `create_effect` with `Effect::new`
    Effect::new(move |_| {
        
        // 4. Wrap all browser-specific code in a cfg block
        // This prevents the SSR build from trying to type-check
        // `Closure` and `window`, which are not `Send + Sync`.
        #[cfg(target_arch = "wasm32")]
        {
            // Get the window object (panics if not in a browser, but
            // this code only runs in wasm32)
            let window = leptos::leptos_dom::helpers::window();

            // Clone the window for use in the closure
            let window_clone = window.clone();

            // Define the resize handler closure
            let on_resize = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                // Get the new width and update the signal using the cloned window
                let w = window_clone
                    .inner_width()
                    .expect("Failed to get inner width")
                    .as_f64()
                    .expect("Inner width is not a number") as u32;
                set_width.set(w); // Use .set() - requires the `Set` trait
            }) as Box<dyn FnMut(_)>);

            // Set the initial width when the component mounts using the original window
            let w = window
                .inner_width()
                .expect("Failed to get inner width")
                .as_f64()
                .expect("Inner width is not a number") as u32;
            set_width.set(w); // Use .set() - requires the `Set` trait

            // Add the event listener for "resize"
            window
                .add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())
                .expect("Failed to add resize event listener");

            // 5. Use `StoredValue::new_local` for the !Send closure
            let stored_on_resize = StoredValue::new_local(on_resize);

            // 6. `on_cleanup` is now found and captures the (local) StoredValue
            on_cleanup(move || {
                // This code only runs on the client
                let window = leptos::leptos_dom::helpers::window();
                
                // 7. Use `with_value()` with the WithValue trait imported
                stored_on_resize.with_value(|on_resize_closure| {
                    // ...and use it to remove the listener.
                    window
                        .remove_event_listener_with_callback("resize", on_resize_closure.as_ref().unchecked_ref())
                        .expect("Failed to remove resize event listener");
                });
            });
        }
    });

    // This is the reactive part.
    // This closure will re-run whenever the `width` signal changes.
    let view_to_render = move || {
        let current_width = width.get(); // Use .get() - requires the `Get` trait

        // Find the first breakpoint where current_width < max_width
        let matching_view = breakpoints
            .iter()
            .find(|(max_width, _)| current_width < *max_width);

        if let Some((_, view_fn)) = matching_view {
            // Render the view from the matching breakpoint
            view_fn().into_view()
        } else {
            // Render the fallback view
            fallback().into_view()
        }
    };

    // Return the reactive view
    view! {
        {view_to_render}
    }
}