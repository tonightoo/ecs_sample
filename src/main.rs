mod components;
mod systems;
use components::*;
use systems::*;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::thread;
use std::time::Duration;

#[derive(Default)]
struct Game {
    input: Input,
    position: Position,
    velocity: Velocity,
    character_view: CharacterView,
}

enum EventResult {
    Continue,
    End,
}

impl Game {
    fn new() -> Result<Game, String> {
        Ok(Self {
            position: Position { x: 50f64, y: 50f64 },
            ..Default::default()
        })
    }

    fn update(&mut self) -> Result<(), String> {
        update_velocity(&self.input, &mut self.velocity);
        update_position(&self.velocity, &mut self.position);
        update_character_view(&self.position, &mut self.character_view);
        Ok(())
    }

    fn event(&mut self, event: &sdl2::event::Event) -> Result<EventResult, String> {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return Ok(EventResult::End),
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                repeat: false,
                ..
            } => self.input.left = true,
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                repeat: false,
                ..
            } => self.input.right = true,
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => self.input.up = true,
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => self.input.down = true,
            Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            } => self.input.left = false,
            Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => self.input.right = false,
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => self.input.up = false,
            Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            } => self.input.down = false,
            _ => {}
        }

        return Ok(EventResult::Continue);
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        update_window(&self.character_view, canvas);
        canvas.present();
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("hello", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut game: Game = Game::new()?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        //Handle events
        for event in event_pump.poll_iter() {
            let result = game.event(&event);
            match result {
                Ok(_event_result) => match _event_result {
                    EventResult::Continue => {}
                    EventResult::End => break 'running,
                },
                Err(_) => break 'running,
            }
        }

        let _result = game.update()?;
        let _result = game.draw(&mut canvas)?;

        // Time management
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
