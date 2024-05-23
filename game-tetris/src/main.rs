mod colors;
mod config;
mod playground;
mod rotation;
mod tetrisblock;

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
    let mut playground_frame = config::GAME_SPEED;
    let mut status = Status::Run;
    let mut score: u16 = 0;
    let score_bg = ".....";
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
    //let i = playground.get_index(10, 10);
    //let x = playground.get_cell(i);
    //let strdd = format!("one: {}, {}", x.location.x, x.location.y);
    //sc.queue(MoveTo(0, 0))?.queue(Print(strdd))?;

    let mut my_tetris_block = TetrisBlock::random();

    let start_location = Location {
        x: max_x / 8 - 2,
        y: 0,
    };

    let mut block_location: Location = Location {
        x: start_location.x,
        y: start_location.y,
    };
    let mut block_location_new: Location = Location {
        x: block_location.x,
        y: block_location.y,
    };

    let mut block_rotation: Rotation = Rotation::Deg0;
    let mut block_rotation_new: Rotation = block_rotation;

    let mut block_life = config::BLOCK_LIFE;

    let mut my_next_tetris_block = TetrisBlock::random();
    let mut next_block_location: Location = Location {
        x: (playground.width + 4),
        y: 4,
    };

    playground.draw_side_bar(&mut sc);
    // sc.flush()?;
    // let my_color = Color {
    //     fg_color: my_tetris_block.get_color(),
    //     bg_color: my_tetris_block.get_color(),
    // };

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
        playground_frame -= 1;
        match status {
            Status::New => {}
            Status::Run => {
                // ----
                // -> first draw
                // insert into playground
                my_tetris_block.update_block(
                    &mut block_rotation,
                    &mut block_location,
                    &mut playground,
                    false,
                );
                playground.draw_playground(&mut sc);
                playground.draw_border(&mut sc);

                playground.draw_score(&mut sc, &mut score, &score_bg);

                sc.flush()?;

                // ----
                // -> events
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

                            // disabled :)
                            /*
                            KeyCode::Char(' ') => {
                                playground_frame = 1; // skip here
                                block_life = config::BLOCK_LIFE;
                                my_tetris_block.update_block(
                                    &mut block_rotation,
                                    &mut block_location,
                                    &mut playground,
                                    true,
                                );

                                for i in block_location.y..playground.height - 1 {
                                    if my_tetris_block.check_invalid(
                                        &mut block_rotation,
                                        &Location {
                                            x: block_location.x,
                                            y: i,
                                        },
                                        &mut playground,
                                        false,
                                    ) {
                                        block_location.y = i - 1;
                                        block_location_new.y = i - 0;
                                    }
                                }
                            }
                                    */
                            KeyCode::Char('w') => {
                                block_rotation_new = block_rotation.next();
                            }
                            KeyCode::Char('a') => {
                                if block_location_new.x > 0 {
                                    block_location_new.x -= 1;
                                }
                            }
                            KeyCode::Char('s') => {
                                playground_frame = config::GAME_SPEED; // turbo here

                                block_location_new.y += 1;
                            }
                            KeyCode::Char('d') => {
                                // az kadr nazane biroon
                                block_location_new.x += 1;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                } else {
                    // Timeout expired and no `Event` is available
                }

                // ----
                // -> physics
                if playground_frame == 0 {
                    // move down block
                    block_location_new.y += 1;
                }

                my_tetris_block.update_block(
                    &mut block_rotation,
                    &mut block_location,
                    &mut playground,
                    true,
                );

                let mut longest_row_size: u16 = 1;
                for i in 0..my_tetris_block.get_block(&block_rotation_new).len() {
                    if my_tetris_block.get_block(&block_rotation_new)[i].len() as u16
                        > longest_row_size
                    {
                        longest_row_size =
                            my_tetris_block.get_block(&block_rotation_new)[i].len() as u16
                    }
                }

                // tadakhol nadashte bashe
                if !my_tetris_block.check_invalid(
                    &mut block_rotation_new,
                    &mut block_location_new,
                    &mut playground,
                    false,
                ) &&
                // barkhord ba divar raast
                (playground.width
                    > block_location_new.x
                        + longest_row_size -1)
                {
                    block_location = Location {
                        x: block_location_new.x,
                        y: block_location_new.y,
                    };
                    block_rotation = block_rotation_new;
                } else {
                    if block_location_new.y != block_location.y {
                        block_life -= 1;
                    }
                    block_location_new = Location {
                        x: block_location.x,
                        y: block_location.y,
                    };
                    block_rotation_new = block_rotation;
                }

                my_tetris_block.update_block(
                    &mut block_rotation,
                    &mut block_location,
                    &mut playground,
                    false,
                );
                my_next_tetris_block.just_draw_block(
                    &Rotation::Deg0,
                    &mut next_block_location,
                    &mut playground,
                    &mut sc,
                    false,
                );

                // new block
                if block_life == 0 {
                    // check score here :D
                    for i in 0..4 {
                        // (0..4) range = longest block (I)

                        let row_for_move = playground.check_rows(&mut sc);
                        if row_for_move != 0 {
                            playground.move_rows(&mut sc, row_for_move);
                            if i > 0 {
                                score += (playground.width) * (i + 1);
                            } else {
                                score += playground.width;
                            }
                        }
                    }

                    block_life = config::BLOCK_LIFE;

                    my_next_tetris_block.just_draw_block(
                        &Rotation::Deg0,
                        &mut next_block_location,
                        &mut playground,
                        &mut sc,
                        true,
                    );

                    my_tetris_block = my_next_tetris_block;
                    while my_tetris_block.get_block(&Rotation::Deg0)
                        == my_next_tetris_block.get_block(&Rotation::Deg0)
                    {
                        my_next_tetris_block = TetrisBlock::random();
                    }

                    block_location = Location {
                        x: start_location.x,
                        y: start_location.y,
                    };

                    block_rotation = Rotation::Deg0;

                    block_location_new = Location {
                        x: block_location.x,
                        y: block_location.y,
                    };
                    block_rotation_new = block_rotation;

                    if !my_tetris_block.check_invalid(
                        &mut block_rotation,
                        &mut block_location,
                        &mut playground,
                        false,
                    ) {
                        my_tetris_block.update_block(
                            &mut block_rotation,
                            &mut block_location,
                            &mut playground,
                            false,
                        );
                    } else {
                        status = Status::End;
                    }
                }
                // ---- end physics

                // ----
                // -> drawing

                playground.draw_playground(&mut sc);
                playground.draw_border(&mut sc);
                sc.flush()?;
                // ----
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
        if playground_frame == 0 {
            playground_frame = config::GAME_SPEED
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
