use egui_plot::PlotPoint;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct GammaCorrectionData {
    constant: f64,
    gamma: f64,

    #[serde(skip)]
    plot_points: Vec<PlotPoint>,
}

impl Default for GammaCorrectionData {
    fn default() -> Self {
        let mut instance = Self {
            constant: 1.,
            gamma: 1.,
            plot_points: vec![],
        };
        instance.calculate_plot();
        instance
    }
}

impl GammaCorrectionData {
    const PLOT_POINTS_COUNT: usize = 255;

    #[allow(clippy::cast_precision_loss)]
    fn calculate_plot(&mut self) {
        self.plot_points = (0..GammaCorrectionData::PLOT_POINTS_COUNT)
            .map(|x| PlotPoint::new(x as f64 / GammaCorrectionData::PLOT_POINTS_COUNT as f64, 0.))
            .collect();
        self.set_parameters(self.constant, self.gamma);
    }

    pub fn set_parameters(&mut self, constant: f64, gamma: f64) {
        assert!(constant >= 1., "Константа должна быть больше, либо равна 1");
        assert!(gamma > 0., "Коеффициенты гамма должен быть больше 0");
        assert!(
            self.plot_points.len() == GammaCorrectionData::PLOT_POINTS_COUNT,
            "Не выделен массив под график"
        );

        self.constant = constant;
        self.gamma = gamma;

        for (x, point) in self.plot_points.iter_mut().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let x_norm = x as f64 / GammaCorrectionData::PLOT_POINTS_COUNT as f64;
            point.x = x_norm;
            point.y = constant * (point.x.powf(gamma));
        }
    }

    pub fn reset_parameters(&mut self) {
        *self = Self::default();
    }

    pub fn constant(&self) -> f64 {
        self.constant
    }

    pub fn gamma(&self) -> f64 {
        self.gamma
    }

    pub fn plot_points(&self) -> &Vec<PlotPoint> {
        &self.plot_points
    }

    pub fn restore(&mut self) {
        self.calculate_plot();
    }
}
