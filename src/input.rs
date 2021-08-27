use raylib::RaylibHandle;

pub type KeyboardKey = raylib::consts::KeyboardKey;

pub struct Input<'a> {
    handle: &'a RaylibHandle,
}

impl<'a> Input<'a> {
    pub fn new(handle: &RaylibHandle) -> Input {
        Input { handle: handle }
    }

    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        self.handle.is_key_down(key)
    }
}
