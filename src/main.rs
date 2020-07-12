mod components;
mod graphics;
mod resources;

use cgmath::prelude::*;
use cgmath::{Matrix4, Point3, Vector2, Vector3};
use components::{create_cube, Camera, Light};
use graphics::{Renderer, ShaderProgram};
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

    let mesh = create_cube(&gl, 0.5);

    let mut shader_program = ShaderProgram::from_res(&gl, &resources, "shaders/default").unwrap();
    let model_matrix = Matrix4::<f32>::identity();

    let (width, height) = window.size();
    let aspect = width as f32 / height as f32;

    let mut camera = Camera::from_focal_length(50.0, 36.0, 0.01, 100.0, aspect);

    let mut light = Light {
        matrix: Matrix4::look_at_dir(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, -1.0, 1.0),
            Vector3::unit_y(),
        ),
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
            String::from("light_direction"),
            &light.matrix.transform_vector(Vector3::unit_z()),
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
