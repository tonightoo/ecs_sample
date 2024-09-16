extern crate sdl2;

use crate::components::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub(crate) fn update_velocity(input: &Input, velocity: &mut Velocity) {
    velocity.x = 0f64;
    velocity.y = 0f64;
    if input.left {
        velocity.x = -10f64;
    }
    if input.right {
        velocity.x = 10f64;
    }
    if input.up {
        velocity.y = -10f64;
    }
    if input.down {
        velocity.y = 10f64;
    }
}

pub(crate) fn update_position(velocity: &Velocity, position: &mut Position) {
    position.x += velocity.x;
    position.y += velocity.y;
}

pub(crate) fn update_character_view(position: &Position, view: &mut CharacterView) {
    view.position = position.clone();
    view.dir = 0f64;
    view.radius = 10f64;
}

pub(crate) fn update_window(view: &CharacterView, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    draw_circle(&view, canvas);
}

pub(crate) fn draw_circle(view: &CharacterView, canvas: &mut WindowCanvas) {
    for x in (-view.radius as i32)..(view.radius as i32) {
        let mut y: f64;
        y = (view.radius * view.radius) - (x as f64 * x as f64);
        y = y.sqrt();
        let screen_x = view.position.x + x as f64;
        let screen_y_plus = view.position.y + y as f64;
        //let _result = canvas.draw_point(Point::new(screen_x as i32, screen_y as i32));

        let screen_y_minus = view.position.y - y as f64;
        let src_point = Point::new(screen_x as i32, screen_y_plus as i32);
        let dst_point = Point::new(screen_x as i32, screen_y_minus as i32);

        let _result = canvas.draw_line(src_point, dst_point);
        //let _result = canvas.draw_point(Point::new(screen_x as i32, screen_y as i32));
    }

    //for y in (-view.radius as i32)..(view.radius as i32) {
    //    let mut x: f64;
    //    x = (view.radius * view.radius) - (y as f64 * y as f64);
    //    x = x.sqrt();
    //    let screen_x = view.position.x + x as f64;
    //    let screen_y = view.position.y + y as f64;
    //    let _result = canvas.draw_point(Point::new(screen_x as i32, screen_y as i32));
    //    let screen_x = view.position.x - x as f64;
    //    let _result = canvas.draw_point(Point::new(screen_x as i32, screen_y as i32));
    //}
}
