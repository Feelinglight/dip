use std::collections::{HashMap, HashSet};

use egui_plot::PlotPoint;
use intensity::{
    gamma_correction::{GammaCorrect, GammaCorrectionParams},
    log_transform::{LogTransform, LogTransformParams},
};
use uuid::Uuid;

use crate::pipeline::{Pipeline, PipelineStep};

#[derive(Default)]
pub struct TransformEditorCache {
    curves: HashMap<Uuid, CurveCache>,
}

pub enum CurveCache {
    Gamma {
        params_snapshot: GammaCorrectionParams,
        points: Vec<PlotPoint>,
    },
    Log {
        params_snapshot: LogTransformParams,
        points: Vec<PlotPoint>,
    },
}

impl TransformEditorCache {
    pub fn gamma_points(&mut self, step_id: Uuid, params: &GammaCorrectionParams) -> &[PlotPoint] {
        let cache = self
            .curves
            .entry(step_id)
            .or_insert_with(|| CurveCache::gamma(params));
        if cache.gamma_params_changed(params) {
            *cache = CurveCache::gamma(params);
        }
        cache.points()
    }

    pub fn log_points(&mut self, step_id: Uuid, params: &LogTransformParams) -> &[PlotPoint] {
        let cache = self
            .curves
            .entry(step_id)
            .or_insert_with(|| CurveCache::log(params));
        if cache.log_params_changed(params) {
            *cache = CurveCache::log(params);
        }
        cache.points()
    }

    pub fn retain_pipeline_steps(&mut self, pipeline: &Pipeline) {
        let live_ids: HashSet<_> = pipeline.steps().iter().map(PipelineStep::id).collect();
        self.curves.retain(|id, _| live_ids.contains(id));
    }
}

impl CurveCache {
    const GAMMA_POINTS_COUNT: usize = 100;
    const LOG_POINTS_COUNT: usize = 100;

    fn gamma(params: &GammaCorrectionParams) -> Self {
        let points = calculate_points(Self::GAMMA_POINTS_COUNT, |x| {
            image::GrayImage::gamma_correct_single(x, params, 1.)
        });
        Self::Gamma {
            params_snapshot: params.clone(),
            points,
        }
    }

    fn log(params: &LogTransformParams) -> Self {
        let points = calculate_points(Self::LOG_POINTS_COUNT, |x| {
            image::GrayImage::log_transform_single(x, params, 1.)
        });
        Self::Log {
            params_snapshot: params.clone(),
            points,
        }
    }

    fn gamma_params_changed(&self, params: &GammaCorrectionParams) -> bool {
        !matches!(
            self,
            Self::Gamma {
                params_snapshot,
                ..
            } if params_snapshot == params
        )
    }

    fn log_params_changed(&self, params: &LogTransformParams) -> bool {
        !matches!(
            self,
            Self::Log {
                params_snapshot,
                ..
            } if params_snapshot == params
        )
    }

    fn points(&self) -> &[PlotPoint] {
        match self {
            Self::Gamma { points, .. } | Self::Log { points, .. } => points,
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn calculate_points(count: usize, y: impl Fn(f64) -> f64) -> Vec<PlotPoint> {
    (0..count)
        .map(|x| {
            let x_norm = x as f64 / count as f64;
            PlotPoint::new(x_norm, y(x_norm))
        })
        .collect()
}
