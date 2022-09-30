use super::CoordVec;
use bitflags::bitflags;

/// Four-way directions.
///
/// These start at North and increment clockwise,
/// so you can convert them to integers with `as` and use them
/// in rotational calculations if you need.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction4 {
    North,
    East,
    South,
    West,
}

impl Direction4 {
    /// All the directions in order.
    /// This is used internally for rotations and flips.
    /// I made it public just in case it's helpful for you the programmer.
    pub const DIRECTIONS: [Direction4; 4] = [
        Direction4::North,
        Direction4::East,
        Direction4::South,
        Direction4::West,
    ];

    /// Rotate this by the given amount.
    pub fn rotate(self, rot: Rotation) -> Self {
        self.rotate_by(rot.steps_clockwise())
    }

    /// Get this direction, rotated by this many steps clockwise.
    /// Negative numbers go counter-clockwise.
    pub fn rotate_by(self, steps_clockwise: i32) -> Self {
        let idx = self as i32;
        let new_idx = ((idx + steps_clockwise).rem_euclid(Self::DIRECTIONS.len() as i32)) as usize;
        Self::DIRECTIONS[new_idx]
    }

    /// Flip this direction.
    pub fn flip(self) -> Self {
        self.rotate_by(2)
    }

    /// Get this direction in radians.
    ///
    /// This uses trigonometric + graphical standard, where:
    /// - 0 radians is to the right
    /// - Positive radians increment *clockwise*. NOTE: this is opposite from normal trig,
    /// but makes sense in computer graphics where +Y is downwards.
    ///
    /// If you need it in degrees just call `.to_degrees` on the result.
    pub fn radians(self) -> f32 {
        ((self as i8) - 1).rem_euclid(4) as f32 * std::f32::consts::TAU / 4.0
    }

    /// Get the deltas a step in this direction would result in, as a CoordVec.
    pub fn deltas(self) -> CoordVec {
        let (x, y) = match self {
            Direction4::North => (0, -1),
            Direction4::East => (1, 0),
            Direction4::South => (0, 1),
            Direction4::West => (-1, 0),
        };
        CoordVec { x, y }
    }

    /// See if this direction points horizontally (ie, is `East` or `West`).
    pub fn is_horizontal(self) -> bool {
        matches!(self, Direction4::East | Direction4::West)
    }

    /// See if this direction points vertically (ie, is `North` or `South`).
    pub fn is_vertical(self) -> bool {
        matches!(self, Direction4::North | Direction4::South)
    }
}

/// Eight-way directions.
///
/// These start at North and increment counter-clockwise,
/// so you can convert them to integers with `as` and use them
/// in rotational calculations if you need.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction8 {
    /// All the directions in order.
    /// This is used internally for rotations and flips.
    /// I made it public just in case it's helpful for you the programmer.
    pub const DIRECTIONS: [Direction8; 8] = [
        Direction8::North,
        Direction8::NorthEast,
        Direction8::East,
        Direction8::SouthEast,
        Direction8::South,
        Direction8::SouthWest,
        Direction8::West,
        Direction8::NorthWest,
    ];

    /// Rotate this by the given amount.
    pub fn rotate(self, rot: Rotation) -> Self {
        self.rotate_by(rot.steps_clockwise())
    }

    /// Get this direction, rotated by this many steps clockwise.
    /// Negative numbers go counter-clockwise.
    pub fn rotate_by(self, steps_clockwise: i32) -> Self {
        let idx = self as i32;
        let new_idx = ((idx + steps_clockwise).rem_euclid(Self::DIRECTIONS.len() as i32)) as usize;
        Self::DIRECTIONS[new_idx]
    }

    /// Flip this direction.
    pub fn flip(self) -> Self {
        self.rotate_by(4)
    }

    /// Get this direction in radians.
    ///
    /// This uses trigonometric + graphical standard, where:
    /// - 0 radians is to the right
    /// - Positive radians increment *clockwise*. NOTE: this is opposite from normal trig,
    /// but makes sense in computer graphics where +Y is downwards.
    ///
    /// If you need it in degrees just call `.to_degrees` on the result.
    pub fn radians(self) -> f32 {
        ((self as i8) - 2).rem_euclid(8) as f32 * std::f32::consts::TAU / 8.0
    }

    /// Get the deltas a step in this direction would result in,
    /// as an CoordVec.
    pub fn deltas(self) -> CoordVec {
        let (x, y) = match self {
            Direction8::North => (0, -1),
            Direction8::NorthEast => (1, -1),
            Direction8::East => (1, 0),
            Direction8::SouthEast => (1, 1),
            Direction8::South => (0, 1),
            Direction8::SouthWest => (-1, 1),
            Direction8::West => (-1, 0),
            Direction8::NorthWest => (-1, -1),
        };
        CoordVec { x, y }
    }
}

impl From<Direction4> for Direction8 {
    fn from(d4: Direction4) -> Self {
        match d4 {
            Direction4::North => Direction8::North,
            Direction4::East => Direction8::East,
            Direction4::South => Direction8::South,
            Direction4::West => Direction8::West,
        }
    }
}

/// Nine-way directions: 8 compass points plus center.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction9 {
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl Direction9 {
    pub const DIRECTIONS: &'static [Direction9] = &[
        Direction9::NorthWest,
        Direction9::North,
        Direction9::NorthEast,
        Direction9::West,
        Direction9::Center,
        Direction9::East,
        Direction9::SouthEast,
        Direction9::South,
        Direction9::SouthWest,
    ];

    /// Rotate this by the given amount.
    pub fn rotate(self, rot: Rotation) -> Self {
        self.rotate_by(rot.steps_clockwise())
    }

    /// Get this direction, rotated by this many steps clockwise.
    /// Negative numbers go counter-clockwise.
    pub fn rotate_by(self, steps_clockwise: i32) -> Self {
        let dir: Result<Direction8, _> = self.try_into();
        match dir {
            Ok(dir) => dir.rotate_by(steps_clockwise).into(),
            Err(()) => self,
        }
    }

    /// Flip this direction.
    pub fn flip(self) -> Self {
        self.rotate_by(4)
    }

    /// Get the deltas a step in this direction would result in,
    /// as an CoordVec.
    pub fn deltas(self) -> CoordVec {
        let (x, y) = match self {
            Direction9::NorthWest => (-1, -1),
            Direction9::North => (0, -1),
            Direction9::NorthEast => (1, -1),
            Direction9::West => (-1, 0),
            Direction9::Center => (0, 0),
            Direction9::East => (1, 0),
            Direction9::SouthWest => (-1, 1),
            Direction9::South => (0, 1),
            Direction9::SouthEast => (1, 1),
        };
        CoordVec { x, y }
    }
}

impl TryFrom<Direction9> for Direction8 {
    type Error = ();

    fn try_from(value: Direction9) -> Result<Self, Self::Error> {
        Ok(match value {
            Direction9::NorthWest => Direction8::NorthWest,
            Direction9::North => Direction8::North,
            Direction9::NorthEast => Direction8::NorthEast,
            Direction9::West => Direction8::West,
            Direction9::Center => Err(())?,
            Direction9::East => Direction8::East,
            Direction9::SouthEast => Direction8::SouthEast,
            Direction9::South => Direction8::South,
            Direction9::SouthWest => Direction8::SouthWest,
        })
    }
}

impl From<Direction8> for Direction9 {
    fn from(dir: Direction8) -> Self {
        match dir {
            Direction8::North => Direction9::North,
            Direction8::NorthEast => Direction9::NorthEast,
            Direction8::East => Direction9::East,
            Direction8::SouthEast => Direction9::SouthEast,
            Direction8::South => Direction9::South,
            Direction8::SouthWest => Direction9::SouthWest,
            Direction8::West => Direction9::West,
            Direction8::NorthWest => Direction9::NorthWest,
        }
    }
}

/// 2-way rotations: clockwise or counterclockwise.
/// These don't indicate any specific angle by themselves, only in relation to something.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

impl Rotation {
    /// Get the number of steps clockwise this does.
    /// - `Clockwise` is 1
    /// - `CounterClockwise` is -1
    pub fn steps_clockwise(&self) -> i32 {
        match self {
            Rotation::Clockwise => 1,
            Rotation::CounterClockwise => -1,
        }
    }
}

bitflags! {
    pub struct Direction4Set: u8 {
        const NORTH = 0b0001;
        const EAST = 0b0010;
        const SOUTH = 0b0100;
        const WEST = 0b1000;
    }
}

impl Direction4Set {
    pub fn has(&self, dir: Direction4) -> bool {
        (self.bits & (1 << dir as u8)) != 0
    }
}

impl From<Direction4> for Direction4Set {
    fn from(dir: Direction4) -> Self {
        match dir {
            Direction4::North => Self::NORTH,
            Direction4::East => Self::EAST,
            Direction4::South => Self::SOUTH,
            Direction4::West => Self::WEST,
        }
    }
}

bitflags! {
    pub struct Direction8Set: u8 {
        const NORTH = 0x01;
        const NORTHEAST = 0x02;
        const EAST = 0x04;
        const SOUTHEAST = 0x08;
        const SOUTH = 0x10;
        const SOUTHWEST = 0x20;
        const WEST = 0x40;
        const NORTHWEST = 0x80;
    }
}

impl Direction8Set {
    pub fn has(&self, dir: Direction8) -> bool {
        (self.bits & (1 << dir as u8)) != 0
    }
}

impl From<Direction8> for Direction8Set {
    fn from(dir: Direction8) -> Self {
        match dir {
            Direction8::North => Direction8Set::NORTH,
            Direction8::NorthEast => Direction8Set::NORTHEAST,
            Direction8::East => Direction8Set::EAST,
            Direction8::SouthEast => Direction8Set::SOUTHEAST,
            Direction8::South => Direction8Set::SOUTH,
            Direction8::SouthWest => Direction8Set::SOUTHWEST,
            Direction8::West => Direction8Set::WEST,
            Direction8::NorthWest => Direction8Set::NORTHWEST,
        }
    }
}
