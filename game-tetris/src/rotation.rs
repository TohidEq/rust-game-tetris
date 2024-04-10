#[derive(Debug, Copy, Clone)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Rotation {
    pub fn next(&self) -> Rotation {
        match self {
            Rotation::Deg0 => Rotation::Deg90,
            Rotation::Deg90 => Rotation::Deg180,
            Rotation::Deg180 => Rotation::Deg270,
            Rotation::Deg270 => Rotation::Deg0,
        }
    }
}
