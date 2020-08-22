use crate::{Node, Scene, State};
use imgui::{im_str, Ui, Window};

pub fn outliner(ui: &Ui, scene: &Scene, mut state: State) -> State {
    Window::new(im_str!("Outliner"))
        .size([250.0, 300.0], imgui::Condition::FirstUseEver)
        .resizable(true)
        .build(&ui, || {
            state = add_node(ui, scene, scene.root_node(), state.clone());
        });
    state
}

fn add_node(ui: &Ui, scene: &Scene, node: &Node, mut state: State) -> State {
    let node_id = node.uuid();

    let name = im_str!("{}", node.name);
    let mut tree_node = imgui::TreeNode::new(&name).open_on_arrow(true);

    if node.children.len() == 0 {
        tree_node = tree_node.leaf(true);
    }
    tree_node = tree_node.selected(state.is_selected(&node_id));

    tree_node.build(ui, || {
        if ui.is_item_clicked(imgui::MouseButton::Left) {
            if state.is_selected(&node_id) {
                if ui.io().key_shift {
                    state.deselect(node_id);
                } else {
                    state.select(node_id, true);
                }
            } else {
                if ui.io().key_shift {
                    state.select(node_id, false);
                } else {
                    state.select(node_id, true);
                }
            }
        }

        for child_id in &node.children {
            let child_node = scene.node(&child_id);
            if let Some(node) = child_node {
                state = add_node(ui, scene, node, state.clone())
            }
        }
    });

    state
}
