use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, stdout, Stdout, Write};
use std::{thread, time};

use crossterm::style::Stylize;

const GAME_SPEED: u16 = 60;

// (%)   100% W == half screen
const PLAYGROUND_WITH: u16 = 60;
// (%)   100% H == full screen
const PLAYGROUND_HEIGHT: u16 = 80;

const MARGIN_LEFT: u16 = 0;
const MARGIN_TOP: u16 = 0;

const BORDER_COLOR_FG: Colors = Colors::Black;
const BORDER_COLOR_BG: Colors = Colors::Yellow;
const BORDER_CHAR: &str = "🮪";
// border is = 1 :D idk how to make it dynamic
// const BORDER: u16 = 1;

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
    text: String,
    location: Location,
    fill: bool,
    color: Color,
}

struct Playground {
    screen_width: u16,
    screen_height: u16,
    width: u16,
    height: u16,

    cells: Vec<Cell>, // [[Cell; PLAYGROUND_WITH as usize]; PLAYGROUND_HEIGHT as usize],
}

fn draw_cell(cell: &Cell, sc: &mut Stdout) {
    let mut text = String::from(&cell.text).white().on_black();

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
        .queue(Print(text.clone()))
        .unwrap()
        .queue(MoveTo(_x + 1, _y))
        .unwrap()
        .queue(Print(text.clone()))
        .unwrap();
}

fn draw_border(playground: &mut Playground, sc: &mut Stdout) {
    let mut new_cell = Cell {
        text: String::from(BORDER_CHAR),
        location: Location { x: 0, y: 0 },
        fill: false,
        color: Color {
            fg_color: BORDER_COLOR_FG,
            bg_color: BORDER_COLOR_BG,
        },
    };

    for y in 0..playground.height + 1 {
        new_cell.location = Location { x: 0, y: y + 1 };
        draw_cell(&new_cell, sc);
    }
    for y in 0..playground.height + 1 {
        new_cell.location = Location {
            x: playground.width + 1,
            y: y + 1,
        };
        draw_cell(&new_cell, sc);
    }
    for x in 0..playground.width {
        new_cell.location = Location {
            x: x + 1,
            y: playground.height + 1,
        };
        draw_cell(&new_cell, sc);
    }
}

fn draw_playground(playground: &mut Playground, sc: &mut Stdout) {
    for cell in &playground.cells {
        if cell.fill == true {
            let new_cell = Cell {
                text: String::from(&cell.text),
                location: Location {
                    x: cell.location.x + 1,
                    y: cell.location.y + 1,
                },
                fill: cell.fill,
                color: Color {
                    fg_color: Colors::Black,
                    bg_color: Colors::Black,
                },
            };
            draw_cell(&new_cell, sc);
        }
    }
}

impl Playground {
    fn create_playground(&mut self) {
        let _x = self.screen_width / 2 * PLAYGROUND_WITH / 100 - 2;
        let _y = self.screen_height * PLAYGROUND_HEIGHT / 100 - 2;
        self.width = _x;
        self.height = _y;

        for y in 0.._y {
            for x in 0.._x {
                let fill = true;
                self.cells.push(Cell {
                    text: String::from(" "),
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

    fn get_index(&mut self, _x: u16, _y: u16) -> usize {
        let index = _y * (self.width) + _x;
        return index as usize;
    }

    fn get_cell(&mut self, index: usize) -> &Cell {
        return &self.cells[index];
    }
}

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

    let mut playground = Playground {
        screen_width: max_x,
        screen_height: max_y,
        width: 0,
        height: 0,
        cells: vec![],
    };

    sc.execute(Hide)?;
    enable_raw_mode()?;

    // init the world
    playground.create_playground();
    sc.execute(terminal::Clear(terminal::ClearType::All))?;
    draw_playground(&mut playground, &mut sc);
    draw_border(&mut playground, &mut sc);

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
