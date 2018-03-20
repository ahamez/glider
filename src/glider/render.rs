extern crate std;
use std::path::Path;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::grid::Grid;
use super::universe::Universe;

/* --------------------------------------------------------------------------------------------- */

enum State {
  Paused,
  Running,
}

/* --------------------------------------------------------------------------------------------- */

pub fn render_universe<G: Grid>(mut u: Universe<G>) {

  let window_rows = 1000;
  let window_cols = 1000;

  let cell_size = 1u32;

  let background_color = Color::RGB(0, 0, 0);
  let cell_color = Color::RGB(255, 255, 255);
  let font_color = Color::RGB(255, 255, 255);

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

  let ttf_context = sdl2::ttf::init().unwrap();
  let font_path = Path::new("./src/res/LeagueMono-Regular.ttf");
  let font = ttf_context.load_font(font_path, 12).unwrap();
  let texture_creator = canvas.texture_creator();

  canvas.set_draw_color(background_color);
  canvas.clear();
  canvas.present();

  let mut event_pump = sdl_context
    .event_pump()
    .unwrap();

  let mut state = State::Running;

  'running: loop {

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
          break 'running
        }

        Event::KeyDown {keycode: Some(Keycode::Space), repeat: false, ..} => {
          state = match state {
            State::Paused  => State::Running,
            State::Running => State::Paused,
          }
        }

        _ => {}
      }
    }

    if let State::Running = state {
      u = u.tick();
    }

    canvas.set_draw_color(background_color);
    canvas.clear();

    canvas.set_draw_color(cell_color);

    let surface = font.render(&format!("g:{} l:{}", u.generation, u.live_cells))
      .blended(font_color).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let sdl2::render::TextureQuery{width, height, ..} = texture.query();
    let target = Rect::new(10, 5, width, height);
    canvas.copy(&texture, None, Some(target)).unwrap();

    for row in 0 .. u.grid.nb_rows() {
      for col in 0 .. u.grid.nb_columns() {
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

    canvas.present();
  }
}

/* --------------------------------------------------------------------------------------------- */
