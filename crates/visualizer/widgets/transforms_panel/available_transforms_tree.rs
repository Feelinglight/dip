use egui_ltreeview::TreeViewBuilder;

use crate::widgets::transforms_panel::data::TransformKind;

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
    let (response, actions) = egui_ltreeview::TreeView::new(egui::Id::new("tree view"))
        .min_width(100.)
        .allow_multi_selection(false)
        .allow_drag_and_drop(false)
        .show(ui, |builder| {
            builder.dir(TransformNodeId::IntensityDir, "Яркость");
            build_leaf(builder, TransformKind::Negative);
            build_leaf(builder, TransformKind::GammaCorrection);
            builder.close_dir();
        });

    for action in &actions {
        if let egui_ltreeview::Action::Activate(activate) = action {
            activate.selected.iter().for_each(|node_id| {
                if let TransformNodeId::Transform(transform_kind) = node_id {
                    on_activate(transform_kind.clone());
                }
            });
        }
    }
    response
}

fn build_leaf(builder: &mut TreeViewBuilder<TransformNodeId>, transform_kind: TransformKind) {
    let leaf_name = transform_kind.name();
    builder.leaf(TransformNodeId::Transform(transform_kind), leaf_name);
}
