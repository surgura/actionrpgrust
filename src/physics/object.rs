use super::bounding_box::BoundingBox;
use super::environment::EnvironmentImpl;
use nalgebra::Vector3;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Object {
    environment: Rc<RefCell<EnvironmentImpl>>,
    inner: Rc<RefCell<ObjectImpl>>,
}

impl Object {
    pub fn new(
        environment: Rc<RefCell<EnvironmentImpl>>,
        inner: Rc<RefCell<ObjectImpl>>,
    ) -> Object {
        Self { environment, inner }
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.inner.borrow().position
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) {
        self.inner.borrow_mut().position = new_position;
    }

    pub fn set_velocity(&mut self, new_velocity: Vector3<f32>) {
        self.inner.borrow_mut().velocity = new_velocity;
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        self.environment
            .borrow_mut()
            .remove_object(self.inner.borrow_mut().object_id);
    }
}

pub struct ObjectImpl {
    pub object_id: u32,
    pub position: Vector3<f32>,
    pub bounding_box: BoundingBox,
    pub velocity: Vector3<f32>,
}

impl ObjectImpl {
    pub fn new(object_id: u32, bounding_box: BoundingBox) -> Self {
        Self {
            object_id,
            position: Vector3::new(0.0, 0.0, 0.0),
            bounding_box,
            velocity: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
