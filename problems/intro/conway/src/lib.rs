#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows: rows,
            cols: cols,
            grid: Vec::with_capacity(rows * cols),
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows: rows,
            cols: cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        return (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        return &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.cols + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut res= Vec::<(usize, usize)>::new();
        self.push_real_neighbour(&mut res, row.wrapping_sub(1), col.wrapping_sub(1));
        self.push_real_neighbour(&mut res, row.wrapping_sub(1), col);
        self.push_real_neighbour(&mut res, row.wrapping_sub(1), col.wrapping_add(1));
        self.push_real_neighbour(&mut res, row, col.wrapping_sub(1));
        self.push_real_neighbour(&mut res, row, col.wrapping_add(1));
        self.push_real_neighbour(&mut res, row.wrapping_add(1), col.wrapping_sub(1));
        self.push_real_neighbour(&mut res, row.wrapping_add(1), col);
        self.push_real_neighbour(&mut res, row.wrapping_add(1), col.wrapping_add(1));
        return res
    }

    pub fn push_real_neighbour(&self, neigh: &mut Vec<(usize, usize)>, row: usize, col: usize) {
        match (row, col) {
            (x, y) if x < self.rows && y < self.cols => {
                neigh.push((x, y));
            },
            _ => {}
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
    newgrid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self {
            grid: grid.clone(),
            newgrid: grid,
        }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        return &self.grid
    }

    pub fn step(&mut self) {
        let (rows, cols) = self.grid.size();
        for i in 0..rows {
            for j in 0..cols {
                let mut alive: i32 = 0;
                let cell: &Cell = self.grid.get(i, j);
                for n in self.grid.neighbours(i, j) {
                    match self.grid.get(n.0, n.1) {
                        Cell::Alive => alive += 1,
                        _ => {},
                    }
                }
                match alive {
                    3 => self.newgrid.set(Cell::Alive, i, j),
                    2 => self.newgrid.set(*cell, i, j),
                    _ => self.newgrid.set(Cell::Dead, i, j),
                }
            }
        }
        std::mem::swap(&mut self.grid, &mut self.newgrid);
    }
}
