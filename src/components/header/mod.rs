use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::stacks::hstack::{HStack, HStackItem, AlignItems as HAlign, JustifyContent as HJustify};
use crate::components::stacks::vstack::{VStack, AlignItems as VAlign};

/// The main Header component for the application.
#[component]
pub fn Header() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    // Design Tokens
    let primary_bg = "#332175";
    let text_color = "white";
    let accent_color = "#f2e300";
    
    // Dynamic styles based on state
    let nav_text_color = move || if is_open.get() { accent_color } else { text_color };
    let caret_rotation = move || if is_open.get() { "180" } else { "0" };

    view! {
        <header
            on:mouseleave=move |_| set_is_open.set(false)
            style=format!("
                position: relative;
                width: 100%;
                margin: 0;
                z-index: 1000;
                background-color: {primary_bg};
                color: {text_color};
                box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
                font-family: 'Zalando Sans Expanded', sans-serif;
            ")
        >
            <HStack
                align=HAlign::Center
                justify=HJustify::SpaceBetween
                style="padding: 0 2rem; height: 60px;"
            >
                // --- LEFT: Logo ---
                <HStackItem style="flex: 1">
                    <A href="/" attr:style="display: flex; align-items: center; text-decoration: none;">
                        <img 
                            src="/assets/logo.svg" 
                            alt="Logotipo da Associação Camará Capoeira" 
                            style="height: 36px; width: auto; object-fit: contain;"
                        />
                    </A>
                </HStackItem>

                // --- CENTER: Navigation Item ---
                <HStackItem>
                    <div
                        on:mouseenter=move |_| set_is_open.set(true)
                        style=move || format!("
                            display: flex;
                            align-items: center;
                            gap: 0.4rem;
                            cursor: pointer;
                            padding: 0.5rem 1rem;
                            transition: color 0.2s ease;
                            color: {};
                        ", nav_text_color())
                    >
                        <A
                            href="/projetos"
                            attr:style="
                                text-decoration: none;
                                font-weight: 700;
                                font-size: 0.9rem;
                                color: inherit; 
                                letter-spacing: 0.05em;
                                text-transform: uppercase;
                            "
                        >
                            "PROJETOS"
                        </A>

                        // Caret Button
                        <button
                            on:click=move |e| {
                                e.prevent_default();
                                e.stop_propagation();
                                set_is_open.update(|open| *open = !*open);
                            }
                            style="
                                background: none;
                                border: none;
                                cursor: pointer;
                                padding: 0;
                                display: flex;
                                align-items: center;
                                color: inherit;
                            "
                            aria-label="Toggle Dropdown"
                        >
                            <svg
                                width="16"
                                height="16"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                style=move || format!(
                                    "transition: transform 0.3s ease; transform: rotate({}deg);",
                                    caret_rotation()
                                )
                            >
                                <polyline points="6 9 12 15 18 9"></polyline>
                            </svg>
                        </button>
                    </div>
                </HStackItem>

                // --- RIGHT: Spacer (Balances the logo on the left to keep nav centered) ---
                <HStackItem style="flex: 1">
                    <div />
                </HStackItem>
            </HStack>

            // --- FULL WIDTH DROPDOWN ---
            <div
                style=move || format!("
                    display: {};
                    position: absolute;
                    top: 100%;
                    left: 0;
                    width: 100%;
                    background-color: {primary_bg};
                    border-top: 1px solid rgba(255,255,255,0.1);
                    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
                    z-index: 999;
                    padding: 1.5rem 0;
                ", if is_open.get() { "block" } else { "none" })
            >
                <VStack
                    align=VAlign::Center
                    style="max-width: 1200px; margin: 0 auto; padding: 0 2rem;"
                    spacing="1rem".to_string()
                >
                    <A
                        href="/projetos/ponto-de-cultura"
                        attr:style=format!("
                            display: block;
                            color: white;
                            text-decoration: none;
                            font-weight: 600;
                            font-size: 1.1rem;
                            transition: color 0.2s ease;
                        ")
                    >
                        // Inline style for hover effect simulation in pure styling
                        // Note: Real hover states usually require CSS classes or state tracking.
                        // Since this is inline, it stays white, but fits the palette.
                        "PONTO DE CULTURA"
                    </A>
                </VStack>
            </div>
        </header>
    }
}