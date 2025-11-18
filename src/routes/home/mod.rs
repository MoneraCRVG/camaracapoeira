use leptos::prelude::*;
use leptos::{html, view};
use crate::components::header::Header;
use crate::components::stacking_container::StackingContainer;
use crate::components::stacks::vstack::{VStack, AlignItems as VAlign};
use crate::components::stacks::hstack::{HStack, AlignItems as HAlign, JustifyContent as HJustify, FlexWrap};
use std::time::Duration;

#[component]
pub fn HomePage() -> impl IntoView {
    // Reference needed for StackingContainer
    let container_ref = NodeRef::new();

    // Design Tokens
    let primary_bg = "#332175";
    let accent_color = "#f2e300";
    let white = "#ffffff";
    let gray_light = "#f4f4f9";

    // --- Slideshow Logic ---
    // We create a resource to fetch images from the server function
    let images_resource = Resource::new(
        || (), 
        |_| get_random_images()
    );

    let (active_index, set_active_index) = signal(0);

    // Set up the slideshow interval
    // REASONING: Mantivemos 5 segundos. Com 3s de transição, teremos:
    // 3s trocando + 2s estático. Isso cria um fluxo dinâmico constante.
    #[cfg(feature = "hydrate")]
    {
        use leptos::leptos_dom::helpers::set_interval_with_handle;
        
        Effect::new(move |_| {
            let handle = set_interval_with_handle(move || {
                set_active_index.update(|i| *i = *i + 1);
            }, Duration::from_secs(5)).ok(); // Ciclo de 5 segundos
            
            on_cleanup(move || {
                if let Some(h) = handle {
                    h.clear();
                }
            });
        });
    }

    view! {
        <StackingContainer node_ref=container_ref>
            // 1. Fixed Header
            // Note: Since this is sticky with a high z-index, it will stay on top of the stacking sections below.
            <div style="position: sticky; top: 0; z-index: 1000;">
                <Header />
            </div>

            // 2. Hero Section (Destaque Principal) with Slideshow Background
            // Added 'position: sticky; top: 0;' to enable the stacking effect.
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
            ", primary_bg, white)>
                
                // A. Slideshow Layer (Background)
                <div style="
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    z-index: -2;
                    pointer-events: none;
                ">
                    <Suspense fallback=move || view! { <div style="width:100%; height:100%; background-color: #332175;"></div> }>
                        {move || {
                            images_resource.get().map(|data| {
                                match data {
                                    Ok(images) => {
                                        if images.is_empty() {
                                            return view! { <div style="display:none;">"No images found"</div> }.into_any();
                                        }

                                        let len = images.len();
                                        let safe_index = active_index.get() % len;

                                        images.into_iter().enumerate().map(move |(index, src)| {
                                            let is_active = safe_index == index;
                                            
                                            // REASONING:
                                            // Alterado transition para 'opacity 3s ease-in-out'.
                                            // Isso garante que o fade in e o fade out ocorram suavemente
                                            // ao mesmo tempo, criando o efeito de 'mistura'.
                                            view! {
                                                <div style=format!("
                                                    position: absolute;
                                                    top: 0;
                                                    left: 0;
                                                    width: 100%;
                                                    height: 100%;
                                                    opacity: {}; 
                                                    transition: opacity 3s ease-in-out;
                                                    z-index: {};
                                                    will-change: opacity; /* Otimização de performance */
                                                ", if is_active { 1.0 } else { 0.0 }, if is_active { 1 } else { 0 })>
                                                    <img 
                                                        src=src
                                                        alt="Slideshow Background"
                                                        width=1920
                                                        height=1080
                                                        style="width: 100%; height: 100%; object-fit: cover; position: absolute; top: 0; left: 0;".to_string()
                                                    />
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    },
                                    Err(_) => view! { <div style="display:none;">"Error loading images"</div> }.into_any()
                                }
                            })
                        }}
                    </Suspense>
                </div>

                // B. Dark Translucid Overlay Layer
                // Mantido para garantir leitura do texto sobre as imagens misturadas
                <div style="
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: rgba(20, 10, 40, 0.75);
                    z-index: -1;
                    backdrop-filter: blur(1px);
                "></div>

                // C. Content Layer (Foreground)
                <div style="position: relative; z-index: 1;">
                    <VStack spacing="1.5rem".to_string() align=VAlign::Center>
                        <h1 style="font-size: 3rem; font-weight: 800; line-height: 1.1; max-width: 800px; font-family: 'Zalando Sans Expanded', sans-serif; text-shadow: 0 2px 10px rgba(0,0,0,0.5);">
                            "CAMARÁ CAPOEIRA"
                        </h1>
                        <div style=format!("width: 80px; height: 6px; background-color: {}; margin: 0.5rem 0; box-shadow: 0 2px 4px rgba(0,0,0,0.3);", accent_color)></div>
                        <h2 style="font-size: 1.5rem; font-weight: 400; opacity: 0.95; text-shadow: 0 2px 4px rgba(0,0,0,0.5);">
                            "Cultura, Educação e Cidadania em Ponta Porã - MS"
                        </h2>
                        <p style="font-size: 1.1rem; max-width: 600px; margin-top: 1rem; line-height: 1.6; text-shadow: 0 1px 3px rgba(0,0,0,0.8);">
                            "Promovendo inclusão social e preservando a cultura afro-brasileira na fronteira através da arte da Capoeira."
                        </p>
                    </VStack>
                </div>
            </div>

            // 3. Section Sobre / Ponto de Cultura
            // Added sticky positioning
            <div style=format!("position: sticky; top: 0; z-index: 2; background-color: {}; padding: 4rem 2rem;", white)>
                <HStack 
                    wrap=FlexWrap::Wrap 
                    justify=HJustify::Center 
                    spacing="4rem".to_string()
                    align=HAlign::Center
                >
                    // Texto
                    <div style="flex: 1; min-width: 300px; max-width: 600px;">
                        <VStack spacing="1.5rem".to_string() align=VAlign::FlexStart>
                            <h3 style=format!("color: {}; font-size: 2rem; font-weight: 700;", primary_bg)>
                                "Ponto de Cultura"
                            </h3>
                            <p style="font-size: 1.1rem; color: #444; line-height: 1.6;">
                                "Reconhecida oficialmente como um Ponto de Cultura, a organização Camará Capoeira atua como um farol de transformação social em Mato Grosso do Sul. Nosso trabalho vai além da roda: combatemos o trabalho infantil, promovemos a educação patrimonial e fortalecemos a identidade cultural de nossa comunidade."
                            </p>
                            <div style=format!("padding: 1rem; border-left: 4px solid {}; background: #f9f9f9;", accent_color)>
                                <p style="font-style: italic; color: #555;">
                                    "Mais do que um esporte, uma ferramenta de inclusão social."
                                </p>
                            </div>
                        </VStack>
                    </div>

                    // Box Informativo Visual
                    <div style="flex: 1; min-width: 300px; max-width: 500px;">
                        <VStack 
                            spacing="1rem".to_string() 
                            style=format!("background-color: {}; padding: 2rem; border-radius: 12px; box-shadow: 0 10px 25px -5px rgba(0,0,0,0.1);", gray_light)
                        >
                            <h4 style="font-weight: bold; color: #333;">"Nossos Pilares"</h4>
                            <ul style="list-style: none; padding: 0; display: flex; flex-direction: column; gap: 0.8rem;">
                                <li style="display: flex; align-items: center; gap: 10px;">
                                    <span style=format!("color: {}; font-weight: bold;", primary_bg)>"✓"</span> "Projeto Cultura Viva"
                                </li>
                                <li style="display: flex; align-items: center; gap: 10px;">
                                    <span style=format!("color: {}; font-weight: bold;", primary_bg)>"✓"</span> "Combate ao Trabalho Infantil"
                                </li>
                                <li style="display: flex; align-items: center; gap: 10px;">
                                    <span style=format!("color: {}; font-weight: bold;", primary_bg)>"✓"</span> "Inclusão Social"
                                </li>
                            </ul>
                        </VStack>
                    </div>
                </HStack>
            </div>

            // 4. Na Mídia (Media Section)
            // Added sticky positioning
            <div style=format!("position: sticky; top: 0; z-index: 3; background-color: {}; padding: 4rem 2rem;", gray_light)>
                <VStack spacing="3rem".to_string() align=VAlign::Center>
                    <h3 style="font-size: 2rem; font-weight: 700; text-align: center; color: #333;">
                        "Reconhecimento na Mídia"
                    </h3>

                    <HStack wrap=FlexWrap::Wrap justify=HJustify::Center spacing="2rem".to_string()>
                        
                        // Card 1
                        <a href="https://www.fundacaodecultura.ms.gov.br/projeto-ponto-de-cultura-camara-promove-oficinas-e-festival-em-ponta-pora-ms/" target="_blank" style="text-decoration: none; color: inherit;">
                            <div style="background: white; padding: 2rem; border-radius: 8px; width: 300px; height: 100%; box-shadow: 0 4px 6px rgba(0,0,0,0.05); transition: transform 0.2s;">
                                <h5 style=format!("color: {}; font-weight: bold; margin-bottom: 0.5rem;", primary_bg)>"Fundação de Cultura MS"</h5>
                                <p style="font-size: 0.9rem; color: #666;">"Projeto Ponto de Cultura Camará promove oficinas e festival em Ponta Porã."</p>
                            </div>
                        </a>

                        // Card 2
                        <a href="https://www.campograndenews.com.br/lado-b/diversao/projeto-leva-capoeira-musica-e-danca-gratuitamente-a-comunidade" target="_blank" style="text-decoration: none; color: inherit;">
                            <div style="background: white; padding: 2rem; border-radius: 8px; width: 300px; height: 100%; box-shadow: 0 4px 6px rgba(0,0,0,0.05); transition: transform 0.2s;">
                                <h5 style=format!("color: {}; font-weight: bold; margin-bottom: 0.5rem;", primary_bg)>"Campo Grande News"</h5>
                                <p style="font-size: 0.9rem; color: #666;">"Projeto leva capoeira, música e dança gratuitamente à comunidade."</p>
                            </div>
                        </a>

                        // Card 3
                        <a href="https://www.pontaporainforma.com.br/grupo-camara-de-ponta-pora-e-destaque-em-encontro-de-capoeira-na-capital/" target="_blank" style="text-decoration: none; color: inherit;">
                            <div style="background: white; padding: 2rem; border-radius: 8px; width: 300px; height: 100%; box-shadow: 0 4px 6px rgba(0,0,0,0.05); transition: transform 0.2s;">
                                <h5 style=format!("color: {}; font-weight: bold; margin-bottom: 0.5rem;", primary_bg)>"Ponta Porã Informa"</h5>
                                <p style="font-size: 0.9rem; color: #666;">"Grupo Camará de Ponta Porã é destaque em encontro de capoeira na capital."</p>
                            </div>
                        </a>

                            // Card 4
                        <a href="https://www.fadc.org.br/noticias/organizacoes-trabalho-infantil" target="_blank" style="text-decoration: none; color: inherit;">
                            <div style="background: white; padding: 2rem; border-radius: 8px; width: 300px; height: 100%; box-shadow: 0 4px 6px rgba(0,0,0,0.05); transition: transform 0.2s;">
                                <h5 style=format!("color: {}; font-weight: bold; margin-bottom: 0.5rem;", primary_bg)>"FADC"</h5>
                                <p style="font-size: 0.9rem; color: #666;">"Luta contra o trabalho infantil e promoção de direitos."</p>
                            </div>
                        </a>

                    </HStack>
                </VStack>
            </div>

            // 5. Realização e Apoio (Partners)
            // Added sticky positioning
            <div style=format!("position: sticky; top: 0; z-index: 4; background-color: {}; color: white; padding: 4rem 2rem; border-top: 1px solid rgba(255,255,255,0.1);", primary_bg)>
                <VStack spacing="2rem".to_string() align=VAlign::Center>
                    <h4 style="text-transform: uppercase; letter-spacing: 0.1em; font-size: 0.9rem; opacity: 0.8;">
                        "Realização e Apoio Institucional"
                    </h4>
                    
                    <HStack 
                        wrap=FlexWrap::Wrap 
                        justify=HJustify::Center 
                        align=HAlign::Center
                        spacing="1.5rem".to_string()
                    >
                        <Badge text="Projeto Cultura Viva" />
                        <Badge text="Ponto de Cultura" />
                        <Badge text="Fundação de Cultura de Mato Grosso do Sul" />
                        <Badge text="SETESC Gov. MS" />
                        <Badge text="Sistema Nacional de Cultura" />
                        <Badge text="Lei Aldir Blanc" />
                        <Badge text="Ministério da Cultura - Governo Federal" />
                    </HStack>
                </VStack>
            </div>

            // Footer
            // Added sticky positioning
            <div style="position: sticky; top: 0; z-index: 5; background-color: #1a113d; color: rgba(255,255,255,0.4); padding: 2rem; text-align: center; font-size: 0.8rem;">
                <p>"© 2024 Associação de Capoeira Camará. Todos os direitos reservados."</p>
                <p>"Ponta Porã - Mato Grosso do Sul - Brasil"</p>
            </div>

        </StackingContainer>
    }
}

// Badge Helper Component
#[component]
fn Badge(text: &'static str) -> impl IntoView {
    view! {
        <div style="
            border: 1px solid rgba(255,255,255,0.3);
            padding: 0.5rem 1rem;
            border-radius: 20px;
            font-size: 0.85rem;
            background: rgba(255,255,255,0.05);
            text-align: center;
        ">
            {text}
        </div>
    }
}

// --- Server Function to choose random images ---
#[server(GetRandomImages, "/api/get_random_images")]
pub async fn get_random_images() -> Result<Vec<String>, ServerFnError> {
    use std::fs;
    use rand::seq::SliceRandom;
    use rand::thread_rng;


    // Path relative to the project root where cargo run / server runs
    let target_dir = "assets/acervo/inicio";
    let mut images = Vec::new();

    // Read directory
    match fs::read_dir(target_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            let ext_str = ext.to_string_lossy().to_lowercase();
                            if ["jpg", "jpeg", "png", "webp"].contains(&ext_str.as_str()) {
                                if let Some(name) = path.file_name() {
                                    // Create the public URL path
                                    images.push(format!("/{}/{}", target_dir, name.to_string_lossy()));
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            // Return error or empty vec depending on preference. 
            // Logging error to server console for debugging
            eprintln!("Error reading image directory: {}", e);
            return Ok(vec![]); // Return empty vec to prevent crash, handled by UI
        }
    }

    // Shuffle and pick 10
    let mut rng = thread_rng();
    images.shuffle(&mut rng);
    images.truncate(10);

    Ok(images)
}