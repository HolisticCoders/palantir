use crate::{Application, Scene};
use imgui::{im_str, Ui, Window};
use nfd::Response;
use std::path::PathBuf;

pub fn debug_ui(ui: &Ui, fps: i32, scene: &mut Scene, app: &Application) {
    Window::new(im_str!("Debug"))
        .always_auto_resize(true)
        .resizable(true)
        .build(&ui, || {
            ui.label_text(&im_str!("{}", fps), im_str!("FPS"));
            ui.label_text(&im_str!("{}", scene.meshes().len()), im_str!("Mesh Count"));

            let import_button_released = ui.button(im_str!("Import"), [100.0, 25.0]);
            if import_button_released {
                on_import_button_released(app, scene)
            }
        });
}

fn on_import_button_released(app: &Application, scene: &mut Scene) {
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
            Response::Okay(path) => scene.load_obj(PathBuf::from(path), &app.resources).unwrap(),
            Response::OkayMultiple(paths) => {
                for path in paths {
                    scene.load_obj(PathBuf::from(path), &app.resources).unwrap();
                }
            }
            Response::Cancel => (),
        }
    }
}
