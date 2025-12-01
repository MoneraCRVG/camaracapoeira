use leptos::prelude::*;
use leptos_meta::{Script, Link};
use crate::components::stacks::hstack::{HStack, AlignItems as HAlign, FlexWrap};
use crate::components::stacks::vstack::{VStack, AlignItems as VAlign};

#[component]
pub fn FooterSection() -> impl IntoView {
    
    view! {
        <div style="position: sticky; top: 0; z-index: 5; background-color: #1a113d; color: #e0e0e0; padding: 0;">
            <Link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY=" crossorigin=""/>
            <Script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo=" crossorigin=""/>

            <div style="max-width: 1200px; margin: 0 auto; padding: 3rem 2rem;">
                <HStack wrap=FlexWrap::Wrap spacing="3rem".to_string() align=HAlign::FlexStart>
                    
                    <div style="flex: 1; min-width: 300px;">
                        <VStack spacing="1rem".to_string() align=VAlign::FlexStart>
                            <h5 style="color: white; font-size: 1.2rem; font-weight: 700;">"Associação Camará Capoeira"</h5>
                            <p style="font-size: 0.95rem; line-height: 1.6; color: rgba(255,255,255,0.7);">
                                "Rua São Judas Tadeu, nº 663"<br/>
                                "Parque dos Ipês II"<br/>
                                "Ponta Porã - MS, 79900-000"
                            </p>
                            <p style="font-size: 0.95rem; margin-top: 1rem;">
                                "Contato: contato@camaracapoeira.org.br"
                            </p>
                        </VStack>
                    </div>

                    <div style="flex: 1; min-width: 300px; height: 300px; border-radius: 8px; overflow: hidden; z-index: 10;">
                        <LeafletMap />
                    </div>

                    <div style="flex: 1; min-width: 200px;">
                         <h5 style="color: white; font-size: 1.2rem; font-weight: 700; margin-bottom: 1rem;">"Navegação"</h5>
                         <nav style="display: flex; flex-direction: column; gap: 0.8rem;">
                            <a href="/" style="color: rgba(255,255,255,0.7); text-decoration: none; transition: color 0.2s;">"Início"</a>
                            <a href="/projetos" style="color: rgba(255,255,255,0.7); text-decoration: none; transition: color 0.2s;">"Projetos"</a>
                            <a href="/sobre" style="color: rgba(255,255,255,0.7); text-decoration: none; transition: color 0.2s;">"Quem Somos"</a>
                            <a href="/contato" style="color: rgba(255,255,255,0.7); text-decoration: none; transition: color 0.2s;">"Fale Conosco"</a>
                         </nav>
                    </div>
                </HStack>

                <div style="margin-top: 3rem; padding-top: 1.5rem; border-top: 1px solid rgba(255,255,255,0.1); text-align: center; font-size: 0.8rem; opacity: 0.5;">
                    <p>"© 2024 Associação de Capoeira Camará. Todos os direitos reservados."</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn LeafletMap() -> impl IntoView {
    let map_id = "footer-map";
    
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        set_timeout(move || {
            let window = web_sys::window().unwrap();
            use wasm_bindgen::JsValue;
            let l = js_sys::Reflect::get(&window, &JsValue::from_str("L"));
            
            if let Ok(l_obj) = l {
                if !l_obj.is_undefined() {
                    let document = window.document().unwrap();
                    if let Some(element) = document.get_element_by_id(map_id) {
                        if element.child_element_count() == 0 {
                            let script = format!("
                                var map = L.map('{}').setView([-22.507296, -55.737256], 15);
                                L.tileLayer('https://{{s}}.tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
                                    attribution: '&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors'
                                }}).addTo(map);
                                L.marker([-22.507296, -55.737256]).addTo(map)
                                    .bindPopup('Sede Camará Capoeira')
                                    .openPopup();
                            ", map_id);
                            
                            let _ = js_sys::eval(&script);
                        }
                    }
                }
            }
        }, std::time::Duration::from_millis(500));
    });

    view! {
        <div id=map_id style="width: 100%; height: 100%; background: #ddd;"></div>
    }
}