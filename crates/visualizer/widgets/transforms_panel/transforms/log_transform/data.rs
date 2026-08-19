use egui_plot::PlotPoint;
use intensity::graduation::log_transform_single;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct LogTransformData {
    constant: f64,
    log_base: f64,

    #[serde(skip)]
    plot_points: Vec<PlotPoint>,
}

impl Default for LogTransformData {
    fn default() -> Self {
        let mut instance = Self {
            constant: 1.,
            log_base: 50.,
            plot_points: vec![],
        };
        instance.calculate_plot();
        instance
    }
}

impl LogTransformData {
    const PLOT_POINTS_COUNT: usize = 1000;

    #[allow(clippy::cast_precision_loss)]
    fn calculate_plot(&mut self) {
        self.plot_points = (0..LogTransformData::PLOT_POINTS_COUNT)
            .map(|x| PlotPoint::new(x as f64 / LogTransformData::PLOT_POINTS_COUNT as f64, 0.))
            .collect();
        self.set_parameters(self.constant, self.log_base);
    }

    pub fn set_parameters(&mut self, constant: f64, log_base: f64) {
        assert!(constant >= 1., "Константа должна быть больше, либо равна 1");
        assert!(
            log_base > 0. && (log_base - 1.).abs() > 0.0001,
            "Основание логарифма должно быть больше 0 и не равно 1"
        );
        assert!(
            self.plot_points.len() == LogTransformData::PLOT_POINTS_COUNT,
            "Не выделен массив под график"
        );

        self.constant = constant;
        self.log_base = log_base;

        #[allow(clippy::cast_precision_loss)]
        for (x, point) in self.plot_points.iter_mut().enumerate() {
            let x_norm = x as f64 / LogTransformData::PLOT_POINTS_COUNT as f64;
            point.x = x_norm;
            point.y = log_transform_single(x_norm, log_base, constant, 1.);
        }
    }

    pub fn reset_parameters(&mut self) {
        *self = Self::default();
    }

    pub fn constant(&self) -> f64 {
        self.constant
    }

    pub fn log_base(&self) -> f64 {
        self.log_base
    }

    pub fn plot_points(&self) -> &Vec<PlotPoint> {
        &self.plot_points
    }

    pub fn restore(&mut self) {
        self.calculate_plot();
    }
}
