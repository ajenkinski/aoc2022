use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use std::path::Path;

use itertools::Itertools;

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    return io::BufReader::new(file).lines().collect();
}

pub type Coord = (usize, usize);

/// A 2D grid convenience type
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Grid<T> {
        assert!(data.iter().map(|row| row.len()).all_equal());
        Grid(data)
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.num_rows(), self.num_cols())
    }

    pub fn num_rows(&self) -> usize {
        self.0.len()
    }

    pub fn num_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn neighbor_coords(&self, coord: Coord) -> impl Iterator<Item = Coord> {
        let (row, col) = (coord.0 as isize, coord.1 as isize);
        let row_range = 0..(self.num_rows() as isize);
        let col_range = 0..(self.num_cols() as isize);

        [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]
        .into_iter()
        .filter_map(move |(nr, nc)| {
            if row_range.contains(&nr) && col_range.contains(&nc) {
                Some((nr as usize, nc as usize))
            } else {
                None
            }
        })
    }

    pub fn all_coords(&self) -> impl Iterator<Item = Coord> {
        (0..self.num_rows()).cartesian_product(0..self.num_cols())
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
