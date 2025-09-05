// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;

use line::line;
use maze::{Maze,load_maze};
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events};

use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

fn cell_to_color(cell: char) -> Color {
  match cell {
    '+' => {
      return Color::BLUEVIOLET;
    },
    '-' => {
      return Color::VIOLET;
    },
    '|' => {
      return Color::VIOLET;
    },
    'g' => {
      return Color::GREEN;
    },
    _ => {
      return Color::WHITE;
    },
  }
}

fn draw_cell(
  framebuffer: &mut Framebuffer,
  xo: usize,
  yo: usize,
  block_size: usize,
  cell: char,
) {
  if cell == ' ' {
    return;
  }
  let color = cell_to_color(cell);
  framebuffer.set_current_color(color);

  for x in xo..xo + block_size {
    for y in yo..yo + block_size {
      framebuffer.set_pixel(x as u32, y as u32);
    }
  }
}

pub fn render_maze(
  framebuffer: &mut Framebuffer,
  maze: &Maze,
  block_size: usize,
  player: &Player,
) {
  for (row_index, row) in maze.iter().enumerate() {
    for (col_index, &cell) in row.iter().enumerate() {
      let xo = col_index * block_size;
      let yo = row_index * block_size;
      draw_cell(framebuffer, xo, yo, block_size, cell);
    }
  }

  framebuffer.set_current_color(Color::WHITESMOKE);

  // draw what the player sees
  let num_rays = 5;
  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    cast_ray(framebuffer, &maze, &player, a, block_size, true);
  }
}

fn render_world(
  framebuffer: &mut Framebuffer,
  maze: &Maze,
  block_size: usize,
  player: &Player,
) {
  let num_rays = framebuffer.width;

  // let hw = framebuffer.width as f32 / 2.0;   // precalculated half width
  let hh = framebuffer.height as f32 / 2.0;  // precalculated half height

  framebuffer.set_current_color(Color::WHITESMOKE);

  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

    // Calculate the height of the stake
    let distance_to_wall = intersect.distance;// how far is this wall from the player
    let distance_to_projection_plane = 70.0; // how far is the "player" from the "camera"
    // this ratio doesn't really matter as long as it is a function of distance
    let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

    // Calculate the position to draw the stake
    let stake_top = (hh - (stake_height / 2.0)) as usize;
    let stake_bottom = (hh + (stake_height / 2.0)) as usize;

    // Draw the stake directly in the framebuffer
    for y in stake_top..stake_bottom {
      framebuffer.set_pixel(i, y as u32); // Assuming white color for the stake
    }
  }
}

fn main() {
  let window_width = 1300;
  let window_height = 900;
  let block_size = 100;

  let (mut window, raylib_thread) = raylib::init()
    .size(window_width, window_height)
    .title("Raycaster Example")
    .log_level(TraceLogLevel::LOG_WARNING)
    .build();

  let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
  framebuffer.set_background_color(Color::new(50, 50, 100, 255));

  let maze = load_maze("maze.txt");

  let mut player = Player {
    pos: Vector2::new(150.0, 150.0),
    a: PI / 3.0,
    fov: PI / 3.0,
  };

  while !window.window_should_close() {
    // 1. clear framebuffer
    framebuffer.clear();

    // 2. move the player on user input
    process_events(&mut player, &window);

    let mut mode = "3D";

    if window.is_key_down(KeyboardKey::KEY_M) {
      mode = if mode == "2D" { "3D" } else { "2D" };
    }

    // 3. draw stuff
    if mode == "2D" {
      render_maze(&mut framebuffer, &maze, block_size, &player);
    } else {
      render_world(&mut framebuffer, &maze, block_size, &player);
    }

    // 4. swap buffers
    framebuffer.swap_buffers(&mut window, &raylib_thread);

    thread::sleep(Duration::from_millis(16));
  }
}



