use crate::{
    colors::Colors,
    config,
    playground::{self, Cell, Color, Location, Playground},
    rotation::Rotation,
};
use rand::{random, Rng};
use std::io::{self, stdout, Stdout, Write};

#[derive(Debug, Copy, Clone)]
pub enum TetrisBlock {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl TetrisBlock {
    pub fn random() -> TetrisBlock {
        match rand::thread_rng().gen_range(0..7) {
            0 => return Self::O,
            1 => return Self::I,
            2 => return Self::T,
            3 => return Self::L,
            4 => return Self::J,
            5 => return Self::S,
            6 => return Self::Z,
            _ => Self::random(),
        }
    }

    pub fn get_block<'a>(&mut self, rotation: Rotation) -> Vec<Vec<&'a str>> {
        match self {
            TetrisBlock::I => return Self::block_i(rotation),
            TetrisBlock::J => return Self::block_j(rotation),
            TetrisBlock::L => return Self::block_l(rotation),
            TetrisBlock::O => return Self::block_o(rotation),
            TetrisBlock::S => return Self::block_s(rotation),
            TetrisBlock::T => return Self::block_t(rotation),
            TetrisBlock::Z => return Self::block_z(rotation),
        }
    }

    pub fn get_color(&mut self) -> Colors {
        match self {
            TetrisBlock::O => return Colors::Yellow,
            TetrisBlock::I => return Colors::Cyan,
            TetrisBlock::T => return Colors::Purple,
            TetrisBlock::L => return Colors::Magenta, // change this(L) color pls :D
            TetrisBlock::J => return Colors::Blue,
            TetrisBlock::S => return Colors::Green,
            TetrisBlock::Z => return Colors::Red,
        }
    }

    fn block_o<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Deg0 | Rotation::Deg90 | Rotation::Deg180 | Rotation::Deg270 => {
                return vec![vec!["X", "X"], vec!["X", "X"]];
            }
        }
    }
    fn block_i<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Deg0 | Rotation::Deg180 => {
                return vec![vec!["X"], vec!["X"], vec!["X"], vec!["X"]];
            }
            Rotation::Deg90 | Rotation::Deg270 => {
                return vec![vec!["X", "X", "X", "X"]];
            }
        }
    }

    fn block_t<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Deg0 => {
                return vec![vec![" ", "X", " "], vec!["X", "X", "X"]];
            }
            Rotation::Deg90 => {
                return vec![vec!["X"], vec!["X", "X"], vec!["X"]];
            }
            Rotation::Deg180 => {
                return vec![vec!["X", "X", "X"], vec![" ", "X", " "]];
            }
            Rotation::Deg270 => {
                return vec![vec![" ", "X"], vec!["X", "X"], vec![" ", "X"]];
            }
        }
    }

    fn block_l<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Deg0 => {
                return vec![vec!["X"], vec!["X"], vec!["X", "X"]];
            }
            Rotation::Deg90 => {
                return vec![vec!["X", "X", "X"], vec!["X"]];
            }
            Rotation::Deg180 => {
                return vec![vec!["X", "X"], vec![" ", "X"], vec![" ", "X"]];
            }
            Rotation::Deg270 => {
                return vec![vec![" ", " ", "X"], vec!["X", "X", "X"]];
            }
        }
    }

    fn block_j<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Deg0 => {
                return vec![vec![" ", "X"], vec![" ", "X"], vec!["X", "X"]];
            }
            Rotation::Deg90 => {
                return vec![vec!["X"], vec!["X", "X", "X"]];
            }
            Rotation::Deg180 => {
                return vec![vec!["X", "X"], vec!["X"], vec!["X"]];
            }
            Rotation::Deg270 => {
                return vec![vec!["X", "X", "X"], vec![" ", " ", "X"]];
            }
        }
    }

    fn block_s<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Deg0 | Rotation::Deg180 => {
                return vec![vec![" ", "X", "X"], vec!["X", "X"]];
            }
            Rotation::Deg90 | Rotation::Deg270 => {
                return vec![vec!["X", " "], vec!["X", "X"], vec![" ", "X"]];
            }
        }
    }

    fn block_z<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Deg0 | Rotation::Deg180 => {
                return vec![vec!["X", "X"], vec![" ", "X", "X"]];
            }
            Rotation::Deg90 | Rotation::Deg270 => {
                return vec![vec![" ", "X"], vec!["X", "X"], vec!["X", " "]];
            }
        }
    }

    pub fn check_invalid(
        &mut self,
        // block_type: TetrisBlock,
        rotation: Rotation,
        location: Location,
        playground: &mut Playground,
        delete: bool, // true -> delete from screen to adding new one
    ) -> bool {
        let blocks = self.get_block(rotation);

        if
        // barkhord be (max y) zamin
        (location.y + blocks.len() as u16 - 1) >= playground.height
        || // biron az (max x) haashie
        (location.x + blocks[0].len() as u16 -1 ) > playground.width
        {
            return true;
        }

        // barkhorb ba block haye to playground
        for y in 0..blocks.len() {
            for x in 0..blocks[y].len() {
                if blocks[y][x] == "X" {
                    let index = playground.get_index(location.x + x as u16, location.y + y as u16);
                    if playground.cells[index].fill {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn update_block(
        &mut self,
        // block_type: TetrisBlock,
        rotation: Rotation,
        location: Location,
        playground: &mut Playground,
        delete: bool, // true -> delete from screen to adding new one
    ) {
        let blocks = self.get_block(rotation);
        for y in 0..blocks.len() {
            for x in 0..blocks[y].len() {
                if blocks[y][x] == "X" {
                    // let index = playground.get_index(location.x + x as u16, location.y + y as u16);

                    let new_cell = Cell {
                        text: if delete {
                            String::from(" ")
                        } else {
                            String::from(config::BLOCK_CHAR)
                        },
                        location: Location {
                            x: location.x + x as u16,
                            y: location.y + y as u16,
                        },
                        fill: !delete,
                        color: if !delete {
                            Color {
                                fg_color: self.get_color(),
                                bg_color: self.get_color(),
                            }
                        } else {
                            Color {
                                fg_color: config::PLAYGROUND_COLOR_FG,
                                bg_color: config::PLAYGROUND_COLOR_BG,
                            }
                        },
                    };
                    playground.update_cell(new_cell);
                }
            }
        }
    }
}
