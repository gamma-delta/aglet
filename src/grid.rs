use std::{iter::Enumerate, slice, vec};

use super::Coord;

/// Like a `HashMap<Coord, T>` but faster. Each grid point might store something.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Grid<T> {
    width: u32,
    height: u32,
    spots: Vec<Option<T>>,
}

impl<T> Grid<T> {
    pub fn new(width: u32, height: u32) -> Grid<T> {
        Self {
            width,
            height,
            spots: std::iter::repeat_with(|| None)
                .take((width * height) as usize)
                .collect(),
        }
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        let idx = self.idx(coord)?;
        self.spots[idx].as_ref()
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        let idx = self.idx(coord)?;
        self.spots[idx].as_mut()
    }

    /// Returns the old value
    pub fn insert(&mut self, coord: Coord, val: T) -> Option<T> {
        let idx = self.idx(coord)?;
        self.spots[idx].replace(val)
    }

    pub fn get_or_insert_with<F: FnOnce() -> T>(&mut self, coord: Coord, fallback: F) -> &mut T {
        // Workaround "get or insert" limitation in borrowck
        if self.get(coord).is_some() {
            return self.get_mut(coord).unwrap();
        }
        self.insert(coord, fallback());
        self.get_mut(coord).unwrap()
    }
    pub fn get_or_insert(&mut self, coord: Coord, fallback: T) -> &mut T {
        self.get_or_insert_with(coord, || fallback)
    }

    pub fn contains(&self, coord: Coord) -> bool {
        match self.idx(coord) {
            Some(idx) => self.spots[idx].is_some(),
            None => false,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Iterate over all the (filled) slots in the grid.
    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter {
            inner: self.spots.iter().enumerate(),
            width: self.width,
        }
    }

    /// Iterate mutably over all the (filled) slots in the grid.
    pub fn iter_mut(&mut self) -> GridIterMut<'_, T> {
        GridIterMut {
            inner: self.spots.iter_mut().enumerate(),
            width: self.width,
        }
    }

    fn idx(&self, coord: Coord) -> Option<usize> {
        if coord.x >= self.width || coord.y >= self.height {
            None
        } else {
            Some((self.width * coord.y + coord.x) as usize)
        }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (Coord, T);

    type IntoIter = GridIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIter {
            inner: self.spots.into_iter().enumerate(),
            width: self.width,
        }
    }
}

/// Borrowing iterator over the filled slots in a [`Grid`].
pub struct GridIter<'a, T> {
    inner: Enumerate<slice::Iter<'a, Option<T>>>,
    width: u32,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, slot)) = self.inner.next() {
            let slot = match slot {
                Some(it) => it,
                None => continue,
            };

            return Some((
                Coord::new(idx as u32 % self.width, idx as u32 / self.width),
                slot,
            ));
        }
        // We've exhausted the internal vec
        None
    }
}

/// Mutably borrowing iterator over the filled slots in a [`Grid`].
pub struct GridIterMut<'a, T> {
    inner: Enumerate<slice::IterMut<'a, Option<T>>>,
    width: u32,
}

impl<'a, T> Iterator for GridIterMut<'a, T> {
    type Item = (Coord, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, slot)) = self.inner.next() {
            let slot = match slot {
                Some(it) => it,
                None => continue,
            };

            return Some((
                Coord::new(idx as u32 % self.width, idx as u32 / self.width),
                slot,
            ));
        }
        // We've exhausted the internal vec
        None
    }
}

/// Owning iterator over the filled slots in a [`Grid`].
pub struct GridIntoIter<T> {
    inner: Enumerate<vec::IntoIter<Option<T>>>,
    width: u32,
}

impl<T> Iterator for GridIntoIter<T> {
    type Item = (Coord, T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, slot)) = self.inner.next() {
            let slot = match slot {
                Some(it) => it,
                None => continue,
            };

            return Some((
                Coord::new(idx as u32 % self.width, idx as u32 / self.width),
                slot,
            ));
        }
        // We've exhausted the internal vec
        None
    }
}
