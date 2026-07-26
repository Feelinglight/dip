use uuid::Uuid;

#[derive(Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize, Debug)]
pub enum TransformKind {
    Negative,
    GammaCorrection,
}

impl TransformKind {
    pub fn name(&self) -> &'static str {
        match self {
            TransformKind::Negative => "Негатив",
            TransformKind::GammaCorrection => "Гамма-коррекция",
        }
    }

    pub fn default_parameters(&self) -> TransformParameters {
        match self {
            TransformKind::Negative => TransformParameters::Negative,
            TransformKind::GammaCorrection => TransformParameters::gamma_correction(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum TransformParameters {
    Negative,
    GammaCorrection(f64),
}

impl TransformParameters {
    pub fn gamma_correction() -> Self {
        TransformParameters::GammaCorrection(1.)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AppliedTransform {
    pub id: Uuid,
    pub kind: TransformKind,
    pub parameters: TransformParameters,
    active: bool,
}

impl AppliedTransform {
    pub fn new(kind: TransformKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            parameters: kind.default_parameters(),
            kind,
            active: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn toggle_active(&mut self) {
        self.active = !self.active;
    }
}
