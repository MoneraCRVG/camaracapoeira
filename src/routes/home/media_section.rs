use leptos::prelude::*;
use wasm_bindgen::JsCast; // Importação necessária para unchecked_into
use crate::components::stacks::vstack::{VStack, AlignItems as VAlign};
use crate::components::stacks::hstack::{HStack, JustifyContent as HJustify, FlexWrap};

#[component]
pub fn MediaSection() -> impl IntoView {
    let primary_bg = "#332175";
    let gray_light = "#f4f4f9";

    view! {
        <div style=format!("position: sticky; top: 0; z-index: 3; background-color: {}; padding: 5rem 2rem;", gray_light)>
            <VStack spacing="3rem".to_string() align=VAlign::Center>
                <h3 style="font-size: 2.5rem; font-weight: 700; text-align: center; color: #333;">
                    "Reconhecimento na Mídia"
                </h3>

                <HStack wrap=FlexWrap::Wrap justify=HJustify::Center spacing="2rem".to_string()>
                    
                    <MediaCard 
                        title="Fundação de Cultura MS"
                        snippet="O projeto promove oficinas de capoeira, percussão e maculelê, além de apresentações culturais em espaços públicos, aproximando diferentes linguagens artísticas da população."
                        link="https://www.fundacaodecultura.ms.gov.br/projeto-ponto-de-cultura-camara-promove-oficinas-e-festival-em-ponta-pora-ms/"
                        color=primary_bg
                    />

                    <MediaCard 
                        title="Campo Grande News"
                        snippet="Em Ponta Porã, o Ponto de Cultura Camará tem movimentado o bairro Parque dos Ipês II com oficinas gratuitas de capoeira, percussão e maculelê, envolvendo diferentes gerações."
                        link="https://www.campograndenews.com.br/lado-b/diversao/projeto-leva-capoeira-musica-e-danca-gratuitamente-a-comunidade"
                        color=primary_bg
                    />

                    <MediaCard 
                        title="Ponta Porã Informa"
                        snippet="O grupo tem se destacado em encontros estaduais, fortalecendo a prática da capoeira e levando o nome de Ponta Porã para eventos de grande relevância cultural na capital."
                        link="https://www.pontaporainforma.com.br/grupo-camara-de-ponta-pora-e-destaque-em-encontro-de-capoeira-na-capital/"
                        color=primary_bg
                    />

                    <MediaCard 
                        title="FADC"
                        snippet="Atuação no combate ao trabalho infantil e promoção de direitos através do 3º Seminário Pelo Direito de Ser e Viver como Criança, estruturando planos de ação municipais."
                        link="https://www.fadc.org.br/noticias/organizacoes-trabalho-infantil"
                        color=primary_bg
                    />

                </HStack>
            </VStack>
        </div>
    }
}

#[component]
fn MediaCard(title: &'static str, snippet: &'static str, link: &'static str, color: &'static str) -> impl IntoView {
    view! {
        <a href=link target="_blank" style="text-decoration: none; color: inherit; flex: 1; min-width: 280px; max-width: 320px;">
            <div style="
                background: white; 
                padding: 2rem; 
                border-radius: 12px; 
                height: 100%; 
                box-shadow: 0 4px 15px rgba(0,0,0,0.05); 
                transition: all 0.2s ease;
                display: flex;
                flex-direction: column;
                gap: 1rem;
                border: 1px solid transparent;
            "
            on:mouseenter=move |e| {
                // unchecked_into agora funciona com o use wasm_bindgen::JsCast
                let _ = e.target().unwrap().unchecked_into::<web_sys::HtmlElement>().style().set_property("transform", "translateY(-5px)");
                let _ = e.target().unwrap().unchecked_into::<web_sys::HtmlElement>().style().set_property("box-shadow", "0 10px 25px rgba(0,0,0,0.1)");
            }
            on:mouseleave=move |e| {
                let _ = e.target().unwrap().unchecked_into::<web_sys::HtmlElement>().style().set_property("transform", "translateY(0)");
                let _ = e.target().unwrap().unchecked_into::<web_sys::HtmlElement>().style().set_property("box-shadow", "0 4px 15px rgba(0,0,0,0.05)");
            }
            >
                <h5 style=format!("color: {}; font-weight: 800; font-size: 1.1rem;", color)>{title}</h5>
                <p style="font-size: 0.9rem; color: #555; line-height: 1.5; flex: 1;">
                    {snippet}
                </p>
                <span style=format!("color: {}; font-size: 0.8rem; font-weight: 700; text-transform: uppercase; margin-top: auto;", color)>
                    "Ler Matéria →"
                </span>
            </div>
        </a>
    }
}