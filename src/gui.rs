use crate::components::{MeshComponent, TransformComponent};
use crate::{Application, Scene};
use cgmath::prelude::*;
use cgmath::Matrix4;
use imgui::{im_str, Ui, Window};
use legion::prelude::*;
use nfd::Response;
use std::path::PathBuf;

pub fn debug_ui(ui: &Ui, fps: i32, scene: &mut Scene, world: &mut World, app: &Application) {
    Window::new(im_str!("Debug"))
        .always_auto_resize(true)
        .resizable(true)
        .build(&ui, || {
            let render_meshes_query = <(Read<TransformComponent>, Read<MeshComponent>)>::query();
            ui.label_text(&im_str!("{}", fps), im_str!("FPS"));
            ui.label_text(
                &im_str!("{}", render_meshes_query.iter(world).count()),
                im_str!("Mesh Count"),
            );

            let import_button_released = ui.button(im_str!("Import"), [100.0, 25.0]);
            if import_button_released {
                on_import_button_released(app, scene, world)
            }
        });
}

fn on_import_button_released(app: &Application, scene: &mut Scene, world: &mut World) {
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
                                    .load_obj(PathBuf::from(path.clone()), &app.resources)
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
                                        .load_obj(PathBuf::from(path.clone()), &app.resources)
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
