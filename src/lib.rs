#![doc = include_str!("../README.md")]

mod area;
mod direction;
mod grid;
mod lines;

pub use area::*;
pub use direction::*;
pub use grid::*;
pub use lines::*;

use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::num::TryFromIntError;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Unsigned-int coordinates
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub const ZERO: Coord = Coord::new(0, 0);

    /// Make a new coord.
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Get this as an index into an array representing a 2d array.
    ///
    /// (AKA, `y * width + x`.)
    pub fn to_2d_idx(self, width: u32) -> u32 {
        // what did you think i was kidding or something
        self.y * width + self.x
    }

    /// Convert this into an ICoord.
    pub fn to_icoord(self) -> CoordVec {
        self.into()
    }

    /// Get a list of this coordinate's orthagonal neighbors.
    /// They are given in clockwise order starting with the neighbor to the north,
    /// as if each of [`Direction4::DIRECTIONS`] had been added to them.
    ///
    /// If a neighbor is out of bounds, it is skipped in the output.
    ///
    /// There may be 2, 3, or 4 neighbors:
    /// - 2 if this is at `(0, 0)`
    /// - 3 if this is on an edge (`x` or `y` are 0)
    /// - 4 otherwise.
    ///
    /// [`Direction4::DIRECTIONS`]: super::Direction4::DIRECTIONS
    pub fn neighbors4(self) -> Vec<Coord> {
        Direction4::DIRECTIONS
            .iter()
            .filter_map(|dir| {
                let iself = self.to_icoord();
                let ineighbor = iself + *dir;
                ineighbor.to_coord() // conveniently already returns an option.
            })
            .collect()
    }

    /// Get a list of this coordinate's orthagonal and diagonal neighbors.
    /// They are given in clockwise order starting with the neighbor to the north,
    /// as if each of [`Direction8::DIRECTIONS`] had been added to them.
    ///
    /// If a neighbor is out of bounds, it is skipped in the output.
    ///
    /// There may be 3, 5, or 8 neighbors:
    /// - 3 if this is at `(0, 0)`
    /// - 5 if this is on an edge (`x` or `y` are 0)
    /// - 8 otherwise.
    /// [`Direction8::DIRECTIONS`]: super::Direction8::DIRECTIONS
    pub fn neighbors8(self) -> Vec<Coord> {
        Direction8::DIRECTIONS
            .iter()
            .filter_map(|dir| {
                let iself = self.to_icoord();
                let ineighbor = iself + *dir;
                ineighbor.to_coord() // conveniently already returns an option.
            })
            .collect()
    }

    pub fn area(self, width: u32, height: u32) -> Area {
        Area::new(self, width, height)
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<u32> for Coord {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<u32> for Coord {
    fn mul_assign(&mut self, rhs: u32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<Coord> for Coord {
    type Output = Self;
    fn mul(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign<Coord> for Coord {
    fn mul_assign(&mut self, rhs: Coord) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

/// Try to convert an ICoord to a Coord.
/// Will return Error if the ICoord has any negatives in it.
impl TryFrom<CoordVec> for Coord {
    type Error = TryFromIntError;
    fn try_from(value: CoordVec) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Signed-int coordinates
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CoordVec {
    pub x: i32,
    pub y: i32,
}

impl CoordVec {
    /// Create a new ICoord
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Return the quadrant this coordinate is in.
    ///
    /// - 1: +X, +Y
    /// - 2: -X, +Y
    /// - 3: -X, -Y
    /// - 4: +X, -Y
    ///
    /// Zeroes are treated as positive.
    pub fn quadrant(self) -> u32 {
        match (self.x >= 0, self.y >= 0) {
            (true, true) => 1,
            (false, true) => 2,
            (false, false) => 3,
            (true, false) => 4,
        }
    }

    /// Try to convert this to a Coord.
    /// Returns `None` in case any part is negative.
    pub fn to_coord(self) -> Option<Coord> {
        self.try_into().ok()
    }

    /// Get a list of this coordinate's orthagonal neighbors.
    /// They are given in clockwise order starting with the neighbor to the north,
    /// as if each of [`Direction4::DIRECTIONS`] had been added to them.
    ///
    /// [`Direction4::DIRECTIONS`]: super::Direction4::DIRECTIONS
    pub fn neighbors4(self) -> [CoordVec; 4] {
        [
            self + Direction4::North,
            self + Direction4::East,
            self + Direction4::South,
            self + Direction4::West,
        ]
    }

    /// Get a list of this coordinate's orthagonal and diagonal neighbors.
    /// They are given in clockwise order starting with the neighbor to the north,
    /// as if each of [`Direction8::DIRECTIONS`] had been added to them.
    ///
    /// [`Direction8::DIRECTIONS`]: super::Direction8::DIRECTIONS
    pub fn neighbors8(self) -> [CoordVec; 8] {
        [
            self + Direction8::North,
            self + Direction8::NorthEast,
            self + Direction8::East,
            self + Direction8::SouthEast,
            self + Direction8::South,
            self + Direction8::SouthWest,
            self + Direction8::West,
            self + Direction8::NorthWest,
        ]
    }

    /// Turn this into the closest Direction9 it is pointing in.
    ///
    /// This uses the convention that north is positive Y.
    pub fn point9(self) -> Direction9 {
        if self.x == 0 && self.y == 0 {
            return Direction9::Center;
        }
        // there's gotta be a better way to do this
        let angle = (-self.y as f32).atan2(self.x as f32) + std::f32::consts::PI;
        match angle / std::f32::consts::TAU * 16.0 {
            a if a < 1.0 => Direction9::East,
            a if a < 3.0 => Direction9::NorthEast,
            a if a < 5.0 => Direction9::North,
            a if a < 7.0 => Direction9::NorthWest,
            a if a < 9.0 => Direction9::West,
            a if a < 11.0 => Direction9::SouthWest,
            a if a < 13.0 => Direction9::South,
            a if a < 15.0 => Direction9::SouthEast,
            _ => Direction9::East,
        }
    }
}

impl Add for CoordVec {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for CoordVec {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for CoordVec {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for CoordVec {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Add<Direction4> for CoordVec {
    type Output = Self;
    fn add(self, rhs: Direction4) -> Self::Output {
        self + rhs.deltas()
    }
}

impl AddAssign<Direction4> for CoordVec {
    fn add_assign(&mut self, rhs: Direction4) {
        *self += rhs.deltas();
    }
}

impl Add<Direction8> for CoordVec {
    type Output = Self;
    fn add(self, rhs: Direction8) -> Self::Output {
        self + rhs.deltas()
    }
}

impl AddAssign<Direction8> for CoordVec {
    fn add_assign(&mut self, rhs: Direction8) {
        *self += rhs.deltas();
    }
}

impl Mul<i32> for CoordVec {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<i32> for CoordVec {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl From<Coord> for CoordVec {
    fn from(value: Coord) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl Display for CoordVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
