use either::Either;

pub struct Grid {
    /// Left to right, up to bottom
    pub grid: Vec<Vec<usize>>,
}

impl Grid {
    pub fn num_rows(&self) -> usize {
        self.grid.len()
    }

    pub fn num_columns(&self) -> usize {
        self.grid[0].len()
    }

    pub fn right(&self, row: usize) -> impl Iterator<Item = usize> + '_ {
        self.grid[row].iter().copied()
    }

    pub fn down(&self, column: usize) -> impl Iterator<Item = usize> + '_ {
        self.grid.iter().map(move |row| row[column])
    }

    pub fn diagonal_from_first_row(&self, column: usize) -> impl Iterator<Item = usize> + '_ {
        if column + self.num_rows() > self.num_columns() {
            // ends on last column
            Either::Left((column..self.num_columns()).map(move |i| self.grid[i - column][i]))
        } else {
            // ends on last row
            Either::Right((0..self.num_rows()).map(move |i| self.grid[i][i + column]))
        }
    }

    pub fn diagonal_from_first_column(&self, row: usize) -> impl Iterator<Item = usize> + '_ {
        if row + self.num_columns() > self.num_rows() {
            // ends on last row
            Either::Left((row..self.num_rows()).map(move |i| self.grid[i][i - row]))
        } else {
            // ends on last column
            Either::Right((0..self.num_columns()).map(move |i| self.grid[i + row][i]))
        }
    }

    pub fn bidiagonal_from_first_row(&self, column: usize) -> impl Iterator<Item = usize> + '_ {
        if self.num_rows() > column {
            // ends on first column
            Either::Left((0..=column).rev().map(move |i| self.grid[column - i][i]))
        } else {
            // ends on last row
            Either::Right((0..self.num_rows()).map(move |i| self.grid[i][column - i]))
        }
    }

    pub fn bidiagonal_from_last_column(&self, row: usize) -> impl Iterator<Item = usize> + '_ {
        if row + self.num_columns() > self.num_rows() {
            // ends on last row
            Either::Left(
                (row..self.num_rows())
                    .map(move |i| self.grid[i][self.num_columns() - 1 - (i - row)]),
            )
        } else {
            // ends on first column
            Either::Right(
                (0..self.num_columns())
                    .rev()
                    .map(move |i| self.grid[row + self.num_columns() - 1 - i][i]),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::Grid;

    fn assert_iter<I: Iterator>(iter: I, expected: Vec<I::Item>)
    where
        I::Item: Debug + PartialEq,
    {
        assert_eq!(iter.collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_grid() {
        let grid = Grid {
            grid: vec![vec![0, 1], vec![2, 3], vec![4, 5]],
        };
        assert_iter(grid.right(1), vec![2, 3]);
        assert_iter(grid.down(1), vec![1, 3, 5]);
        assert_iter(grid.diagonal_from_first_row(1), vec![1]);
        assert_iter(grid.diagonal_from_first_column(1), vec![2, 5]);
        assert_iter(grid.bidiagonal_from_first_row(1), vec![1, 2]);
        assert_iter(grid.bidiagonal_from_last_column(1), vec![3, 4]);
    }
}
