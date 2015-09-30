#[macro_use]
extern crate glium;
extern crate clock_ticks;

use glium::Surface;
use glium::glutin;

fn main() {
    use glium::DisplayBuild;

    let display = glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    let frames = 60 * 5;
    let trials = 3;

    for _ in 0.. trials {
        let start_ns = clock_ticks::precise_time_ns();

        let mut last = start_ns;
        for f in 0..frames {
            let before = last;
            display.draw().finish().unwrap();
            let after = clock_ticks::precise_time_ns();
            println!("Frame #{:3}: {:8}  {:10}", f, after - before, (after % 16_666_667));

            last = after;
        }

        let duration_ns = clock_ticks::precise_time_ns() - start_ns;
        let duration_s = (duration_ns as f64) / 1_000_000_000f64;
        let fps = (frames as f64) / duration_s;
        let dropped = (duration_s - (frames as f64 * (1f64/60f64))) / (1f64/60f64);

        println!("{} frames in {:.6} seconds = {:.3} fps (estimated {:.1} frames dropped)", frames, duration_s, fps, dropped);
    }
}

