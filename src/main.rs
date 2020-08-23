mod app;
mod components;
mod resources;
mod scene;

use crate::components::{MeshComponent, TransformComponent};
use app::Application;
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector2, Vector3};
use components::{Camera, Light};
use imgui::{im_str, Context, Window};
use legion::prelude::*;
use nfd::Response;
use palantir_lib::{Renderer, ShaderProgram, TCamera};
use scene::Scene;
use sdl2::event::{Event, WindowEvent};
use sdl2::mouse::MouseState;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let mut app = Application::new(1280, 720).unwrap();
    let (width, height) = app.window.size();
    let aspect = width as f32 / height as f32;

    let universe = Universe::new();
    let mut world = universe.create_world();

    let mut scene = Scene::new();
    scene.camera_mut().set_aspect_ratio(aspect);

    let lambert_shader_path = app.resources.resource_name_to_path("shaders/lambert.glsl");
    let lambert_shader = ShaderProgram::from_path(lambert_shader_path).unwrap();
    let mut renderer = Renderer::new(lambert_shader);

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &app.window);
    let imgui_renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| app.video.gl_get_proc_address(s) as _);

    let mut last_frame = Instant::now();

    'main: loop {
        // EVENT HANDLING
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
                    scene.camera_mut().set_aspect_ratio(x as f32 / y as f32);
                },
                Event::MouseMotion { xrel, yrel, .. } => {
                    const PAN_SENSITIVITY: f32 = 0.005;
                    const ORBIT_SENSITIVITY: f32 = 0.01;
                    const ZOOM_SENSITIVITY: f32 = 0.01;

                    if mouse_state.left() {
                        let y_angle = xrel as f32 * ORBIT_SENSITIVITY;
                        let x_angle = yrel as f32 * ORBIT_SENSITIVITY;
                        scene.camera_mut().rotate(Vector3::unit_y(), y_angle);
                        scene.camera_mut().rotate(Vector3::unit_x(), x_angle);
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
                            scene.camera_mut().zoom(zoom_amount);
                        }
                    } else if mouse_state.middle() {
                        let x = xrel as f32 * PAN_SENSITIVITY;
                        let y = yrel as f32 * PAN_SENSITIVITY;
                        scene.camera_mut().pan(x, y);
                    }
                }
                Event::MouseWheel { y, .. } => scene.camera_mut().zoom(y as f32),
                Event::KeyDown { keycode, .. } => match keycode {
                    //TODO: refacto to focus on selection
                    Some(sdl2::keyboard::Keycode::F) => scene.camera_mut().focus(),
                    _ => (),
                },
                _ => {}
            }
        }

        // RENDER SCENE
        let light_matrix = scene.camera().matrix().inverse_transform().unwrap()
            * Matrix4::from_translation(Vector3::new(-2.0, 2.0, 1.0));
        scene.light_mut().set_matrix(light_matrix);

        renderer.clear(0.1, 0.1, 0.1);

        let render_meshes_query = <(Read<TransformComponent>, Read<MeshComponent>)>::query();
        for (transform, mesh) in render_meshes_query.iter(&mut world) {
            renderer.draw_mesh(
                &transform.matrix,
                &mesh.mesh,
                scene.camera(),
                scene.light(),
                gl::TRIANGLES,
            );
        }

        // IMGUI STUFF
        imgui_sdl2.prepare_frame(imgui.io_mut(), &app.window, &app.events.mouse_state());

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        imgui.io_mut().delta_time = delta_s;

        let ui = imgui.frame();

        let fps = 1 as f32 / delta_s;

        Window::new(im_str!("Debug"))
            .always_auto_resize(true)
            .resizable(true)
            .build(&ui, || {
                ui.label_text(&im_str!("{}", fps as i32), im_str!("FPS"));
                ui.label_text(
                    &im_str!("{}", render_meshes_query.iter(&world).count()),
                    im_str!("Mesh Count"),
                );

                let import_button = ui.button(im_str!("Import"), [100.0, 25.0]);
                if import_button {
                    let file_choice = nfd::dialog_multiple()
                        .filter("obj")
                        .default_path(
                            app.resources
                                .root_path()
                                .to_str()
                                .expect("Could not convert path buffer to string."),
                        )
                        .open();

                    if let Ok(file) = file_choice {
                        match file {
                            Response::Okay(path) => {
                                world.insert(
                                    (),
                                    (0..1).map(|_| {
                                        (
                                            TransformComponent {
                                                matrix: Matrix4::identity(),
                                            },
                                            MeshComponent {
                                                mesh: scene
                                                    .load_obj(
                                                        PathBuf::from(path.clone()),
                                                        &app.resources,
                                                    )
                                                    .expect("Error loading mesh."),
                                            },
                                        )
                                    }),
                                );
                            }
                            Response::OkayMultiple(paths) => {
                                for path in paths {
                                    world.insert(
                                        (),
                                        (0..1).map(|_| {
                                            (
                                                TransformComponent {
                                                    matrix: Matrix4::identity(),
                                                },
                                                MeshComponent {
                                                    mesh: scene
                                                        .load_obj(
                                                            PathBuf::from(path.clone()),
                                                            &app.resources,
                                                        )
                                                        .expect("Error loading mesh."),
                                                },
                                            )
                                        }),
                                    );
                                }
                            }
                            Response::Cancel => (),
                        }
                    }
                }
            });

        imgui_sdl2.prepare_render(&ui, &app.window);
        imgui_renderer.render(ui);

        app.window.gl_swap_window();
    }
}
