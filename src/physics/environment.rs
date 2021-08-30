use super::bounding_box::BoundingBox;
use super::object::{Object, ObjectImpl};
use nalgebra::Vector3;
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
        let object = self.inner.borrow_mut().new_object(bounding_box);
        Object::new(Rc::clone(&self.inner), object)
    }

    pub fn get_collisions(&self, bounding_box: BoundingBox, position: Vector3<f32>) -> Vec<u32> {
        self.inner
            .borrow()
            .objects
            .iter()
            .filter(|object| {
                Self::collides(
                    bounding_box,
                    position,
                    object.borrow().bounding_box,
                    object.borrow().position,
                )
            })
            .map(|object| object.borrow().object_id)
            .collect()
    }

    pub fn collides(
        a_box: BoundingBox,
        a_position: Vector3<f32>,
        b_box: BoundingBox,
        b_position: Vector3<f32>,
    ) -> bool {
        return !(a_position[0] - a_box.x > b_position[0] + b_box.x
            || b_position[0] - b_box.x > a_position[0] + a_box.x
            || a_position[1] - a_box.y > b_position[1] + b_box.y
            || b_position[1] - b_box.y > a_position[1] + a_box.y
            || a_position[2] - a_box.z > b_position[2] + b_box.z
            || b_position[2] - b_box.z > a_position[2] + a_box.z);
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
