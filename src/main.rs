mod player;
pub use player::*;

mod input;
pub use input::Input;

mod color;
pub use color::Color;

mod render_target;
pub use render_target::RenderTarget;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    rl.set_target_fps(60);

    let mut player = Player::new();

    while !rl.window_should_close() {
        let input = Input::new(&rl);
        player.update(&input);

        let mut draw_handle = rl.begin_drawing(&thread);
        let mut target = RenderTarget::new(&mut draw_handle);

        target.clear_background(Color::WHITE);
        player.draw(&mut target);
    }
}
