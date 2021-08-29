use super::bounding_box::BoundingBox;
use super::object::{Object, ObjectImpl};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Environment {
    inner: RefCell<EnvironmentImpl>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            inner: RefCell::new(EnvironmentImpl::new()),
        }
    }

    /// Add an object to the environment.
    /// Remains in the environment until returned object ref drops.
    pub fn new_object<'a>(&'a mut self, bounding_box: BoundingBox) -> Object<'a> {
        let object = self.inner.borrow_mut().new_object(bounding_box);
        Object::new(&self.inner, object)
    }

    pub fn get_collisions(&self, bounding_box: BoundingBox) -> Vec<u32> {
        todo!();
        Vec::new()
    }
}

pub struct EnvironmentImpl {
    // TODO this datastructure can be significantly improved
    objects: Vec<Rc<RefCell<ObjectImpl>>>,
}

impl EnvironmentImpl {
    pub fn new() -> EnvironmentImpl {
        EnvironmentImpl {
            objects: Vec::new(),
        }
    }

    fn new_object(&mut self, bounding_box: BoundingBox) -> Rc<RefCell<ObjectImpl>> {
        self.objects.push(Rc::new(RefCell::new(ObjectImpl::new(
            self.objects.len() as u32,
            bounding_box,
        ))));
        Rc::clone(self.objects.last().unwrap())
    }

    pub fn remove_object(&mut self, object_id: u32) {
        todo!();
    }
}
