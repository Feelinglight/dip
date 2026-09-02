use egui_ltreeview::TreeViewBuilder;

use crate::pipeline::{Transform, TransformKind};

#[derive(Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize, Debug)]
enum TransformNodeId {
    IntensityDir,
    Transform(TransformKind),
}

/// Отобразить все возможные преобразования в виде дерева
/// При активации ноды преобразования (дабл клик) вызывает функцию `on_activate`
pub fn show_available_transforms(
    ui: &mut egui::Ui,
    mut on_activate: impl FnMut(TransformKind),
) -> egui::Response {
    let id = ui.make_persistent_id("available_transforms_tree");
    let (response, actions) = egui_ltreeview::TreeView::new(id)
        .min_width(100.)
        .allow_multi_selection(false)
        .allow_drag_and_drop(false)
        .show(ui, |builder| {
            builder.dir(TransformNodeId::IntensityDir, "Яркость");
            for kind in Transform::available_kinds() {
                build_leaf(builder, *kind);
            }
            builder.close_dir();
        });

    for action in &actions {
        if let egui_ltreeview::Action::Activate(activate) = action {
            activate.selected.iter().for_each(|node_id| {
                if let TransformNodeId::Transform(transform_kind) = node_id {
                    on_activate(*transform_kind);
                }
            });
        }
    }
    response
}

fn build_leaf(builder: &mut TreeViewBuilder<TransformNodeId>, transform_kind: TransformKind) {
    let leaf_name = transform_kind_label(transform_kind);
    builder.leaf(TransformNodeId::Transform(transform_kind), leaf_name);
}

pub(super) const fn transform_kind_label(transform_kind: TransformKind) -> &'static str {
    match transform_kind {
        TransformKind::Negative => "Негатив",
        TransformKind::GammaCorrection => "Гамма-коррекция",
        TransformKind::LogTransform => "Логарифм. преобраз.",
    }
}
