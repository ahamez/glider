extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub mod glider;
use glider::dense_grid::DenseGrid;
use glider::rle::{Rle, RleEntry};
use glider::universe::Universe;


use std::time::Duration;

/* --------------------------------------------------------------------------------------------- */

fn main() {

  let rle = Rle {
    pattern: vec![
      RleEntry::Live(3), 
      RleEntry::NewRow,
      RleEntry::Dead(2),
      RleEntry::Live(1),
      RleEntry::NewRow,
      RleEntry::Dead(1),
      RleEntry::Live(1),
    ]
  };

  // let rle = Rle {
  //   pattern: vec![
  //     RleEntry::Live(3), 
  //     RleEntry::NewRow,
  //     RleEntry::Live(1),
  //     RleEntry::Dead(1),
  //     RleEntry::Live(1),
  //     RleEntry::NewRow,
  //     RleEntry::Live(3),
  //   ]
  // };

  let grid_rows = 100;
  let grid_cols = 100;

  let window_rows = 1000;
  let window_cols = 1000;

  let cell_size = 10u32;

  let background_color = Color::RGB(0, 0, 0);
  let cell_color = Color::RGB(255, 255, 255);

  let grid = DenseGrid::new_from_rle(&rle, grid_rows, grid_cols);
  let mut u = Universe::new(grid);

  let sdl_context = sdl2::init()
    .unwrap();

  let video_subsystem = sdl_context
    .video()
    .unwrap();

  let window = video_subsystem
    .window("Glider", window_rows, window_cols)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  let mut canvas = window
    .into_canvas()
    .accelerated()
    .build()
    .unwrap();

  canvas.set_draw_color(background_color);
  canvas.clear();
  canvas.present();
  
  let mut event_pump = sdl_context
    .event_pump()
    .unwrap();

  'running: loop {
      
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        }
        _ => {}
      }
    }
      
    u = u.tick();
    // println!("gen: {}, live: {}", u.generation(), u.live_cells());

    canvas.set_draw_color(background_color);
    canvas.clear();

    canvas.set_draw_color(cell_color);

    for row in 0 .. grid_rows {
      for col in 0 .. grid_cols {
        if u.at(row, col) {
          let _ = canvas.fill_rect(
            Rect::new(
              col as i32 * cell_size as i32,
              row as i32 * cell_size as i32,
              cell_size,
              cell_size));
        }
      }
    }

    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    canvas.present();
  }
}

/* --------------------------------------------------------------------------------------------- */
