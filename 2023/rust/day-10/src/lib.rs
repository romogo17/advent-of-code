use glam::IVec2;
use nom_locate::LocatedSpan;

#[derive(Debug, Eq, PartialEq)]
pub enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    Ground,
}

impl PipeType {
    pub fn is_pipe_connection_valid(&self, other: &PipeType, direction: &Direction) -> bool {
        match (self, direction) {
            (PipeType::Vertical, Direction::North) => match other {
                PipeType::Vertical => true,
                PipeType::SouthEast => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::Vertical, Direction::South) => match other {
                PipeType::Vertical => true,
                PipeType::NorthEast => true,
                PipeType::NorthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::Horizontal, Direction::West) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthEast => true,
                PipeType::SouthEast => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::Horizontal, Direction::East) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthWest => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::NorthEast, Direction::North) => match other {
                PipeType::Vertical => true,
                PipeType::SouthEast => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::NorthEast, Direction::East) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthWest => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::NorthWest, Direction::North) => match other {
                PipeType::Vertical => true,
                PipeType::SouthEast => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::NorthWest, Direction::West) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthEast => true,
                PipeType::SouthEast => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::SouthEast, Direction::South) => match other {
                PipeType::Vertical => true,
                PipeType::NorthEast => true,
                PipeType::NorthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::SouthEast, Direction::East) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthWest => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::SouthWest, Direction::South) => match other {
                PipeType::Vertical => true,
                PipeType::NorthEast => true,
                PipeType::NorthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::SouthWest, Direction::West) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthEast => true,
                PipeType::SouthEast => true,
                _ => false,
            },

            (PipeType::Start, Direction::North) => match other {
                PipeType::Vertical => true,
                PipeType::SouthEast => true,
                PipeType::SouthWest => true,
                _ => false,
            },

            (PipeType::Start, Direction::South) => match other {
                PipeType::Vertical => true,
                PipeType::NorthEast => true,
                PipeType::NorthWest => true,
                _ => false,
            },

            (PipeType::Start, Direction::West) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthEast => true,
                PipeType::SouthEast => true,
                _ => false,
            },

            (PipeType::Start, Direction::East) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthWest => true,
                PipeType::SouthWest => true,
                _ => false,
            },

            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
pub struct PipeInfo<'a> {
    pub span: SpanIVec2<'a>,
    pub pipe_type: PipeType,
}

pub type Span<'a> = LocatedSpan<&'a str>;
pub type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;
