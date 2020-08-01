mod app;
mod components;
mod resources;

use app::Application;
use cgmath::prelude::*;
use cgmath::{Matrix4, Point3, Vector2, Vector3};
use components::{load_obj, Camera, Light};
use imgui::{im_str, Context, ImString};
use palantir_lib::TCamera;
use palantir_lib::{Mesh, Renderer, ShaderProgram};
use sdl2::event::{Event, WindowEvent};
use sdl2::mouse::MouseState;
use std::time::Instant;

fn main() {
    let mut app = Application::new(1280, 720).unwrap();

    // TODO: Make the renderer work in a begin scene, submit and end scene way
    // Also make it independent from the Camera and Light class (?)
    // either by just passing matrices/vectors/whatever or by creating Traits
    let default_shader_path = app.resources.resource_name_to_path("shaders/flat.glsl");
    let mut default_shader = ShaderProgram::from_path(default_shader_path).unwrap();
    default_shader.bind();
    default_shader.set_uniform_vector3("u_color".to_string(), &Vector3::<f32>::new(1.0, 0.0, 1.0));
    let renderer = Renderer::new(default_shader);

    let mut meshes = Vec::<Mesh>::new();

    let plane_path = app.resources.resource_name_to_path("meshes/plane.obj");
    let plane = load_obj(plane_path, &app.resources).unwrap();
    meshes.push(plane);

    let obj_path = app.resources.resource_name_to_path("meshes/suzanne.obj");
    let obj = load_obj(obj_path, &app.resources).unwrap();
    meshes.push(obj);

    let (width, height) = app.window.size();
    let aspect = width as f32 / height as f32;

    let mut camera = Camera::from_focal_length(50.0, 36.0, 0.01, 1000.0, aspect);

    let mut light = Light::new();
    light.set_matrix(Matrix4::look_at_dir(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, -1.0, 1.0),
        Vector3::unit_y(),
    ));

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &app.window);
    let imgui_renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| app.video.gl_get_proc_address(s) as _);

    let mut last_frame = Instant::now();

    'main: loop {
        let mouse_state = MouseState::new(&app.events);
        for event in app.events.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) {
                continue;
            }
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window {
                    win_event: WindowEvent::SizeChanged(x, y),
                    ..
                } => unsafe {
                    gl::Viewport(0, 0, x, y);
                    camera.set_aspect_ratio(x as f32 / y as f32);
                },
                Event::MouseMotion { xrel, yrel, .. } => {
                    const PAN_SENSITIVITY: f32 = 0.005;
                    const ORBIT_SENSITIVITY: f32 = 0.01;
                    const ZOOM_SENSITIVITY: f32 = 0.01;

                    if mouse_state.left() {
                        let y_angle = xrel as f32 * ORBIT_SENSITIVITY;
                        let x_angle = yrel as f32 * ORBIT_SENSITIVITY;
                        camera.rotate(Vector3::unit_y(), y_angle);
                        camera.rotate(Vector3::unit_x(), x_angle);
                    } else if mouse_state.right() {
                        let mouse_vector = Vector2::new(xrel as f32, yrel as f32);
                        let mut direction = mouse_vector.dot(Vector2::unit_x())
                            + mouse_vector.dot(Vector2::unit_y());
                        if direction > 0.0 {
                            direction = 1.0;
                        } else if direction < 0.0 {
                            direction = -1.0;
                        }
                        let zoom_amount = direction * mouse_vector.magnitude() * ZOOM_SENSITIVITY;
                        if !zoom_amount.is_nan() {
                            camera.zoom(zoom_amount);
                        }
                    } else if mouse_state.middle() {
                        let x = xrel as f32 * PAN_SENSITIVITY;
                        let y = yrel as f32 * PAN_SENSITIVITY;
                        camera.pan(x, y);
                    }
                }
                Event::MouseWheel { y, .. } => camera.zoom(y as f32),
                Event::KeyDown { keycode, .. } => match keycode {
                    //TODO: refacto to focus on selection
                    Some(sdl2::keyboard::Keycode::F) => camera.focus(),
                    _ => (),
                },
                _ => {}
            }
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &app.window, &app.events.mouse_state());

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        imgui.io_mut().delta_time = delta_s;

        let ui = imgui.frame();

        let fps = 1 as f32 / delta_s;
        let fps_str = format!("{}", fps);
        ui.label_text(im_str!("FPS"), &ImString::new(fps_str));

        light.set_matrix(
            camera.matrix().inverse_transform().unwrap()
                * Matrix4::from_translation(Vector3::new(-2.0, 2.0, 1.0)),
        );

        renderer.clear(0.1, 0.1, 0.1);
        for mesh in &meshes {
            renderer.draw_mesh(&mesh, &camera, &light, gl::TRIANGLES);
        }

        imgui_sdl2.prepare_render(&ui, &app.window);
        imgui_renderer.render(ui);

        app.window.gl_swap_window();
    }
}
