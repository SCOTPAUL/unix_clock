extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use opengl_graphics::*;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::*;
use crate::graphics::Transformed;
use std::time::{Duration, SystemTime};

struct ClockFace {
    clock_circle_coords: [f64; 4]
}

impl ClockFace {
    fn new(window_size: &[f64; 2]) -> Self {
        ClockFace { clock_circle_coords: [0.0, 0.0, window_size[0], window_size[1]] }
    }

    fn draw(&self, window_size: &[f64; 2], c: &graphics::Context, gl: &mut GlGraphics, character_cache: &mut GlyphCache){

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let ellipse = graphics::Ellipse::new_border(RED, 5.0);
        ellipse.draw(self.clock_circle_coords, &c.draw_state, c.transform, gl);

        let centre_x = window_size[0] / 2.0;
        let centre_y = window_size[1] / 2.0;
        let radius = (window_size[0] / 2.0) - 20.0;

        for i in 0..10 {

            let angle_rad = (36.0 * i as f64).to_radians();

            let pos_x = centre_x + radius * angle_rad.sin();
            let pos_y = centre_y - radius * angle_rad.cos();

            let transform = c.transform.trans(pos_x, pos_y);

            graphics::text(RED, 20, &format!("{}", i), character_cache, transform, gl);            
        }
    }

}

struct ClockHand {
    rotation: f64,
    position: u8
}

impl ClockHand {
    fn draw(&self, window_size: &[f64; 2], c: &graphics::Context, gl: &mut GlGraphics){
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let value_at_posn = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string().chars().nth(self.position.into()).unwrap();

        let line = graphics::Line::new(RED, 2.5);

        let centre_x = window_size[0] / 2.0;
        let centre_y = window_size[1] / 2.0;
        let radius = self.position as f64 * 20.0 + 50.0;

        let angle_rad = (36.0 * value_at_posn.to_digit(10).unwrap() as f64).to_radians();

        let pos_x = centre_x + radius * angle_rad.sin();
        let pos_y = centre_y - radius * angle_rad.cos();

        line.draw_from_to([centre_x, centre_y], [pos_x, pos_y], &c.draw_state, c.transform, gl);//([0.0, 0.0, 5.0, 5.0], &c.draw_state, transform, gl);

    }
}

struct Clock {
    clock_face: ClockFace,
    clock_hands: Vec<ClockHand>
}

impl Clock {
    fn new(window_size: &[f64; 2]) -> Self {
        let mut clock_hands = vec![];

        for i in 0..10 {
            clock_hands.push(ClockHand { rotation: 0.0, position: i });
        }

        let clock_face = ClockFace::new(window_size);


        Clock { clock_face: clock_face, clock_hands: clock_hands }
    }

    fn draw(&self, window_size: &[f64; 2], c: &graphics::Context, gl: &mut GlGraphics, character_cache: &mut GlyphCache) {
        self.clock_face.draw(window_size, c, gl, character_cache);
        for hand in &self.clock_hands {
            hand.draw(window_size, c, gl);
        }
    }
}

pub struct App {
    gl: GlGraphics // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs, window: &Window) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let mut glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
        let clock = Clock::new(&args.window_size);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            clock.draw(&args.window_size, &c, gl, &mut glyph_cache);

            // Draw a box rotating around the middle of the screen.
            //rectangle(RED, square, transform, gl);
            //line(RED, 3.0, [0.0, 0.0, 15.0, 15.0], transform, gl);
            //line(RED, 5.0, [0.0, 0.0, -25.0, -25.0], transform, gl);

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("unix_clock", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl)
    };

    let mut event_settings = EventSettings::new();
    event_settings.max_fps = 5;
    event_settings.ups = 5;

    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &window);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}