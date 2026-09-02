use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel};
use uuid::Uuid;

use super::{Transform, TransformKind};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PipelineStep {
    id: Uuid,
    transform: Transform,
    active: bool,
}

impl PipelineStep {
    #[must_use]
    pub fn new(kind: TransformKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Transform::new(kind),
            active: true,
        }
    }

    #[must_use]
    pub const fn id(&self) -> Uuid {
        self.id
    }

    #[must_use]
    pub const fn transform(&self) -> &Transform {
        &self.transform
    }

    #[must_use]
    pub const fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.active
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    pub fn apply_to<P, Container>(&self, image_buffer: &mut ImageBuffer<P, Container>)
    where
        P: Pixel,
        Container: Deref<Target = [P::Subpixel]> + DerefMut,
    {
        if self.active {
            self.transform.apply(image_buffer);
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Pipeline {
    steps: Vec<PipelineStep>,
}

impl Pipeline {
    #[must_use]
    pub fn steps(&self) -> &[PipelineStep] {
        &self.steps
    }

    pub fn steps_mut_for_params(&mut self) -> &mut [PipelineStep] {
        &mut self.steps
    }

    pub fn add(&mut self, kind: TransformKind) -> Uuid {
        let step = PipelineStep::new(kind);
        let id = step.id();
        self.steps.push(step);
        id
    }

    pub fn clear(&mut self) {
        self.steps.clear();
    }

    pub fn remove(&mut self, id: Uuid) -> Option<PipelineStep> {
        let index = self.index_of(id)?;
        Some(self.steps.remove(index))
    }

    pub fn move_step(&mut self, id: Uuid, target_index: usize) -> bool {
        let Some(source_index) = self.index_of(id) else {
            return false;
        };
        let target_index = target_index.min(self.steps.len());
        let target_index = if target_index > source_index {
            target_index - 1
        } else {
            target_index
        };

        if source_index == target_index {
            return false;
        }

        let step = self.steps.remove(source_index);
        self.steps.insert(target_index, step);
        true
    }

    pub fn toggle(&mut self, id: Uuid) -> bool {
        self.step_mut(id).is_some_and(|step| {
            step.toggle();
            true
        })
    }

    pub fn activate(&mut self, id: Uuid) -> bool {
        self.step_mut(id).is_some_and(|step| {
            step.activate();
            true
        })
    }

    pub fn deactivate(&mut self, id: Uuid) -> bool {
        self.step_mut(id).is_some_and(|step| {
            step.deactivate();
            true
        })
    }

    pub fn apply_to<P, Container>(&self, image_buffer: &mut ImageBuffer<P, Container>)
    where
        P: Pixel,
        Container: Deref<Target = [P::Subpixel]> + DerefMut,
    {
        for step in &self.steps {
            step.apply_to(image_buffer);
        }
    }

    fn index_of(&self, id: Uuid) -> Option<usize> {
        self.steps.iter().position(|step| step.id == id)
    }

    fn step_mut(&mut self, id: Uuid) -> Option<&mut PipelineStep> {
        self.steps.iter_mut().find(|step| step.id == id)
    }
}
