use embedded_graphics::egline;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::DisplayBuilder;
use embedded_graphics_simulator::RgbDisplay;
use integer_sqrt::IntegerSquareRoot;
use std::thread;
use std::time::Duration;

fn draw_perp2(
    display: &mut RgbDisplay,
    p_start: Point,
    delta: Point,
    direction: Point,
    initial_error: i32,
    width: i32,
    color: Rgb888,
) {
    let mut p = p_start;
    let mut error = initial_error;

    let old_len = ((delta.x.pow(2) + delta.y.pow(2)) as f32).sqrt() as i32;
    let len = (delta.x.pow(2) + delta.y.pow(2)).integer_sqrt();
    let width = delta.x.abs().max(delta.y.abs()) * width;

    assert_eq!(old_len, len);

    for _ in (0..width).step_by(len as usize) {
        display.set_pixel(p.x as usize, p.y as usize, color);

        let e_double = error * 2;

        if e_double > delta.y {
            error += delta.y;

            p -= Point::new(0, direction.y);
        }

        if e_double < delta.x {
            error += delta.x;

            p += Point::new(direction.x, 0);
        }
    }
}

fn draw_line(display: &mut RgbDisplay, p0: Point, p1: Point, width: i32) {
    let mut delta = p1 - p0;

    if delta.x < 0 {
        delta = Point::new(-delta.x, delta.y);
    }
    if delta.y > 0 {
        delta = Point::new(delta.x, -delta.y);
    }

    let direction = match (p0.x >= p1.x, p0.y >= p1.y) {
        (false, false) => Point::new(1, 1),
        (false, true) => Point::new(1, -1),
        (true, false) => Point::new(-1, 1),
        (true, true) => Point::new(-1, -1),
    };

    let mut error = delta.x + delta.y;

    let mut p = p0;

    while p != p1 {
        draw_perp2(display, p, delta, direction, error, width, Rgb888::YELLOW);

        display.set_pixel(p.x as usize, p.y as usize, Rgb888::WHITE);

        let e_double = error * 2;

        if e_double > delta.y {
            draw_perp2(
                display,
                p,
                delta,
                direction,
                error + delta.y,
                width,
                Rgb888::YELLOW,
            );
            error += delta.y;
            p += Point::new(direction.x, 0);
        }

        if e_double < delta.x {
            error += delta.x;
            p += Point::new(0, direction.y);
        }
    }

    // Draw center line using existing e-g `Line`
    display.draw(egline!(p0, p1, stroke_color = Some(Rgb888::WHITE)));
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Delete me and update 'strokes' demo")
        .size(256, 256)
        .scale(3)
        .build_rgb();

    // draw_line(&mut display, 20, 20, 100, 50, 1);

    // draw_line(&mut display, 10, 100, 50, 100, 1);

    let mut angle: f32 = 0.0;

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        display.clear();

        let x = 127 + (angle.cos() * 120.0) as i32;
        let y = 127 + (angle.sin() * 120.0) as i32;

        draw_line(&mut display, Point::new(127, 127), Point::new(x, y), 10);

        angle += 0.1;

        thread::sleep(Duration::from_millis(50));
    }
}
