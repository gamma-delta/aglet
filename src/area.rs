use super::Coord;

#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub corner: Coord,
    pub width: u32,
    pub height: u32,
}

impl Area {
    pub fn new(corner: Coord, width: u32, height: u32) -> Self {
        Self {
            corner,
            width,
            height,
        }
    }
}

impl IntoIterator for Area {
    type Item = Coord;

    type IntoIter = AreaIter;

    fn into_iter(self) -> Self::IntoIter {
        AreaIter {
            area: self,
            cursor: 0,
        }
    }
}

pub struct AreaIter {
    area: Area,
    cursor: u32,
}

impl Iterator for AreaIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.area.width * self.area.height {
            return None;
        }

        let x = self.cursor % self.area.width;
        let y = self.cursor / self.area.width;
        self.cursor += 1;
        Some(Coord::new(x, y))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for AreaIter {
    fn len(&self) -> usize {
        (self.area.width * self.area.height - self.cursor) as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Edges {
    pub corner: Coord,
    pub width: u32,
    pub height: u32,
}

impl Edges {
    pub fn new(corner: Coord, width: u32, height: u32) -> Self {
        Self {
            corner,
            width,
            height,
        }
    }
}

impl IntoIterator for Edges {
    type Item = Coord;

    type IntoIter = EdgesIter;

    fn into_iter(self) -> Self::IntoIter {
        EdgesIter {
            edges: self,
            cursor: 0,
        }
    }
}

pub struct EdgesIter {
    edges: Edges,
    cursor: u32,
}

impl Iterator for EdgesIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.edges.corner
            + if self.cursor < self.edges.width {
                Coord::new(self.cursor, 0)
            } else if self.cursor < self.edges.width + self.edges.height - 1 {
                Coord::new(self.edges.width - 1, self.cursor - self.edges.width + 1)
            } else if self.cursor < self.edges.width * 2 + self.edges.height - 2 {
                Coord::new(
                    self.edges.width - (self.cursor + 3 - self.edges.width - self.edges.height),
                    self.edges.height - 1,
                )
            } else if self.cursor < self.edges.width * 2 + self.edges.height * 2 - 4 {
                Coord::new(
                    0,
                    self.edges.height
                        - (self.cursor + 4 - self.edges.height - self.edges.width * 2),
                )
            } else {
                return None;
            };
        self.cursor += 1;
        Some(out)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for EdgesIter {
    fn len(&self) -> usize {
        (2 * self.edges.width + 2 * self.edges.height - 2 - self.cursor) as usize
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{Area, Coord, Edges};

    #[test]
    fn areas() {
        let area = Area::new(Coord::new(0, 0), 5, 5);
        let area_vec: Vec<_> = area.into_iter().collect();
        let area_set: HashSet<_> = area.into_iter().collect();
        assert_eq!(area_vec.len(), 25);
        assert_eq!(area_set.len(), 25);
    }

    #[test]
    fn edges() {
        let edges: Vec<_> = Edges::new(Coord::new(0, 0), 5, 4)
            .into_iter()
            .map(|c| (c.x, c.y))
            .collect();
        assert_eq!(
            edges,
            [
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (3, 3),
                (2, 3),
                (1, 3),
                (0, 3),
                (0, 2),
                (0, 1)
            ]
        );

        let edges: Vec<_> = Edges::new(Coord::new(7, 11), 3, 4)
            .into_iter()
            .map(|c| (c.x, c.y))
            .collect();
        assert_eq!(
            edges,
            [
                (7, 11),
                (8, 11),
                (9, 11),
                (9, 12),
                (9, 13),
                (9, 14),
                (8, 14),
                (7, 14),
                (7, 13),
                (7, 12)
            ]
        );

        let edges: Vec<_> = Edges::new(Coord::new(0, 0), 2, 6)
            .into_iter()
            .map(|c| (c.x, c.y))
            .collect();
        assert_eq!(
            edges,
            [
                (0, 0),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (0, 5),
                (0, 4),
                (0, 3),
                (0, 2),
                (0, 1)
            ]
        );
    }
}
