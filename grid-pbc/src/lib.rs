use core::panic;

pub struct PBCGrid<T> {
    x: usize,
    y: usize,
    data: Vec<T>,
}

impl<T: Default> PBCGrid<T> {
    pub fn new_default(x: usize, y: usize) -> Self {
        if x == 0 || y == 0 {
            panic!()
        }

        Self {
            x,
            y,
            data: (0..(x * y)).into_iter().map(|_| T::default()).collect(),
        }
    }
}

impl<T: Clone> PBCGrid<T> {
    pub fn new_clone(x: usize, y: usize, d: T) -> Self {
        if x == 0 || y == 0 {
            panic!()
        }

        Self {
            x,
            y,
            data: (0..(x * y)).into_iter().map(|_| d.clone()).collect(),
        }
    }
}

impl<T: Copy> PBCGrid<T> {
    pub fn new_copy(x: usize, y: usize, d: T) -> Self {
        if x == 0 || y == 0 {
            panic!()
        }

        Self {
            x,
            y,
            data: (0..(x * y)).into_iter().map(|_| d).collect(),
        }
    }
}

impl<T> PBCGrid<T> {
    pub fn new_empty(x: usize, y: usize) -> Self {
        if x == 0 || y == 0 {
            panic!()
        }

        Self {
            x,
            y,
            data: Vec::with_capacity(x * y),
        }
    }

    pub fn new_from_vec(x: usize, y: usize, v: Vec<T>) -> Self {
        // TODO fix panic
        if x == 0 || y == 0 {
            panic!()
        }

        if x * y != v.len() {
            panic!()
        }

        Self { x, y, data: v }
    }

    pub fn get_xy(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.x + x]
    }

    pub fn get_xy_up(&self, x: usize, y: usize) -> &T {
        self.get_xy(x, (y + self.y - 1) % self.y) // + self.y => no overflow (-1)
    }

    pub fn get_xy_down(&self, x: usize, y: usize) -> &T {
        self.get_xy(x, (y + 1) % self.y)
    }

    pub fn get_xy_left(&self, x: usize, y: usize) -> &T {
        self.get_xy((x + self.x - 1) % self.x, y) // + self.y => no overflow (-1)
    }

    pub fn get_xy_right(&self, x: usize, y: usize) -> &T {
        self.get_xy((x + 1) % self.x, y)
    }

    pub fn get_xy_adjacent(&self, x: usize, y: usize) -> Vec<&T> {
        vec![
            self.get_xy_up(x, y),
            self.get_xy_down(x, y),
            self.get_xy_left(x, y),
            self.get_xy_right(x, y),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::PBCGrid;

    #[test]
    fn test_get_xy() {
        let grid = PBCGrid::new_from_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(*grid.get_xy(0, 0), 1);
        assert_eq!(*grid.get_xy(2, 2), 9);
        assert_eq!(*grid.get_xy(1, 0), 4);
        assert_eq!(*grid.get_xy(1, 1), 5);
    }

    #[test]
    fn test_pbc() {
        let grid = PBCGrid::new_from_vec(4, 4, (0..=15).collect());
        assert_eq!(grid.get_xy_adjacent(0, 0), vec![&12, &4, &3, &1])
    }
}
