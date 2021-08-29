use super::bounding_box::BoundingBox;
use super::environment::EnvironmentImpl;
use nalgebra::Vector3;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Object<'a> {
    environment: &'a RefCell<EnvironmentImpl>,
    inner: Rc<RefCell<ObjectImpl>>,
}

impl<'a> Object<'a> {
    pub fn new(
        environment: &'a RefCell<EnvironmentImpl>,
        inner: Rc<RefCell<ObjectImpl>>,
    ) -> Object<'a> {
        Self { environment, inner }
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.inner.borrow().position
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) {
        self.inner.borrow_mut().position = new_position;
    }
}

impl<'a> Drop for Object<'a> {
    fn drop(&mut self) {
        self.environment
            .borrow_mut()
            .remove_object(self.inner.borrow_mut().object_id);
    }
}

pub struct ObjectImpl {
    object_id: u32,
    position: Vector3<f32>,
    bounding_box: BoundingBox,
}

impl ObjectImpl {
    pub fn new(object_id: u32, bounding_box: BoundingBox) -> Self {
        Self {
            object_id,
            position: Vector3::new(0.0, 0.0, 0.0),
            bounding_box,
        }
    }
}
