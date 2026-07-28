use egui::Label;
use egui_ltreeview::{DirPosition, NodeBuilder};

use crate::widgets::transforms_window::{common::make_rich_text, data::AppliedTransform};

enum TransformContextActions {
    Enable(usize),
    Disable(usize),
    Delete(usize),
}

static GROUP_1_ID: usize = 0;
static FIRST_TRANSFORM_ID: usize = 1;

/// Отобразить примененные преобразования в виде дерева
/// Может менять преобразования в векторе местами и изменять сами преобразования
pub fn show_applied_transforms(
    ui: &mut egui::Ui,
    transforms: &mut Vec<AppliedTransform>,
) -> egui::Response {
    let mut context_menu_actions = Vec::<TransformContextActions>::new();

    let (response, actions) = egui_ltreeview::TreeView::new(egui::Id::new("tree view cur"))
        .allow_multi_selection(false)
        .allow_drag_and_drop(true)
        .show(ui, |builder| {
            builder.dir(GROUP_1_ID, "Группа 1");
            for (idx, transform) in transforms.iter().enumerate() {
                let node_id = idx + FIRST_TRANSFORM_ID;
                let node = NodeBuilder::leaf(node_id)
                    .label_ui(|ui| {
                        show_transform_node(ui, node_id, transform);
                    })
                    .context_menu(|ui| {
                        show_context_menu(ui, node_id, &mut context_menu_actions);
                    });
                builder.node(node);
            }
            builder.close_dir();
        });

    for action in &actions {
        match action {
            egui_ltreeview::Action::Move(dnd) => {
                if let Some(source) = dnd.source.first() {
                    move_transform(*source, dnd.target, dnd.position, transforms);
                }
            }
            egui_ltreeview::Action::Activate(activate) => {
                if let Some(node_id) = activate.selected.first() {
                    toggle_transform_active(*node_id, transforms);
                }
            }
            _ => {}
        }
    }

    for action in context_menu_actions {
        apply_action(&action, transforms);
    }

    response
}

fn show_transform_node(ui: &mut egui::Ui, node_id: usize, transform: &AppliedTransform) {
    ui.add_enabled(
        transform.is_active(),
        Label::new(format!("{}. {}", node_id, transform.kind.name())).selectable(false),
    );
}

fn show_context_menu(
    ui: &mut egui::Ui,
    node_id: usize,
    actions: &mut Vec<TransformContextActions>,
) {
    if ui.button("Включить").clicked() {
        actions.push(TransformContextActions::Enable(node_id));
        ui.close();
    }
    if ui.button("Выключить").clicked() {
        actions.push(TransformContextActions::Disable(node_id));
        ui.close();
    }
    ui.separator();
    if ui.button("Удалить").clicked() {
        actions.push(TransformContextActions::Delete(node_id));
        ui.close();
    }
}

fn apply_action(action: &TransformContextActions, transforms: &mut Vec<AppliedTransform>) {
    match *action {
        TransformContextActions::Enable(node_id) => {
            activate_transform(node_id, transforms);
        }
        TransformContextActions::Disable(node_id) => {
            deactivate_transform(node_id, transforms);
        }
        TransformContextActions::Delete(node_id) => {
            delete_transform(node_id, transforms);
        }
    }
}

fn move_transform<T>(
    moved_node_idx: usize,
    target_dir: usize,
    position_in_target_dir: DirPosition<usize>,
    transforms: &mut [T],
) {
    // 1 - Группу нельзя переносить. 2 - Переносить можно только внутри первой группы
    if moved_node_idx == GROUP_1_ID || target_dir != GROUP_1_ID {
        return;
    }

    // Элемент в векторе преобразований, который нужно перенести
    let applied_vec_source_idx = moved_node_idx - 1;

    // Место, на котором должен оказаться перемещаемый элемент в векторе преобразований
    let applied_vec_target_idx = match position_in_target_dir {
        DirPosition::First => 1,
        DirPosition::Before(node_id) => node_id,
        DirPosition::After(node_id) => node_id + 1,
        DirPosition::Last => transforms.len() + 1,
    } - 1;

    if applied_vec_source_idx == applied_vec_target_idx {
        return;
    }

    if applied_vec_source_idx < applied_vec_target_idx {
        transforms
            .get_mut(applied_vec_source_idx..applied_vec_target_idx)
            .expect(
                "Слайс для перемещения элемента, \
                    когда source_idx < target_idx, вычислен неверно",
            )
            .rotate_left(1);
    } else {
        transforms
            .get_mut(applied_vec_target_idx..applied_vec_source_idx + 1)
            .expect(
                "Слайс для перемещения элемента, \
                    когда target_idx < source_idx, вычислен неверно",
            )
            .rotate_right(1);
    }
}

fn get_node_mut(
    node_id: usize,
    transforms: &mut [AppliedTransform],
) -> Option<&mut AppliedTransform> {
    if node_id == GROUP_1_ID {
        None
    } else {
        Some(
            transforms
                .get_mut(node_id - FIRST_TRANSFORM_ID)
                .expect("ID ноды превышает количество преобразований в векторе"),
        )
    }
}

fn toggle_transform_active(node_id: usize, transforms: &mut [AppliedTransform]) {
    if let Some(node) = get_node_mut(node_id, transforms) {
        node.toggle_active();
    }
}

fn activate_transform(node_id: usize, transforms: &mut [AppliedTransform]) {
    if let Some(node) = get_node_mut(node_id, transforms) {
        node.activate();
    }
}

fn deactivate_transform(node_id: usize, transforms: &mut [AppliedTransform]) {
    if let Some(node) = get_node_mut(node_id, transforms) {
        node.deactivate();
    }
}

fn delete_transform(node_id: usize, transforms: &mut Vec<AppliedTransform>) {
    if node_id != GROUP_1_ID {
        let vec_idx = node_id - FIRST_TRANSFORM_ID;
        assert!(
            vec_idx <= transforms.len(),
            "ID ноды превышает количество преобразований в векторе"
        );
        transforms.remove(vec_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_transforms() -> Vec<usize> {
        vec![1, 2, 3, 4]
    }

    #[test]
    fn move_transform_first() {
        let moved_node_idx = 4;

        let mut transforms = make_test_transforms();
        move_transform(moved_node_idx, 0, DirPosition::First, &mut transforms);

        let mut expected = make_test_transforms();
        expected
            .get_mut(0..moved_node_idx)
            .expect("Выход за границы вектора")
            .rotate_right(1);

        assert_eq!(transforms, expected);
    }
}
