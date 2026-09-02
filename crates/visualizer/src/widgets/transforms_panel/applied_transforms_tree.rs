use egui::Label;
use egui_ltreeview::{DirPosition, NodeBuilder};
use uuid::Uuid;

use crate::pipeline::{Pipeline, PipelineStep};

use super::available_transforms_tree::transform_kind_label;

enum TransformContextActions {
    Enable(Uuid),
    Disable(Uuid),
    Delete(Uuid),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
enum AppliedTransformNodeId {
    Root,
    Step(Uuid),
}

/// Отобразить примененные преобразования в виде дерева
/// Может менять преобразования в векторе местами и изменять сами преобразования
pub fn show_applied_transforms(
    ui: &mut egui::Ui,
    pipeline: &mut Pipeline,
    changed: &mut bool,
) -> egui::Response {
    let mut context_menu_actions = Vec::<TransformContextActions>::new();

    let id = ui.make_persistent_id("applied_transforms_tree");
    let (response, actions) = egui_ltreeview::TreeView::new(id)
        .allow_multi_selection(false)
        .allow_drag_and_drop(true)
        .show(ui, |builder| {
            builder.dir(AppliedTransformNodeId::Root, "Группа 1");
            for (idx, step) in pipeline.steps().iter().enumerate() {
                let node_id = AppliedTransformNodeId::Step(step.id());
                let node = NodeBuilder::leaf(node_id)
                    .label_ui(|ui| {
                        show_transform_node(ui, idx, step);
                    })
                    .context_menu(|ui| {
                        show_context_menu(ui, step.id(), &mut context_menu_actions);
                    });
                builder.node(node);
            }
            builder.close_dir();
        });

    for action in &actions {
        match action {
            egui_ltreeview::Action::Move(dnd) => {
                if let Some(AppliedTransformNodeId::Step(source)) = dnd.source.first()
                    && let Some(target_index) =
                        target_index_for_position(dnd.target, dnd.position, pipeline)
                {
                    *changed |= pipeline.move_step(*source, target_index);
                }
            }
            egui_ltreeview::Action::Activate(activate) => {
                if let Some(AppliedTransformNodeId::Step(step_id)) = activate.selected.first() {
                    pipeline.toggle(*step_id);
                    *changed = true;
                }
            }
            _ => {}
        }
    }

    for action in context_menu_actions {
        apply_action(&action, pipeline);
        *changed = true;
    }

    response
}

fn show_transform_node(ui: &mut egui::Ui, idx: usize, step: &PipelineStep) {
    ui.add_enabled(
        step.is_active(),
        Label::new(format!(
            "{}. {}",
            idx + 1,
            transform_kind_label(step.transform().kind())
        ))
        .selectable(false),
    );
}

fn show_context_menu(ui: &mut egui::Ui, step_id: Uuid, actions: &mut Vec<TransformContextActions>) {
    if ui.button("Включить").clicked() {
        actions.push(TransformContextActions::Enable(step_id));
        ui.close();
    }
    if ui.button("Выключить").clicked() {
        actions.push(TransformContextActions::Disable(step_id));
        ui.close();
    }
    ui.separator();
    if ui.button("Удалить").clicked() {
        actions.push(TransformContextActions::Delete(step_id));
        ui.close();
    }
}

fn apply_action(action: &TransformContextActions, pipeline: &mut Pipeline) {
    match *action {
        TransformContextActions::Enable(step_id) => {
            pipeline.activate(step_id);
        }
        TransformContextActions::Disable(step_id) => {
            pipeline.deactivate(step_id);
        }
        TransformContextActions::Delete(step_id) => {
            pipeline.remove(step_id);
        }
    }
}

fn target_index_for_position(
    target_dir: AppliedTransformNodeId,
    position_in_target_dir: DirPosition<AppliedTransformNodeId>,
    pipeline: &Pipeline,
) -> Option<usize> {
    // 1 - Группу нельзя переносить. 2 - Переносить можно только внутри первой группы
    if target_dir != AppliedTransformNodeId::Root {
        return None;
    }

    match position_in_target_dir {
        DirPosition::First => Some(0),
        DirPosition::Before(AppliedTransformNodeId::Step(id)) => step_index(pipeline, id),
        DirPosition::After(AppliedTransformNodeId::Step(id)) => {
            step_index(pipeline, id).map(|idx| idx + 1)
        }
        DirPosition::Last => Some(pipeline.steps().len()),
        DirPosition::Before(AppliedTransformNodeId::Root)
        | DirPosition::After(AppliedTransformNodeId::Root) => None,
    }
}

fn step_index(pipeline: &Pipeline, id: Uuid) -> Option<usize> {
    pipeline.steps().iter().position(|step| step.id() == id)
}
