mod app;
mod components;
mod graphics;
mod resources;

use app::Application;
use cgmath::prelude::*;
use cgmath::{Matrix4, Point3, Vector2, Vector3};
use components::{create_cube, Camera, Light};
use graphics::{Mesh, Renderer, ShaderProgram};
use sdl2::event::{Event, WindowEvent};
use sdl2::mouse::MouseState;

fn main() {
    let mut app = Application::new(900, 900).unwrap();

    // let mesh = create_cube(&app.gl, 0.5);
    let mesh = Mesh::from_res(&app.gl, &app.resources, "meshes/suzanne.obj").unwrap();

    let mut shader_program =
        ShaderProgram::from_res(&app.gl, &app.resources, "shaders/default").unwrap();
    let model_matrix = Matrix4::<f32>::identity();

    let (width, height) = app.window.size();
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

    loop {
        match process_frame(
            &mut app,
            &mut camera,
            &mut light,
            &mut shader_program,
            &model_matrix,
            &mesh,
        ) {
            true => break,
            false => (),
        }
    }
}

/// Return true if we shall exit the main loop.
fn process_frame(
    app: &mut Application,
    camera: &mut Camera,
    light: &mut Light,
    shader_program: &mut ShaderProgram,
    model_matrix: &Matrix4<f32>,
    mesh: &Mesh,
) -> bool {
    let mouse_state = MouseState::new(&app.events);
    for event in app.events.poll_iter() {
        match event {
            Event::Quit { .. } => return true,
            Event::Window {
                win_event: WindowEvent::SizeChanged(x, y),
                ..
            } => unsafe {
                app.gl.Viewport(0, 0, x, y);
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
                    let mut direction =
                        mouse_vector.dot(Vector2::unit_x()) + mouse_vector.dot(Vector2::unit_y());
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

    Renderer::clear(&app.gl, 0.1, 0.1, 0.1);
    Renderer::draw(&app.gl, &mesh, &shader_program, gl::TRIANGLES);

    app.window.gl_swap_window();
    return false;
}
