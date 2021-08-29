use super::bounding_box::BoundingBox;
use super::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Environment {
    inner: Rc<RefCell<EnvironmentImpl>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            inner: Rc::new(RefCell::new(EnvironmentImpl::new())),
        }
    }

    /// Add an object to the environment.
    /// Remains in the environment until returned object ref drops.
    pub fn new_object(&mut self, bounding_box: BoundingBox) -> Object {
        Object::new(
            self.inner.as_ref().borrow_mut().new_object(),
            self.inner.clone(),
            bounding_box,
        )
    }
}

pub struct EnvironmentImpl {}

impl EnvironmentImpl {
    pub fn new() -> EnvironmentImpl {
        EnvironmentImpl {}
    }

    fn new_object(&mut self) -> u32 {
        0
    }

    pub fn remove_object(&mut self, object_id: u32) {}
}
