use crate::{Coord, CoordVec};

/// Iterates over coordinates on a line using Bresenham's algorithm.
///
/// Implementation taken mostly from https://crates.io/crates/bresenham,
/// with some new features.
#[derive(Debug)]
pub struct LineIter {
    cursor: CoordVec,
    deltas: CoordVec,
    x1: i32,
    diff: i32,
    octant: Octant,
    end_mode: LineEndMode,
}

impl LineIter {
    /// Creates a new iterator. Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    pub fn new(start: Coord, end: Coord) -> LineIter {
        Self::new_with_end_mode(start, end, LineEndMode::StopBefore)
    }

    pub fn new_with_end_mode(start: Coord, end: Coord, end_mode: LineEndMode) -> LineIter {
        let octant = Octant::from_points(start, end);

        let start = octant.to_octant0(start.into());
        let end = octant.to_octant0(end.into());

        let dx = end.x as i32 - start.x as i32;
        let dy = end.y as i32 - start.y as i32;

        LineIter {
            cursor: start,
            deltas: CoordVec::new(dx, dy),
            x1: end.x,
            diff: dy - dx,
            octant,
            end_mode,
        }
    }
}

impl Iterator for LineIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let stop = match self.end_mode {
            LineEndMode::StopBefore => self.cursor.x >= self.x1,
            LineEndMode::StopAt => self.cursor.x > self.x1,
            LineEndMode::Never => false,
        };
        if stop {
            return None;
        }

        let out = match self.octant.from_octant0(self.cursor.into()).try_into() {
            Ok(it) => it,
            Err(_) => return None,
        };

        if self.diff >= 0 {
            self.cursor.y += 1;
            self.diff -= self.deltas.x;
        }

        self.diff += self.deltas.y;

        // loop inc
        self.cursor.x += 1;

        Some(out)
    }
}

/// Where to stop the iteration of the line.
#[derive(Debug, Clone, Copy, Default)]
pub enum LineEndMode {
    /// Stop immediately before the endpoint is reached
    #[default]
    StopBefore,
    /// Stop once the endpoint is reached, so the iterator includes the end.
    StopAt,
    /// Just keep on going past the end point. You should probably use `.take(n)` or similar
    /// to prevent your program from going forever.
    Never,
}

#[derive(Debug)]
struct Octant(u8);

impl Octant {
    /// adapted from http://codereview.stackexchange.com/a/95551
    #[inline]
    fn from_points(start: Coord, end: Coord) -> Octant {
        let mut dx = end.x as i32 - start.x as i32;
        let mut dy = end.y as i32 - start.y as i32;

        let mut octant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2
        }

        if dx < dy {
            octant += 1
        }

        Octant(octant)
    }

    #[inline]
    fn to_octant0(&self, p: CoordVec) -> CoordVec {
        let (x, y) = match self.0 {
            0 => (p.x, p.y),
            1 => (p.y, p.x),
            2 => (p.y, -p.x),
            3 => (-p.x, p.y),
            4 => (-p.x, -p.y),
            5 => (-p.y, -p.x),
            6 => (-p.y, p.x),
            7 => (p.x, -p.y),
            _ => unreachable!(),
        };
        CoordVec::new(x, y)
    }

    #[inline]
    fn from_octant0(&self, p: CoordVec) -> CoordVec {
        let (x, y) = match self.0 {
            0 => (p.x, p.y),
            1 => (p.y, p.x),
            2 => (-p.y, p.x),
            3 => (-p.x, p.y),
            4 => (-p.x, -p.y),
            5 => (-p.y, -p.x),
            6 => (p.y, -p.x),
            7 => (p.x, -p.y),
            _ => unreachable!(),
        };
        CoordVec::new(x, y)
    }
}

/// might as well
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wp_example() {
        let li = LineIter::new(Coord::new(0, 1), Coord::new(6, 4));
        let res: Vec<_> = li.map(|c| (c.x, c.y)).collect();

        assert_eq!(res, [(0, 1), (1, 1), (2, 2), (3, 2), (4, 3), (5, 3)]);
    }

    #[test]
    fn test_inverse_wp() {
        let li = LineIter::new(Coord::new(6, 4), Coord::new(0, 1));
        let res: Vec<_> = li.map(|c| (c.x, c.y)).collect();

        assert_eq!(res, [(6, 4), (5, 4), (4, 3), (3, 3), (2, 2), (1, 2)])
    }

    #[test]
    fn test_straight_hline() {
        let li = LineIter::new(Coord::new(2, 3), Coord::new(5, 3));
        let res: Vec<_> = li.map(|c| (c.x, c.y)).collect();

        assert_eq!(res, [(2, 3), (3, 3), (4, 3)]);
    }

    #[test]
    fn test_straight_vline() {
        let li = LineIter::new(Coord::new(2, 3), Coord::new(2, 6));
        let res: Vec<_> = li.map(|c| (c.x, c.y)).collect();

        assert_eq!(res, [(2, 3), (2, 4), (2, 5)]);
    }

    #[test]
    fn test_endmode_stop_at() {
        let li =
            LineIter::new_with_end_mode(Coord::new(0, 1), Coord::new(6, 4), LineEndMode::StopAt);
        let res: Vec<_> = li.map(|c| (c.x, c.y)).collect();
        assert_eq!(
            res,
            [(0, 1), (1, 1), (2, 2), (3, 2), (4, 3), (5, 3), (6, 4)]
        );

        let li =
            LineIter::new_with_end_mode(Coord::new(6, 4), Coord::new(0, 1), LineEndMode::StopAt);
        let res: Vec<_> = li.map(|c| (c.x, c.y)).collect();
        assert_eq!(
            res,
            [(6, 4), (5, 4), (4, 3), (3, 3), (2, 2), (1, 2), (0, 1)]
        );

        let li =
            LineIter::new_with_end_mode(Coord::new(2, 3), Coord::new(5, 3), LineEndMode::StopAt);
        let res: Vec<_> = li.map(|c| (c.x, c.y)).collect();
        assert_eq!(res, [(2, 3), (3, 3), (4, 3), (5, 3)]);

        let li =
            LineIter::new_with_end_mode(Coord::new(2, 3), Coord::new(2, 6), LineEndMode::StopAt);
        let res: Vec<_> = li.map(|c| (c.x, c.y)).collect();
        assert_eq!(res, [(2, 3), (2, 4), (2, 5), (2, 6)]);
    }

    #[test]
    fn test_endmode_stop_never() {
        let li =
            LineIter::new_with_end_mode(Coord::new(0, 1), Coord::new(2, 2), LineEndMode::Never)
                .take(7);
        let res: Vec<_> = li.map(|c| (c.x, c.y)).collect();
        assert_eq!(
            res,
            [(0, 1), (1, 1), (2, 2), (3, 2), (4, 3), (5, 3), (6, 4)]
        );
    }

    #[test]
    fn test_why_isnt_foxfire_working() {
        let li = LineIter::new_with_end_mode(
            Coord::new(31 * 72 + 32, 31 * 24 + 15),
            Coord::new(31 * 72 + 36, 31 * 24 + 14),
            LineEndMode::Never,
        )
        .take(10);
        let res: Vec<_> = li.collect();
        println!("{:?}", res);
    }
}
