use leptos::prelude::*;
use leptos::html::Canvas;
#[cfg(any(feature = "hydrate", target_arch = "wasm32"))]
use wasm_bindgen::prelude::*;
#[cfg(any(feature = "hydrate", target_arch = "wasm32"))]
use wasm_bindgen::JsCast;
#[cfg(any(feature = "hydrate", target_arch = "wasm32"))]
use web_sys::{WebGlRenderingContext as GL, WebGlProgram, WebGlShader, WebGlTexture};

#[component]
pub fn WebGLSlideshow(
    #[prop(into)] images: Vec<String>, 
    #[prop(default = 5000)] interval_ms: u64,
    #[prop(default = 3000)] transition_ms: u64,
) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();

    let _ = &images;
    let _ = &interval_ms;
    let _ = &transition_ms;

    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        use std::rc::Rc;
        use std::cell::RefCell;
        use std::ops::Deref;

        if images.is_empty() { return; }

        let canvas = canvas_ref.get().expect("Canvas not found");
        let canvas_sys: web_sys::HtmlCanvasElement = canvas.deref().clone().unchecked_into();
        
        let gl = canvas_sys
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<GL>()
            .unwrap();

        // --- WebGL Setup ---
        let vert_code = "
            attribute vec2 position;
            varying vec2 v_texCoord;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                v_texCoord = (position + 1.0) * 0.5; 
                v_texCoord.y = 1.0 - v_texCoord.y; 
            }
        ";
        let frag_code = "
            precision mediump float;
            varying vec2 v_texCoord;
            uniform sampler2D u_image0;
            uniform sampler2D u_image1;
            uniform float u_mix;
            
            void main() {
                vec4 color0 = texture2D(u_image0, v_texCoord);
                vec4 color1 = texture2D(u_image1, v_texCoord);
                gl_FragColor = mix(color0, color1, u_mix);
            }
        ";

        let program = link_program(&gl, vert_code, frag_code).unwrap();
        gl.use_program(Some(&program));

        let vertices: [f32; 12] = [
            -1.0, -1.0,  1.0, -1.0, -1.0,  1.0, 
            -1.0,  1.0,  1.0, -1.0,  1.0,  1.0,
        ];
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        let position_loc = gl.get_attrib_location(&program, "position");
        gl.enable_vertex_attrib_array(position_loc as u32);
        gl.vertex_attrib_pointer_with_i32(position_loc as u32, 2, GL::FLOAT, false, 0, 0);

        let texture0 = create_texture(&gl);
        let texture1 = create_texture(&gl);
        
        let image_elements: Vec<web_sys::HtmlImageElement> = images.iter().map(|src| {
            let img = web_sys::HtmlImageElement::new().unwrap();
            img.set_cross_origin(Some("anonymous"));
            img.set_src(src);
            img
        }).collect();

        // Initialize dimensions
        let initial_width = canvas_sys.client_width() as u32;
        let initial_height = canvas_sys.client_height() as u32;
        canvas_sys.set_width(initial_width);
        canvas_sys.set_height(initial_height);
        gl.viewport(0, 0, initial_width as i32, initial_height as i32);

        let state = Rc::new(RefCell::new(AnimationState {
            current_img_idx: 0,
            next_img_idx: 1,
            mix_ratio: 0.0,
            is_transitioning: false,
            last_switch_time: 0.0,
            start_transition_time: 0.0,
            uploaded_idx_0: None,
            uploaded_idx_1: None,
            cached_width: initial_width,
            cached_height: initial_height,
            frame_count: 0,
        }));

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        
        let gl_clone = gl.clone();
        let canvas_loop = canvas_sys.clone(); 
        
        let program_clone = program.clone();
        let images_clone = image_elements.clone();
        let t0_clone = texture0.clone();
        let t1_clone = texture1.clone();
        let state_clone = state.clone();

        let window = web_sys::window().unwrap();
        let performance = window.performance().expect("should have performance");

        *g.borrow_mut() = Some(Closure::new(move || {
            let now = performance.now();
            let mut s = state_clone.borrow_mut();
            s.frame_count += 1;

            // --- Logic Update ---
            if !s.is_transitioning {
                if now - s.last_switch_time > interval_ms as f64 {
                    s.is_transitioning = true;
                    s.start_transition_time = now;
                }
            } else {
                let progress = (now - s.start_transition_time) / transition_ms as f64;
                if progress >= 1.0 {
                    s.mix_ratio = 0.0;
                    s.is_transitioning = false;
                    s.current_img_idx = s.next_img_idx;
                    s.next_img_idx = (s.current_img_idx + 1) % images_clone.len();
                    s.last_switch_time = now;
                } else {
                    s.mix_ratio = progress as f32;
                }
            }

            if s.frame_count % 20 == 0 {
                 let current_w = canvas_loop.client_width() as u32;
                 let current_h = canvas_loop.client_height() as u32;
                 
                 if current_w != s.cached_width || current_h != s.cached_height {
                     s.cached_width = current_w;
                     s.cached_height = current_h;
                     
                     canvas_loop.set_width(current_w);
                     canvas_loop.set_height(current_h);
                     gl_clone.viewport(0, 0, current_w as i32, current_h as i32);
                 }
            }

            // --- Texture Updates ---
            let img_curr = &images_clone[s.current_img_idx];
            let img_next = &images_clone[s.next_img_idx];

            if img_curr.complete() && s.uploaded_idx_0 != Some(s.current_img_idx) {
                update_texture(&gl_clone, &t0_clone, img_curr);
                s.uploaded_idx_0 = Some(s.current_img_idx);
            }
            
            if img_next.complete() && s.uploaded_idx_1 != Some(s.next_img_idx) {
                update_texture(&gl_clone, &t1_clone, img_next);
                s.uploaded_idx_1 = Some(s.next_img_idx);
            }

            // --- Draw ---
            gl_clone.active_texture(GL::TEXTURE0);
            gl_clone.bind_texture(GL::TEXTURE_2D, Some(&t0_clone));
            let loc0 = gl_clone.get_uniform_location(&program_clone, "u_image0");
            gl_clone.uniform1i(loc0.as_ref(), 0);

            gl_clone.active_texture(GL::TEXTURE1);
            gl_clone.bind_texture(GL::TEXTURE_2D, Some(&t1_clone));
            let loc1 = gl_clone.get_uniform_location(&program_clone, "u_image1");
            gl_clone.uniform1i(loc1.as_ref(), 1);

            let loc_mix = gl_clone.get_uniform_location(&program_clone, "u_mix");
            gl_clone.uniform1f(loc_mix.as_ref(), s.mix_ratio);

            gl_clone.draw_arrays(GL::TRIANGLES, 0, 6);

            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    });

    view! {
        <canvas
            node_ref=canvas_ref
            style="width: 100%; height: 100%; display: block;"
        />
    }
}

// Helpers guarded by cfg to prevent dead code warnings on server
#[cfg(any(feature = "hydrate", target_arch = "wasm32"))]
struct AnimationState {
    current_img_idx: usize,
    next_img_idx: usize,
    mix_ratio: f32,
    is_transitioning: bool,
    last_switch_time: f64,
    start_transition_time: f64,
    uploaded_idx_0: Option<usize>,
    uploaded_idx_1: Option<usize>,
    cached_width: u32,
    cached_height: u32,
    frame_count: u64,
}

#[cfg(any(feature = "hydrate", target_arch = "wasm32"))]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[cfg(any(feature = "hydrate", target_arch = "wasm32"))]
fn create_texture(gl: &GL) -> WebGlTexture {
    let tex = gl.create_texture().unwrap();
    gl.bind_texture(GL::TEXTURE_2D, Some(&tex));
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
    tex
}

#[cfg(any(feature = "hydrate", target_arch = "wasm32"))]
fn update_texture(gl: &GL, texture: &WebGlTexture, img: &web_sys::HtmlImageElement) {
    gl.bind_texture(GL::TEXTURE_2D, Some(texture));
    let _ = gl.tex_image_2d_with_u32_and_u32_and_image(
        GL::TEXTURE_2D, 0, GL::RGBA as i32, GL::RGBA, GL::UNSIGNED_BYTE, img
    );
}

#[cfg(any(feature = "hydrate", target_arch = "wasm32"))]
fn link_program(gl: &GL, vert_source: &str, frag_source: &str) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("Unable to create shader object")?;
    let vert_shader = compile_shader(gl, GL::VERTEX_SHADER, vert_source)?;
    let frag_shader = compile_shader(gl, GL::FRAGMENT_SHADER, frag_source)?;
    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);
    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_else(|| "Unknown link error".into()))
    }
}

#[cfg(any(feature = "hydrate", target_arch = "wasm32"))]
fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or("Unable to create shader object")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or_else(|| "Unknown compile error".into()))
    }
}