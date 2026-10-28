use std::collections::{HashMap, HashSet, hash_map::Entry};

use egui_plot::PlotPoint;
use uuid::Uuid;

use crate::{
    pipeline::{Pipeline, PipelineStep, Transform},
    widgets::transforms_panel::transforms::transform_single,
};

const CURVE_POINTS_COUNT: usize = 100;

struct Curve {
    transform: Transform,
    points: Vec<PlotPoint>,
}

#[derive(Default)]
pub struct TransformPanelCache {
    curves: HashMap<Uuid, Curve>,
}

impl TransformPanelCache {
    pub fn curve_points(&mut self, step_id: Uuid, transform: Transform) -> &[PlotPoint] {
        &match self.curves.entry(step_id) {
            Entry::Occupied(mut entry) => {
                if entry.get().transform != transform {
                    entry.insert(Curve {
                        points: calculate_transform_points(&transform),
                        transform,
                    });
                }
                entry.into_mut()
            }
            Entry::Vacant(entry) => entry.insert(Curve {
                points: calculate_transform_points(&transform),
                transform,
            }),
        }
        .points
        .as_slice()
    }

    pub fn drop_stale(&mut self, pipeline: &Pipeline) {
        let live_ids: HashSet<_> = pipeline.steps().iter().map(PipelineStep::id).collect();
        self.curves.retain(|id, _| live_ids.contains(id));
    }
}

// #[allow(clippy::cast_precision_loss)]

#[allow(clippy::cast_precision_loss)]
fn calculate_transform_points(transform: &Transform) -> Vec<PlotPoint> {
    (0..CURVE_POINTS_COUNT)
        .map(|x| {
            let x_norm = x as f64 / CURVE_POINTS_COUNT as f64;
            PlotPoint::new(x_norm, transform_single(x_norm, transform))
        })
        .collect()
}
