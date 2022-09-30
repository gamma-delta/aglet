use super::Coord;

/// Like a `HashMap<Coord, T>` but faster. Each grid point might store something.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    pub fn insert(&mut self, coord: Coord, val: T) -> Option<T> {
        let idx = self.idx(coord)?;
        self.spots[idx].replace(val)
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

    pub fn iter(&self) -> impl Iterator<Item = (Coord, &T)> {
        self.spots.iter().enumerate().filter_map(|(idx, spot)| {
            spot.as_ref().map(|spot| {
                (
                    Coord::new(idx as u32 % self.width, idx as u32 / self.width),
                    spot,
                )
            })
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Coord, &mut T)> {
        self.spots.iter_mut().enumerate().filter_map(|(idx, spot)| {
            spot.as_mut().map(|spot| {
                (
                    Coord::new(idx as u32 % self.width, idx as u32 / self.width),
                    spot,
                )
            })
        })
    }

    pub fn into_iter(self) -> impl Iterator<Item = (Coord, T)> {
        let width = self.width;
        self.spots
            .into_iter()
            .enumerate()
            .filter_map(move |(idx, spot)| {
                spot.map(|spot| (Coord::new(idx as u32 % width, idx as u32 / width), spot))
            })
    }

    fn idx(&self, coord: Coord) -> Option<usize> {
        if coord.x >= self.width || coord.y >= self.height {
            None
        } else {
            Some((self.width * coord.y + coord.x) as usize)
        }
    }
}
