pub mod render_gl;
pub mod resources;

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector2, Vector3};
use render_gl::{Camera, Light, Mesh, Renderer, Vertex};
use resources::Resources;
use sdl2::event::{Event, WindowEvent};
use sdl2::mouse::MouseState;
use std::path::Path;

fn main() {
    let resources = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Palantir", 900, 900)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    unsafe {
        gl.Viewport(0, 0, 900, 900);
        gl.Enable(gl::DEPTH_TEST);
    }

    let mut shader_program =
        render_gl::Program::from_res(&gl, &resources, "shaders/default").unwrap();

    #[rustfmt::skip]
    let vertices: Vec<Vertex> = vec![
        // Top
        Vertex { position: Vector3::new(0.5, 0.5, 0.5),     color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 1.0, 0.0) },
        Vertex { position: Vector3::new(-0.5, 0.5, 0.5),    color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 1.0, 0.0) },
        Vertex { position: Vector3::new(-0.5, 0.5, -0.5),   color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 1.0, 0.0) },
        Vertex { position: Vector3::new(0.5, 0.5, -0.5),    color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 1.0, 0.0) },

        // Bottom
        Vertex { position: Vector3::new(0.5, -0.5, 0.5),    color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, -1.0, 0.0) },
        Vertex { position: Vector3::new(-0.5, -0.5, 0.5),   color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, -1.0, 0.0) },
        Vertex { position: Vector3::new(-0.5, -0.5, -0.5),  color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, -1.0, 0.0) },
        Vertex { position: Vector3::new(0.5, -0.5, -0.5),   color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, -1.0, 0.0) },

        // Right
        Vertex { position: Vector3::new(0.5, 0.5, 0.5),     color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(1.0, 0.0, 0.0) },
        Vertex { position: Vector3::new(0.5, 0.5, -0.5),    color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(1.0, 0.0, 0.0) },
        Vertex { position: Vector3::new(0.5, -0.5, -0.5),   color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(1.0, 0.0, 0.0) },
        Vertex { position: Vector3::new(0.5, -0.5, 0.5),    color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(1.0, 0.0, 0.0) },

        // Left
        Vertex { position: Vector3::new(-0.5, 0.5, 0.5),    color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(-1.0, 0.0, 0.0) },
        Vertex { position: Vector3::new(-0.5, 0.5, -0.5),   color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(-1.0, 0.0, 0.0) },
        Vertex { position: Vector3::new(-0.5, -0.5, -0.5),  color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(-1.0, 0.0, 0.0) },
        Vertex { position: Vector3::new(-0.5, -0.5, 0.5),   color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(-1.0, 0.0, 0.0) },

        // Front
        Vertex { position: Vector3::new(0.5, 0.5, -0.5),    color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 0.0, -1.0) },
        Vertex { position: Vector3::new(-0.5, 0.5, -0.5),   color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 0.0, -1.0) },
        Vertex { position: Vector3::new(-0.5, -0.5, -0.5),  color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 0.0, -1.0) },
        Vertex { position: Vector3::new(0.5, -0.5, -0.5),   color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 0.0, -1.0) },

        // Back
        Vertex { position: Vector3::new(0.5, 0.5, 0.5),     color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 0.0, 1.0) },
        Vertex { position: Vector3::new(-0.5, 0.5, 0.5),    color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 0.0, 1.0) },
        Vertex { position: Vector3::new(-0.5, -0.5, 0.5),   color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 0.0, 1.0) },
        Vertex { position: Vector3::new(0.5, -0.5, 0.5),    color: Vector3::new(0.5, 0.5, 0.5),     normal: Vector3::new(0.0, 0.0, 1.0) },
    ];
    #[rustfmt::skip]
    let indices = vec![
        0, 1, 2, // top
        0, 2, 3,
        4, 5, 6, // bot
        4, 6, 7,
        8, 9, 10, // right
        8, 10, 11,
        12, 13, 14, // left
        12, 14, 15,
        16, 17, 18, // front
        16, 18, 19,
        20, 21, 22, // back
        20, 22, 23,
    ];
    let mesh = Mesh::new(&gl, vertices, indices);
    let model_matrix = Matrix4::<f32>::identity();

    let (width, height) = window.size();
    let aspect = width as f32 / height as f32;

    let mut camera = Camera::from_focal_length(50.0, 36.0, 0.01, 100.0, aspect);

    let mut light = Light {
        matrix: Matrix4::from_translation(Vector3::new(1.0, 1.0, 1.0)),
        color: Vector3::new(1.0, 1.0, 1.0),
        ambient_strength: 0.25,
        power: 1.0,
    };

    'main: loop {
        let mouse_state = MouseState::new(&event_pump);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window {
                    win_event: WindowEvent::SizeChanged(x, y),
                    ..
                } => unsafe {
                    gl.Viewport(0, 0, x, y);
                    camera.set_aspect_ratio(x as f32 / y as f32);
                },
                Event::MouseMotion { xrel, yrel, .. } => {
                    const ORBIT_SENSITIVITY: f32 = 0.01;
                    const ZOOM_SENSITIVITY: f32 = 0.01;

                    if mouse_state.left() {
                        let y_angle = xrel as f32 * ORBIT_SENSITIVITY;
                        let x_angle = yrel as f32 * ORBIT_SENSITIVITY;
                        camera.rotate(Vector3::unit_y(), y_angle);
                        camera.rotate(Vector3::unit_x(), x_angle);
                    } else if mouse_state.right() {
                        let mouse_vector = Vector2::new(xrel as f32, yrel as f32);
                        let mouse_vector_normalized = mouse_vector.clone().normalize();
                        let mut direction = mouse_vector_normalized.dot(Vector2::unit_x())
                            + mouse_vector_normalized.dot(-Vector2::unit_y());
                        if direction > 0.0 {
                            direction = 1.0;
                        } else if direction < 0.0 {
                            direction = -1.0;
                        }
                        let zoom_amount = direction * mouse_vector.magnitude() * ZOOM_SENSITIVITY;
                        if !zoom_amount.is_nan() {
                            camera.zoom(zoom_amount);
                        }
                    }
                }
                Event::MouseWheel { y, .. } => camera.zoom(y as f32),

                _ => {}
            }
        }

        light.matrix = camera.view_matrix().inverse_transform().unwrap()
            * Matrix4::from_translation(Vector3::new(-2.0, 2.0, 1.0));

        shader_program.set_uniform_matrix4(String::from("model"), &model_matrix);
        shader_program.set_uniform_matrix4(String::from("view"), &camera.view_matrix());
        shader_program.set_uniform_matrix4(String::from("projection"), &camera.projection_matrix());
        shader_program.set_uniform_vector3(
            String::from("light_position"),
            &position_from_matrix(&light.matrix),
        );
        shader_program.set_uniform_vector3(String::from("light_color"), &light.color);
        shader_program.set_uniform_float(
            String::from("light_ambient_strength"),
            light.ambient_strength,
        );
        shader_program.set_uniform_float(String::from("light_power"), light.power);

        Renderer::clear(&gl, 0.1, 0.1, 0.1);
        Renderer::draw(&gl, &mesh, &shader_program, gl::TRIANGLES);

        window.gl_swap_window();
    }
}

fn position_from_matrix(matrix: &Matrix4<f32>) -> Vector3<f32> {
    Vector3::new(matrix.w.x, matrix.w.y, matrix.w.z)
}
