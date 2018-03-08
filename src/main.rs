extern crate sdl2;

pub mod glider;
use glider::dense_grid::DenseGrid;
use glider::rle::{Rle, RleEntry};
use glider::universe::Universe;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use std::time::Duration;

/* --------------------------------------------------------------------------------------------- */

fn main() {

  // Glider pattern
  let rle = Rle {
    pattern: vec![
      RleEntry::Live(3), 
      RleEntry::NewLine,
      RleEntry::Dead(2),
      RleEntry::Live(1),
      RleEntry::NewLine,
      RleEntry::Dead(1),
      RleEntry::Live(1),
    ]
  };
  // let rle = Rle {
  //   pattern: vec![
  //     RleEntry::Live(3), 
  //     RleEntry::NewLine,
  //     RleEntry::Live(1),
  //     RleEntry::Dead(1),
  //     RleEntry::Live(1),
  //     RleEntry::NewLine,
  //     RleEntry::Live(3),
  //   ]
  // };

  let grid = DenseGrid::new_from_rle(&rle, 100, 100);
  let mut u = Universe::new(grid);

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem.window("Glider", 1000, 1000)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
      
      for event in event_pump.poll_iter() {
        match event {
          Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            break 'running
          },
          _ => {}
        }
      }
      
      u = u.tick();
      println!("gen: {}, live: {}", u.generation(), u.live_cells());

      canvas.set_draw_color(Color::RGB(0, 0, 0));
      canvas.clear();

      let cell_color = Color::RGB(255, 255, 255);
      canvas.set_draw_color(cell_color);

      for row in 0 .. 100 {
        for col in 0 .. 100 {
          if u.at(row, col) {
            let _ = canvas.fill_rect(Rect::new(col as i32 * 10, row as i32 * 10, 10, 10));
          }
        }
      }

      std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
      canvas.present();
    }
}

/* --------------------------------------------------------------------------------------------- */
