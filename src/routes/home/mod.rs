pub mod hero;
pub mod about_section;
pub mod media_section;
pub mod footer_section;
pub mod webgl_slideshow;

use leptos::prelude::*;
use crate::components::header::Header;
use crate::components::stacking_container::StackingContainer;

use self::hero::HeroSection;
use self::about_section::AboutSection;
use self::media_section::MediaSection;
use self::footer_section::FooterSection;

#[component]
pub fn HomePage() -> impl IntoView {
    let container_ref = NodeRef::new();

    // Recurso de imagens (mantido aqui para ser passado ao Hero)
    let images_resource = Resource::new(
        || (), 
        |_| get_random_images()
    );

    view! {
        <StackingContainer node_ref=container_ref>
            // 1. Header (Fixo no topo, acima de tudo)
            <div style="position: sticky; top: 0; z-index: 1000;">
                <Header />
            </div>

            // 2. Hero Section (Com WebGL Slideshow)
            <HeroSection images_resource=images_resource />

            // 3. Sobre / Ponto de Cultura
            <AboutSection />

            // 4. Na Mídia (Snippets Reais)
            <MediaSection />

            // 5. Parceiros (Adicionado para corrigir o aviso de unused import e exibir a seção)
            // 6. Footer (Mapa + Nav)
            <FooterSection />

        </StackingContainer>
    }
}

// --- Server Function mantida igual ---
#[server(GetRandomImages, "/api/get_random_images")]
pub async fn get_random_images() -> Result<Vec<String>, ServerFnError> {
    use std::fs;
    use rand::seq::SliceRandom;
    use rand::rng;

    let target_dir = "assets/acervo/inicio";
    let mut images = Vec::new();

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
                                    images.push(format!("/{}/{}", target_dir, name.to_string_lossy()));
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading image directory: {}", e);
            return Ok(vec![]); 
        }
    }

    let mut rng = rng();
    images.shuffle(&mut rng);
    images.truncate(10);

    Ok(images)
}