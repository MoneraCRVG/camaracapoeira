#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Common setup
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_meta::MetaTags;
    use leptos_actix::{generate_route_list, LeptosRoutes, handle_server_fns};
    use camaracapoeira::app::*;
    use leptos::hydration::{AutoReload, HydrationScripts};
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let site_root = leptos_options.site_root.clone().to_string();

    println!("listening on http://{}", &addr);

    // =================================================================
    // GATEWAY MODE
    // =================================================================
    #[cfg(feature = "gateway_lb")]
    {
        use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
        use std::time::Duration;
        use awc::Client;
        use tokio::time::sleep;
        use actix_web::http::header;
        use tokio::task::spawn_local;

        log::info!("🚀 Gateway Load Balancer Mode ENABLED");

        let upstream_url = "http://127.0.0.1:3020";
        let gateway_addr = "0.0.0.0:3021";

        let is_upstream_alive = Arc::new(AtomicBool::new(true));
        let health_flag = is_upstream_alive.clone();
        
        spawn_local(async move {
            let client = Client::default();
            loop {
                let check_url = format!("{}/pkg/camaracapoeira.css", upstream_url);
                let request = client.head(&check_url).timeout(Duration::from_secs(2));
                match request.send().await {
                    Ok(resp) if resp.status().is_success() => {
                        if !health_flag.load(Ordering::Relaxed) {
                            log::info!("✅ Upstream {} is BACK ONLINE.", upstream_url);
                            health_flag.store(true, Ordering::Relaxed);
                        }
                    },
                    Ok(resp) => {
                        log::warn!("⚠️ Upstream status {}. Considering UNHEALTHY.", resp.status());
                        health_flag.store(false, Ordering::Relaxed);
                    }
                    Err(e) => {
                        if health_flag.load(Ordering::Relaxed) {
                            log::error!("🔥 Upstream DOWN ({:?}). Switching to FAILOVER.", e);
                            health_flag.store(false, Ordering::Relaxed);
                        }
                    }
                }
                let sleep_duration = if health_flag.load(Ordering::Relaxed) { 10 } else { 2 };
                sleep(Duration::from_secs(sleep_duration)).await;
            }
        });

        log::info!("🛡️  Gateway listening on http://{} -> Forwarding to {}", gateway_addr, upstream_url);

        HttpServer::new(move || {
            let leptos_options = leptos_options.clone();
            let site_root = site_root.clone();
            let health_flag = is_upstream_alive.clone();
            
            let client = Client::builder()
                .disable_redirects()
                .timeout(Duration::from_secs(5))
                .finish();

            App::new()
                // Enable compression on the Gateway so the user gets compressed content,
                // but the Gateway handles it, not the Upstream.
                .wrap(middleware::Compress::default()) 
                .default_service(web::to({
                    let site_root = site_root.clone();
                    move |req: HttpRequest, payload: web::Payload| {
                        let health_flag = health_flag.clone();
                        let client = client.clone();
                        let leptos_options = leptos_options.clone();
                        let _site_root = site_root.clone();
                        
                        async move {
                            if health_flag.load(Ordering::Relaxed) {
                                let forward_url = format!("{}{}", upstream_url, req.uri());
                                let method = req.method().clone();
                                
                                let mut forwarded_req = client.request(method, &forward_url);
                                for (key, value) in req.headers() {
                                    // CRITICAL FIX: Do not forward Accept-Encoding.
                                    // This forces the upstream to return plain text, preventing
                                    // awc from auto-decompressing while headers say "gzip".
                                    if key != header::CONNECTION && key != header::HOST && key != header::ACCEPT_ENCODING {
                                        forwarded_req = forwarded_req.insert_header((key.clone(), value.clone()));
                                    }
                                }
                                
                                if let Some(peer_addr) = req.peer_addr() {
                                    forwarded_req = forwarded_req.insert_header(("X-Forwarded-For", peer_addr.to_string()));
                                }

                                match forwarded_req.send_stream(payload).await {
                                    Ok(upstream_res) => {
                                        let mut client_resp = HttpResponse::build(upstream_res.status());
                                        for (key, value) in upstream_res.headers() {
                                            client_resp.insert_header((key.clone(), value.clone()));
                                        }
                                        Ok(client_resp.streaming(upstream_res))
                                    },
                                    Err(e) => {
                                        log::error!("❌ Proxy Failed: {}. LOCAL FALLBACK.", e);
                                        health_flag.store(false, Ordering::Relaxed);
                                        serve_local_leptos(leptos_options).await
                                    }
                                }
                            } else {
                                serve_local_leptos(leptos_options).await
                            }
                        }
                    }
                }))
                .service(Files::new("/pkg", format!("{site_root}/pkg")))
                .service(Files::new("/assets", &site_root))
                .service(favicon)
        })
        .bind(gateway_addr)?
        .workers(2)
        .run()
        .await
    }

    // =================================================================
    // STANDARD MODE
    // =================================================================
    #[cfg(not(feature = "gateway_lb"))]
    {
        let routes = generate_route_list(App);
        
        HttpServer::new(move || {
            let leptos_options = leptos_options.clone();
            let site_root = leptos_options.site_root.clone().to_string();
            let routes = routes.clone();

            App::new()
                .route("/api/{tail:.*}", handle_server_fns())
                .service(Files::new("/pkg", format!("{site_root}/pkg")))
                .service(Files::new("/assets", &site_root))
                .service(favicon)
                .leptos_routes(routes, {
                    let leptos_options = leptos_options.clone();
                    move || {
                        view! {
                            <!DOCTYPE html>
                            <html lang="pt-BR">
                                <head>
                                    <meta charset="utf-8"/>
                                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                    <AutoReload options=leptos_options.clone() />
                                    <HydrationScripts options=leptos_options.clone()/>
                                    <MetaTags/>
                                </head>
                                <body>
                                    <App/>
                                </body>
                            </html>
                        }
                    }
                })
                .app_data(web::Data::new(leptos_options.to_owned()))
                .wrap(middleware::Compress::default())
                .wrap(middleware::DefaultHeaders::new().add(("X-Frame-Options", "DENY")))
                .wrap(middleware::Logger::default())
        })
        .bind(&addr)?
        .workers(1)
        .run()
        .await
    }
}

// Fallback handler
#[cfg(feature = "gateway_lb")]
async fn serve_local_leptos(
    _options: leptos::config::LeptosOptions,
) -> actix_web::Result<actix_web::HttpResponse> {
    // Note: Full SSR rendering requires resolved imports for `leptos::ssr` which
    // can be tricky depending on feature flags.
    // For reliability of the Gateway LB feature right now, we return a simple
    // fallback page.
    // To enable full SSR, ensure `use leptos::ssr::render_to_string` resolves.
    
    let html = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <title>Camará Capoeira - Offline Mode</title>
    <style>body{font-family:sans-serif;text-align:center;padding:50px;background:#f4f4f9;color:#333;}</style>
</head>
<body>
    <h1>Site em Manutenção / Offline Mode</h1>
    <p>O servidor principal está indisponível no momento.</p>
    <p>The main server is currently unavailable. Please try again later.</p>
</body>
</html>"#;

    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    use camaracapoeira::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}