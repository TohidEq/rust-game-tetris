use crossterm::ExecutableCommand;
use crossterm::{cursor::MoveTo, style::Print, QueueableCommand};
use std::io::Stdout;
use std::io::Write;

use crossterm::style::Stylize;

use crate::colors;
use crate::config;
use colors::Colors;

// #[derive(Debug)]
pub struct Location {
    pub x: u16,
    pub y: u16,
}
pub struct Color {
    pub(crate) fg_color: Colors,
    pub(crate) bg_color: Colors,
}
pub struct Cell {
    pub(crate) text: String,
    pub(crate) location: Location,
    pub(crate) fill: bool,
    pub(crate) color: Color,
}
pub struct Playground {
    pub screen_width: u16,
    pub screen_height: u16,
    pub width: u16,
    pub height: u16,

    pub cells: Vec<Cell>, // [[Cell; PLAYGROUND_WITH as usize]; PLAYGROUND_HEIGHT as usize],
}

impl Playground {
    pub fn new(max_x: u16, max_y: u16) -> Playground {
        return Playground {
            screen_width: max_x,
            screen_height: max_y,
            width: 0,
            height: 0,
            cells: vec![],
        };
    }
    pub fn create_playground(&mut self) {
        let _x = self.screen_width / 2 * config::PLAYGROUND_WITH / 100 - 2;
        let _y = self.screen_height * config::PLAYGROUND_HEIGHT / 100 - 2;
        self.width = _x;
        self.height = _y;

        for y in 0.._y {
            for x in 0.._x {
                let fill = false;
                self.cells.push(Cell {
                    text: String::from(" "),
                    location: Location { x, y },
                    fill: fill,
                    color: Color {
                        fg_color: config::PLAYGROUND_COLOR_FG,
                        bg_color: config::PLAYGROUND_COLOR_BG,
                    },
                })
            }
        }
    }

    pub fn get_index(&mut self, _x: u16, _y: u16) -> usize {
        let index = _y * (self.width) + _x;
        return index as usize;
    }

    pub fn get_cell(&mut self, index: usize) -> &Cell {
        return &self.cells[index];
    }

    pub fn update_cell(&mut self, cell: Cell) {
        let x = cell.location.x;
        let y = cell.location.y;
        let index = self.get_index(x, y);

        self.cells[index] = cell;
    }

    pub fn draw_border(&mut self, sc: &mut Stdout) {
        let mut new_cell = Cell {
            text: String::from(config::BORDER_CHAR),
            location: Location { x: 0, y: 0 },
            fill: false,
            color: Color {
                fg_color: config::BORDER_COLOR_FG,
                bg_color: config::BORDER_COLOR_BG,
            },
        };

        for y in 0..&self.height + 1 {
            new_cell.location = Location { x: 0, y: y + 1 };
            Playground::draw_cell(&new_cell, sc);

            new_cell.location = Location {
                x: &self.width + 1,
                y: y + 1,
            };
            Playground::draw_cell(&new_cell, sc);
        }

        for x in 0..&self.width + 0 {
            new_cell.location = Location {
                x: x + 1,
                y: &self.height + 1,
            };
            Playground::draw_cell(&new_cell, sc);
        }
    }

    pub fn draw_playground(&mut self, sc: &mut Stdout) {
        for cell in &self.cells {
            let new_cell = Cell {
                text: String::from(&cell.text),
                location: Location {
                    x: cell.location.x + 1,
                    y: cell.location.y + 1,
                },
                fill: cell.fill,
                color: Color {
                    fg_color: cell.color.fg_color,
                    bg_color: cell.color.bg_color,
                },
            };
            Playground::draw_cell(&new_cell, sc)
        }
    }

    pub fn draw_score(&mut self, sc: &mut Stdout, score: &mut u16, score_bg: &str) {
        let x = (self.width * 2) + 8;
        let mut y = 11; // ▼
        let mut text = String::from(score_bg.clone()).white();

        text = Colors::fg_color(&config::BORDER_COLOR_BG, text);
        text = Colors::bg_color(&config::PLAYGROUND_COLOR_BG, text);

        sc.queue(MoveTo(x, y + 1))
            .unwrap()
            .queue(Print(text.bold()))
            .unwrap();

        text = String::from(format!("{}", score)).white();

        text = Colors::fg_color(&config::BORDER_COLOR_BG, text);
        text = Colors::bg_color(&config::PLAYGROUND_COLOR_BG, text);

        sc.queue(MoveTo(x, y + 1))
            .unwrap()
            .queue(Print(text.bold()))
            .unwrap();
    }

    pub fn draw_side_bar(&mut self, sc: &mut Stdout) {
        for y in (1)..(self.screen_height) {
            for x in (self.width + 3)..(self.screen_width) {
                let new_cell = Cell {
                    text: String::from(" "),
                    location: Location { x: x, y: y },
                    fill: false,
                    color: Color {
                        fg_color: config::PLAYGROUND_COLOR_FG,
                        bg_color: config::PLAYGROUND_COLOR_BG,
                    },
                };
                Playground::draw_cell(&new_cell, sc);
            }
        }

        let x = (self.width * 2) + 8;
        let mut y = 2; // ▼
        let mut text = String::from("Next").white();

        text = Colors::fg_color(&config::BORDER_COLOR_BG, text);
        text = Colors::bg_color(&config::PLAYGROUND_COLOR_BG, text);

        sc.queue(MoveTo(x, y))
            .unwrap()
            .queue(Print(text.bold()))
            .unwrap();

        text = String::from("  ▼  ").white();

        text = Colors::fg_color(&config::BORDER_COLOR_BG, text);
        text = Colors::bg_color(&config::PLAYGROUND_COLOR_BG, text);

        sc.queue(MoveTo(x, y + 1))
            .unwrap()
            .queue(Print(text.bold()))
            .unwrap();

        y = 10;
        text = String::from("Score").white();

        text = Colors::fg_color(&config::BORDER_COLOR_BG, text);
        text = Colors::bg_color(&config::PLAYGROUND_COLOR_BG, text);

        sc.queue(MoveTo(x, y))
            .unwrap()
            .queue(Print(text.bold()))
            .unwrap();

        text = String::from("  ▼  ").white();

        text = Colors::fg_color(&config::BORDER_COLOR_BG, text);
        text = Colors::bg_color(&config::PLAYGROUND_COLOR_BG, text);

        sc.queue(MoveTo(x, y + 1))
            .unwrap()
            .queue(Print(text.bold()))
            .unwrap();

        y = 16;
        text = String::from("\\ (^_^) /").white();

        text = Colors::fg_color(&config::BORDER_COLOR_BG, text);
        text = Colors::bg_color(&config::PLAYGROUND_COLOR_BG, text);

        sc.queue(MoveTo(x, y))
            .unwrap()
            .queue(Print(text.bold()))
            .unwrap();

        // text = String::from("  ▼  ").white();

        // text = Colors::fg_color(&config::BORDER_COLOR_BG, text);
        // text = Colors::bg_color(&config::PLAYGROUND_COLOR_BG, text);

        // sc.queue(MoveTo(x, y + 1))
        //     .unwrap()
        //     .queue(Print(text.bold()))
        //     .unwrap();
    }

    pub fn draw_cell(cell: &Cell, sc: &mut Stdout) {
        let mut text = String::from(&cell.text).white().on_black();

        //
        text = Colors::fg_color(&cell.color.fg_color, text);
        text = Colors::bg_color(&cell.color.bg_color, text);
        //

        let _x = cell.location.x * 2 + config::MARGIN_LEFT;
        let _y = cell.location.y + config::MARGIN_TOP;

        sc.queue(MoveTo(_x, _y))
            .unwrap()
            .queue(Print(text.clone()))
            .unwrap()
            .queue(MoveTo(_x + 1, _y))
            .unwrap()
            .queue(Print(text.clone()))
            .unwrap();
    }

    pub fn move_rows(&mut self, sc: &mut Stdout, row: u16) {
        //             1..5
        for y in (1..row + 1).rev() {
            for x in (0..self.width) {
                let index = self.get_index(x, y - 1);
                let cell = self.get_cell(index);

                let new_cell = Cell {
                    text: cell.text.clone(),
                    location: Location { x: x, y: y },
                    fill: cell.fill,
                    color: Color {
                        fg_color: (cell.color.fg_color),
                        bg_color: (cell.color.bg_color),
                    },
                };

                self.update_cell(new_cell);
            }
        }
        // sc.queue(MoveTo(0, 0))
        //     .unwrap()
        //     .queue(Print(format!("two: {}", row)))
        //     .unwrap();
    }

    pub fn check_rows(&mut self, sc: &mut Stdout) -> u16 {
        for y in (1..self.height as u16).rev() {
            let mut is_fill_all: bool = true;

            for x in (0..self.width) {
                let index = self.get_index(x, y);
                let cell = self.get_cell(index);
                if cell.fill == false {
                    is_fill_all = false;
                }
            }
            if is_fill_all {
                return y;
            }
        }

        return 0;
    }
}
