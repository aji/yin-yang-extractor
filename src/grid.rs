use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Black,
    White,
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;
    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.cells[r][c]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[r][c]
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid").finish_non_exhaustive()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                match self[(r, c)] {
                    Cell::Empty => write!(f, ". ")?,
                    Cell::Black => write!(f, "B ")?,
                    Cell::White => write!(f, "W ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Grid {
        if rows == 0 || cols == 0 {
            panic!("invalid grid size: {rows},{cols}");
        }
        Grid {
            cells: (0..rows)
                .map(|_| (0..cols).map(|_| Cell::Empty).collect())
                .collect(),
        }
    }

    pub fn rows(&self) -> usize {
        self.cells.len()
    }

    pub fn cols(&self) -> usize {
        self.cells[0].len()
    }

    pub fn subgrid<'g>(&'g self, r: usize, c: usize) -> Subgrid<'g> {
        Subgrid {
            cells: &self.cells[r..],
            offset: c,
        }
    }

    pub fn subgrids<'g>(&'g self, rows: usize, cols: usize) -> Subgrids<'g> {
        Subgrids {
            grid: self,
            r: 0,
            c: 0,
            rows,
            cols,
        }
    }

    pub fn is_valid(&self) -> bool {
        for g in self.subgrids(2, 2) {
            match g.as_2x2() {
                Some(x) if !x.is_empty() => return false,
                _ => {}
            }
        }
        true
    }
}

pub struct Subgrid<'g> {
    cells: &'g [Vec<Cell>],
    offset: usize,
}

impl<'g> Index<(usize, usize)> for Subgrid<'g> {
    type Output = Cell;
    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.cells[r][self.offset + c]
    }
}

impl<'g> Subgrid<'g> {
    fn as_2x2(&self) -> Option<Cell> {
        let c00 = self[(0, 0)];
        let c01 = self[(0, 1)];
        let c10 = self[(1, 0)];
        let c11 = self[(1, 1)];
        match c00 == c01 && c00 == c10 && c00 == c11 {
            true => Some(c00),
            false => None,
        }
    }
}

pub struct Subgrids<'g> {
    grid: &'g Grid,
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
}

impl<'g> Iterator for Subgrids<'g> {
    type Item = Subgrid<'g>;

    fn next(&mut self) -> Option<Self::Item> {
        // wrap col to next line if needed
        if self.c + self.cols > self.grid.cols() {
            self.c = 0;
            self.r += 1;
        }
        // end iterator if needed
        if self.r + self.rows > self.grid.rows() {
            return None;
        }

        let subgrid = self.grid.subgrid(self.r, self.c);
        self.c += 1;
        Some(subgrid)
    }
}
