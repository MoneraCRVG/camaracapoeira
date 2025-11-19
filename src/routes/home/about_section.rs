use leptos::prelude::*;
use crate::components::stacks::vstack::{VStack, AlignItems as VAlign};
use crate::components::stacks::hstack::{HStack, AlignItems as HAlign, JustifyContent as HJustify, FlexWrap};

#[component]
pub fn AboutSection() -> impl IntoView {
    let primary_bg = "#332175";
    let accent_color = "#f2e300";
    let gray_light = "#f4f4f9";
    let white = "#ffffff";

    view! {
        <div id="sobre" style=format!("position: sticky; top: 0; z-index: 2; background-color: {}; padding: 5rem 2rem;", white)>
            <HStack 
                wrap=FlexWrap::Wrap 
                justify=HJustify::Center 
                spacing="4rem".to_string()
                align=HAlign::Center
            >
                // Texto
                <div style="flex: 1; min-width: 300px; max-width: 600px;">
                    <VStack spacing="1.5rem".to_string() align=VAlign::FlexStart>
                        <h3 style=format!("color: {}; font-size: 2.2rem; font-weight: 700;", primary_bg)>
                            "Ponto de Cultura"
                        </h3>
                        <p style="font-size: 1.1rem; color: #444; line-height: 1.6; text-align: justify;">
                            "Reconhecida oficialmente como um Ponto de Cultura, a organização Camará Capoeira atua como um farol de transformação social em Mato Grosso do Sul. Nosso trabalho vai além da roda: combatemos o trabalho infantil, promovemos a educação patrimonial e fortalecemos a identidade cultural de nossa comunidade através de ações continuadas."
                        </p>
                        <div style=format!("padding: 1.5rem; border-left: 5px solid {}; background: #f9f9f9; border-radius: 0 8px 8px 0;", accent_color)>
                            <p style="font-style: italic; color: #555; font-weight: 500;">
                                "Mais do que um esporte, uma ferramenta de inclusão social e cidadania plena."
                            </p>
                        </div>
                    </VStack>
                </div>

                // Box Informativo Visual
                <div style="flex: 1; min-width: 300px; max-width: 450px;">
                    <VStack 
                        spacing="1rem".to_string() 
                        style=format!("
                            background-color: {}; 
                            padding: 2.5rem; 
                            border-radius: 16px; 
                            box-shadow: 0 15px 35px -5px rgba(0,0,0,0.1);
                            border: 1px solid #eee;
                        ", gray_light)
                    >
                        <h4 style=format!("font-weight: 800; color: {}; font-size: 1.2rem; text-transform: uppercase;", primary_bg)>
                            "Nossos Pilares"
                        </h4>
                        <ul style="list-style: none; padding: 0; display: flex; flex-direction: column; gap: 1rem;">
                            <li style="display: flex; align-items: center; gap: 12px; font-size: 1.05rem;">
                                <div style=format!("width: 24px; height: 24px; background: {}; border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-size: 0.8rem;", primary_bg)>"✓"</div>
                                "Projeto Cultura Viva"
                            </li>
                            <li style="display: flex; align-items: center; gap: 12px; font-size: 1.05rem;">
                                <div style=format!("width: 24px; height: 24px; background: {}; border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-size: 0.8rem;", primary_bg)>"✓"</div>
                                "Combate ao Trabalho Infantil"
                            </li>
                            <li style="display: flex; align-items: center; gap: 12px; font-size: 1.05rem;">
                                <div style=format!("width: 24px; height: 24px; background: {}; border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-size: 0.8rem;", primary_bg)>"✓"</div>
                                "Inclusão Social e Educação"
                            </li>
                        </ul>
                    </VStack>
                </div>
            </HStack>
        </div>
    }
}