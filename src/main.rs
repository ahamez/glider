use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

#[macro_use]
extern crate clap;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub mod glider;
use glider::dense_grid::DenseGrid;
use glider::rle::Rle;
use glider::universe::Universe;

/* --------------------------------------------------------------------------------------------- */

fn main() {

  let matches = clap_app!(myapp =>
    (@arg RLE_FILE: +required "Sets the input file to use")
  ).get_matches();

  println!("Using input file: {}", matches.value_of("RLE_FILE").unwrap());

  let file = File::open(matches.value_of("RLE_FILE").unwrap()).unwrap();
  let rle = Rle::read(BufReader::new(file)).unwrap();

  let grid_rows = 1000;
  let grid_cols = 1000;

  let window_rows = 1000;
  let window_cols = 1000;

  let cell_size = 1u32;

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
