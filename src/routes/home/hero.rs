use leptos::prelude::*;
use crate::components::stacks::vstack::{VStack, AlignItems as VAlign};
use super::webgl_slideshow::WebGLSlideshow;

#[component]
pub fn HeroSection(
    // Correção: Tipo Resource agora é genericamente Resource<Data> nesta versão do Leptos
    images_resource: Resource<Result<Vec<String>, ServerFnError>>
) -> impl IntoView {
    let primary_bg = "#332175";
    let white = "#ffffff";
    let accent_color = "#f2e300";

    view! {
        <div style=format!("
            position: sticky;
            top: 0;
            background-color: {}; 
            color: {}; 
            padding: 6rem 2rem; 
            text-align: center; 
            overflow: hidden;
            isolation: isolate;
            z-index: 1; 
            height: 80vh;
            min-height: 600px;
        ", primary_bg, white)>
            
            <div style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; z-index: -2;">
                <Suspense fallback=move || view! { <div style="background:#332175; width:100%; height:100%;"></div> }>
                {move || {
                    // O Resource::get() retorna Option<Result<...>>
                    // Precisamos mapear para exibir a view corretamente
                    match images_resource.get() {
                        Some(Ok(images)) if !images.is_empty() => {
                            view! { <WebGLSlideshow images=images /> }.into_any()
                        },
                        _ => view! { <div style="background:#332175; width:100%; height:100%;"></div> }.into_any()
                    }
                }}
                </Suspense>
            </div>

            <div style="
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                background: rgba(20, 10, 40, 0.6);
                z-index: -1;
                backdrop-filter: blur(2px);
            "></div>

            <div style="position: relative; z-index: 1; height: 100%; display: flex; align-items: center; justify-content: center;">
                <VStack spacing="1.5rem".to_string() align=VAlign::Center>
                    <h1 style="font-size: 3.5rem; font-weight: 800; line-height: 1.1; max-width: 900px; font-family: 'Zalando Sans Expanded', sans-serif; text-shadow: 0 4px 20px rgba(0,0,0,0.6);">
                        "CAMARÁ CAPOEIRA"
                    </h1>
                    <div style=format!("width: 100px; height: 8px; background-color: {}; margin: 0.5rem 0; box-shadow: 0 2px 4px rgba(0,0,0,0.3);", accent_color)></div>
                    <h2 style="font-size: 1.8rem; font-weight: 400; opacity: 0.95; text-shadow: 0 2px 4px rgba(0,0,0,0.5);">
                        "Cultura, Educação e Cidadania em Ponta Porã - MS"
                    </h2>
                    <p style="font-size: 1.2rem; max-width: 700px; margin-top: 1rem; line-height: 1.6; text-shadow: 0 1px 3px rgba(0,0,0,0.8);">
                        "Promovendo inclusão social e preservando a cultura afro-brasileira na fronteira através da arte da Capoeira."
                    </p>
                    <a 
                        href="#sobre"
                        style=format!("
                            margin-top: 2rem;
                            padding: 1rem 2rem;
                            background-color: {};
                            color: {};
                            font-weight: bold;
                            text-decoration: none;
                            border-radius: 50px;
                            transition: transform 0.2s;
                            display: inline-block;
                        ", accent_color, primary_bg)
                    >
                        "Conheça Nossos Projetos"
                    </a>
                </VStack>
            </div>
        </div>
    }
}