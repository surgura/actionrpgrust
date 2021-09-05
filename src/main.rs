mod player;
pub use player::*;

mod input;
pub use input::Input;

mod color;
pub use color::Color;

mod render_target;
pub use render_target::RenderTarget;

mod render_target_3dsprites;
pub use render_target_3dsprites::RenderTarget3DSprites;

mod shader;
pub use shader::Shader;

mod target_dummy;
pub use target_dummy::TargetDummy;

mod physics;

use nalgebra::{Vector2, Vector3};

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    rl.set_target_fps(60);

    let mut shader2d3d = Shader {
        raylib_shader: rl
            .load_shader(&thread, None, Some("./shader3d2d.frag"))
            .unwrap(),
    };

    let mut phys = physics::Environment::new();
    let mut object1 = phys.new_object(physics::BoundingBox::new(16.0, 16.0, 1.0));
    object1.set_position(Vector3::new(-300.0, 0.0, 0.0));
    object1.set_velocity(Vector3::new(1.0, 0.0, 0.0));
    let object2 = phys.new_object(physics::BoundingBox::new(16.0, 16.0, 1.0));

    let mut player = Player::new();

    let target_dummy = TargetDummy::new();

    while !rl.window_should_close() {
        let input = Input::new(&rl);
        player.update(&input);

        phys.step();

        let mut draw_handle = rl.begin_drawing(&thread);
        let mut target = RenderTarget::new(&mut draw_handle);
        target.clear_background(Color::WHITE);

        let camera_pos: Vector2<f32> = Vector2::new(-320.0, 240.0);
        let mut sprites3d = RenderTarget3DSprites::new(&mut target, &mut shader2d3d, camera_pos);
        player.draw(&mut sprites3d);
        target_dummy.draw(&mut sprites3d);
        sprites3d.draw_circle(object1.get_position(), 100.0f32, Color::BLUE);
    }
}
