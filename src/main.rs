extern crate piston_window;

use piston_window::{self as pw, PistonWindow, WindowSettings};

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Boulder Dash", [640, 480])
            .exit_on_esc(true)
            .build()
            .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            pw::clear([1.0; 4], graphics);
            pw::rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );
        });
    }
}
