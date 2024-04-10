use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, stdout, Stdout, Write};
use std::{thread, time};

use crossterm::style::Stylize;

mod config;

mod rotation;

mod colors;
mod playground;
use playground::Playground;

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
    playground.draw_playground(&mut sc);
    playground.draw_border(&mut sc);

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

    sc.flush()?;

    // ====
    let millis = time::Duration::from_millis(1000);
    thread::sleep(millis);

    // game is finished
    sc.execute(terminal::Clear(terminal::ClearType::All))?
        .execute(Show)?;
    disable_raw_mode()?;

    Ok(())
}
