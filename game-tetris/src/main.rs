use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand, QueueableCommand,
};
use rotation::Rotation;
use std::io::{self, stdout, Stdout, Write};
use std::{thread, time};
use tetrisblock::TetrisBlock;

use crossterm::style::Stylize;

mod config;

mod rotation;

mod colors;
mod playground;
mod tetrisblock;
use playground::{Color, Location, Playground};

fn main() -> io::Result<()> {
    // init the screen
    let mut sc = stdout();
    let (max_x_fake, max_y_fake) = size()?;
    let (max_x, max_y): (u16, u16) = (
        if (max_x_fake / 2) % 2 == 0 {
            max_x_fake / 2
        } else {
            max_x_fake / 2 - 1
        },
        max_y_fake,
    );

    let mut playground = Playground::new(max_x, max_y);
    sc.execute(Hide)?;
    enable_raw_mode()?;

    // init the world
    playground.create_playground();
    sc.execute(terminal::Clear(terminal::ClearType::All))?;

    // Main game loop
    // - Eventsfg
    // - Physics
    // - Drawing
    // ====

    // test get_index n get_cell
    let i = playground.get_index(10, 10);
    let x = playground.get_cell(i);
    let strdd = format!("one: {}, {}", x.location.x, x.location.y);

    sc.queue(MoveTo(0, 0))?.queue(Print(strdd))?;

    let mut my_tetris_block = TetrisBlock::random();
    let my_color = Color {
        fg_color: my_tetris_block.get_color(),
        bg_color: my_tetris_block.get_color(),
    };

    my_tetris_block.update_block(
        Rotation::Deg0,
        Location { x: 2, y: 2 },
        &mut playground,
        false,
    );

    playground.draw_playground(&mut sc);
    playground.draw_border(&mut sc);
    sc.flush()?;

    // ====
    let millis = time::Duration::from_millis(5000);
    thread::sleep(millis);

    // game is finished
    sc.execute(terminal::Clear(terminal::ClearType::All))?
        .execute(Show)?;
    disable_raw_mode()?;

    Ok(())
}
