use std::{thread, time};

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const WHITE: [f32; 4] = [0.97, 0.97, 0.97, 1.0];
const BLUE: [f32; 4] = [0.08, 0.12, 0.92, 1.0];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    width: u32,
    height: u32,
    sim: conway::Grid,
    color_list: Vec<Vec<[f32;4]>>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        for i in 0..self.width{
            for j in 0..self.height{
                let color = self.color_list[i as usize][j as usize];

                self.gl.draw(args.viewport(), |c, g| {
                    let (x, y) = (20*i,20*j);
                    let square = rectangle::square(x as f64, y as f64, 20.0); // x, y, size
                    rectangle(color, square, c.transform, g);
                });
            }
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        thread::sleep(time::Duration::from_millis(100));

        self.sim.evolve();

        let state = self.sim.refstate.borrow();
        let mut row: Vec<u8> = vec![0; self.sim.width as usize];

        for i in 0..self.sim.height {
            for (j, col) in row.iter_mut().enumerate() {
                let idx = calc_idx_periodic(j as u32, i,
                                            self.sim.width,
                                            self.sim.height
                                           );

                *col = state[idx];

                // Fill color list with correct colors.
                if *col == 1 {
                    self.color_list[i as usize][j] = BLUE;
                } else {
                    self.color_list[i as usize][j] = WHITE;
                }
            }
        }
    }
}

mod conway;

pub fn calc_idx_periodic(hor_idx: u32, vert_idx: u32,
                     width: u32, height: u32) -> usize {
    ((hor_idx % width) + width * (vert_idx % height)) as usize
}

fn main() {

    let width = 40;
    let height = 40;
    println!("Initializing simulation");
    let sim = conway::Grid::new(width, height);
    println!("Sucessfully initialized");

    sim.reset();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut vec: Vec<Vec<[f32;4]>> = Vec::with_capacity(width as usize * height as usize);
    for i in 0..width{
        vec.push(vec!());
        for j in 0..height{
            vec[i as usize].push(WHITE);
        }
    }

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        width: width,
        height: height,
        sim: sim,
        color_list: vec,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }

    /*
    // Glider 5 by 5
    let state = vec![0, 0, 0, 0, 0,
                     0, 1, 0, 0, 0,
                     0, 1, 0, 1, 0,
                     0, 1, 1, 0, 0,
                     0, 0, 0, 0, 0];
    */
    /*
    // Decapentathlon (period 15) 11 by 18
    // people who don't know Greek call it Penta-decathlon
    let state = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                     0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    */
    /*
    // Pulsar 17 by 17
    let state = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0,
                     0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0,
                     0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0,
                     0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                     0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0,
                     0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0,
                     0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    */
}
