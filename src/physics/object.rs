use super::bounding_box::BoundingBox;
use super::environment::EnvironmentImpl;
use nalgebra::Vector3;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Object {
    object_id: u32,
    environment: Rc<RefCell<EnvironmentImpl>>,

    position: Vector3<f32>,
    pub bounding_box: BoundingBox,
}

impl Object {
    pub fn new(
        object_id: u32,
        environment: Rc<RefCell<EnvironmentImpl>>,
        bounding_box: BoundingBox,
    ) -> Self {
        Self {
            object_id,
            environment,
            position: Vector3::new(0.0, 0.0, 0.0),
            bounding_box,
        }
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) {
        self.position = new_position;
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        self.environment
            .as_ref()
            .borrow_mut()
            .remove_object(self.object_id);
    }
}
