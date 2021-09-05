use super::bounding_box::BoundingBox;
use super::collision::Collision;
use super::object::{Object, ObjectImpl};
use nalgebra::Vector3;
use std::cell::RefCell;
use std::collections::HashSet;
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

    pub fn step(&mut self) -> Vec<Collision> {
        let mut inner = self.inner.borrow_mut();

        let mut should_move: Vec<bool> = Vec::new();
        let mut all_collisions: HashSet<Collision> = HashSet::new();

        for object1 in &inner.objects {
            let o1 = object1.borrow();
            let collisions: Vec<u32> = inner
                .objects
                .iter()
                .filter(|object2| {
                    o1.object_id != object2.borrow().object_id
                        && Self::collides(
                            o1.bounding_box,
                            o1.position + o1.velocity,
                            object2.borrow().bounding_box,
                            object2.borrow().position + object2.borrow().velocity,
                        )
                })
                .map(|object| object.borrow().object_id)
                .collect();
            if collisions.len() == 0 {
                should_move.push(true);
            } else {
                should_move.push(false);
                for object2_id in collisions {
                    all_collisions.insert(Collision::new(object1.borrow().object_id, object2_id));
                }
            }
        }

        for iter in inner.objects.iter_mut().zip(should_move.iter()) {
            let (object, do_move) = iter;
            if *do_move {
                let mut borrowed = object.borrow_mut();
                borrowed.position = borrowed.position + borrowed.velocity;
            }
        }

        return all_collisions.into_iter().collect();
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
