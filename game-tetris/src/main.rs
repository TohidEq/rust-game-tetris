use crossterm::event::{poll, read, Event, KeyCode};
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

pub enum Status {
    New,
    Run,
    End,
    Pause,
    // Score,
    // Menu,
}

fn main() -> io::Result<()> {
    let mut status = Status::Run;
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

    // clear screen
    sc.execute(terminal::Clear(terminal::ClearType::All))?;

    // test get_index n get_cell
    let i = playground.get_index(10, 10);
    let x = playground.get_cell(i);
    let strdd = format!("one: {}, {}", x.location.x, x.location.y);

    sc.queue(MoveTo(0, 0))?.queue(Print(strdd))?;

    let mut my_tetris_block = TetrisBlock::random();
    // let my_color = Color {
    //     fg_color: my_tetris_block.get_color(),
    //     bg_color: my_tetris_block.get_color(),
    // };

    my_tetris_block.update_block(
        Rotation::Deg0,
        Location { x: 2, y: 2 },
        &mut playground,
        false,
    );

    /*
    my_tetris_block.update_block(
        Rotation::Deg0,
        Location { x: 2, y: 2 },
        &mut playground,
        true,
    );
     */
    // Main game loop
    // - Eventsfg
    // - Physics
    // - Drawing
    // ====
    loop {
        match status {
            Status::New => {}
            Status::Run => {
                // ----

                playground.draw_playground(&mut sc);
                playground.draw_border(&mut sc);
                sc.flush()?;
                // ----

                // read and apply keyboard
                // `poll()` waits for an `Event` for a given time period
                if poll(time::Duration::from_millis(10))? {
                    // It's guaranteed that the `read()` won't block when the `poll()`
                    // function returns `true`
                    let key = read()?;

                    // clear the buffer
                    while poll(time::Duration::from_millis(10))? {
                        let _ = read();
                    }

                    match key {
                        Event::Key(event) => match event.code {
                            KeyCode::Char('q') => {
                                status = Status::End;
                            }

                            KeyCode::Char(' ') => {}
                            KeyCode::Char('w') => {
                                // ----
                                my_tetris_block.update_block(
                                    Rotation::Deg0,
                                    Location { x: 2, y: 2 },
                                    &mut playground,
                                    true,
                                );
                                my_tetris_block = TetrisBlock::random();
                                my_tetris_block.update_block(
                                    Rotation::Deg0,
                                    Location { x: 2, y: 2 },
                                    &mut playground,
                                    false,
                                );
                                // ----
                            }
                            KeyCode::Char('a') => {}
                            KeyCode::Char('s') => {}
                            KeyCode::Char('d') => {}
                            _ => {}
                        },
                        _ => {}
                    }
                } else {
                    // Timeout expired and no `Event` is available
                }
            }
            Status::Pause => {}
            Status::End => {
                let millis = time::Duration::from_millis(800);
                thread::sleep(millis);

                // game is finished
                sc.execute(terminal::Clear(terminal::ClearType::All))?
                    .execute(Show)?;
                disable_raw_mode()?;
                break;
            }
        }
    }
    // ====

    Ok(())
}

/*
NOTE
hardafe ke update mishe:

draw
delete az arr
check invalid(update shodeye block)
invalid:
    block ghabli insert mishe
    next block generate mishe
    az aval
valid:
    block update
    block insert
    az aval


ez bere
 */
