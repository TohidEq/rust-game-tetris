use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand, QueueableCommand,
};
use std::{
    f32::consts::E,
    io::{self, stdout, Stdout, Write},
};
use std::{thread, time};

use crossterm::style::Stylize;

const GAME_SPEED: u16 = 60;
const PLAYGROUND_WITH: u16 = 10;
const PLAYGROUND_HEIGHT: u16 = 20;
const MARGIN_LEFT: u16 = 10;
const MARGIN_TOP: u16 = 2;
const BORDER: u16 = 1;

enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Purple,
    Cyan,
    White,
}

struct Location {
    x: u16,
    y: u16,
}
struct Color {
    fg_color: Colors,
    bg_color: Colors,
}
struct Cell {
    location: Location,
    fill: bool,
    color: Color,
}

struct Playground {
    width: u16,
    height: u16,
    cells: Vec<Cell>, // [[Cell; PLAYGROUND_WITH as usize]; PLAYGROUND_HEIGHT as usize],
}

fn draw_cell(cell: &Cell, sc: &mut Stdout) {
    let mut text: crossterm::style::StyledContent<&str> = " ".white().on_black();

    match cell.color.fg_color {
        Colors::Black => {
            text = text.black();
        }
        Colors::Red => {
            text = text.red();
        }
        Colors::Green => {
            text = text.green();
        }
        Colors::Yellow => {
            text = text.yellow();
        }
        Colors::Blue => {
            text = text.blue();
        }
        Colors::Magenta => {
            text = text.magenta();
        }
        Colors::Purple => {
            text = text.black();
        }
        Colors::Cyan => {
            text = text.cyan();
        }
        Colors::White => {
            text = text.white();
        }
    }

    match cell.color.bg_color {
        Colors::Black => {
            text = text.on_black();
        }
        Colors::Red => {
            text = text.on_red();
        }
        Colors::Green => {
            text = text.on_green();
        }
        Colors::Yellow => {
            text = text.on_yellow();
        }
        Colors::Blue => {
            text = text.on_blue();
        }
        Colors::Magenta => {
            text = text.on_magenta();
        }
        Colors::Purple => {
            text = text.on_black();
        }
        Colors::Cyan => {
            text = text.on_cyan();
        }
        Colors::White => {
            text = text.on_white();
        }
    }
    let _x = cell.location.x * 2 + MARGIN_LEFT;
    let _y = cell.location.y + MARGIN_TOP;

    sc.queue(MoveTo(_x, _y))
        .unwrap()
        .queue(Print(text))
        .unwrap()
        .queue(MoveTo(_x + 1, _y))
        .unwrap()
        .queue(Print(text))
        .unwrap();
}

fn create_playground(playground: &mut Playground) {
    for x in 0..PLAYGROUND_WITH {
        for y in 0..PLAYGROUND_HEIGHT {
            let fill = if x < BORDER
                || x >= PLAYGROUND_WITH - BORDER
                || y < BORDER
                || y >= PLAYGROUND_HEIGHT - BORDER
            {
                true
            } else {
                false
            };
            playground.cells.push(Cell {
                location: Location { x, y },
                fill: fill,
                color: Color {
                    fg_color: Colors::Black,
                    bg_color: Colors::Black,
                },
            })
        }
    }
}

fn draw_playground(playground: &mut Playground, sc: &mut Stdout) {
    for cell in &playground.cells {
        if cell.fill == true {
            draw_cell(cell, sc);
        }
    }
}

fn main() -> io::Result<()> {
    // init the screen
    let mut sc = stdout();
    let (max_x_fake, max_y_fake) = size()?;
    let (max_x, max_y): (u16, u16) = (max_x_fake / 2 - 1, max_y_fake - 2);

    let mut playground = Playground {
        width: max_x,
        height: max_y,
        cells: vec![],
    };

    sc.execute(Hide)?;
    enable_raw_mode()?;

    // init the world
    create_playground(&mut playground);
    draw_playground(&mut playground, &mut sc);
    sc.flush()?;

    // Main game loop
    // - Eventsfg
    // - Physics
    // - Drawing
    // ====

    let millis = time::Duration::from_millis(1000);
    thread::sleep(millis);

    // game is finished

    sc.execute(terminal::Clear(terminal::ClearType::All))?
        .execute(Show)?;
    disable_raw_mode()?;

    Ok(())
}
