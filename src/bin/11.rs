use std::collections::LinkedList;

use either::Either;
use project_euler::grid::Grid;

fn main() {
    #[allow(clippy::zero_prefixed_literal)]
    let grid = Grid {
        grid: vec![
            vec![
                08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
            ],
            vec![
                49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
            ],
            vec![
                81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
            ],
            vec![
                52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
            ],
            vec![
                22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
            ],
            vec![
                24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
            ],
            vec![
                32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
            ],
            vec![
                67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
            ],
            vec![
                24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
            ],
            vec![
                21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
            ],
            vec![
                78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
            ],
            vec![
                16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
            ],
            vec![
                86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
            ],
            vec![
                19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
            ],
            vec![
                04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
            ],
            vec![
                88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
            ],
            vec![
                04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
            ],
            vec![
                20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
            ],
            vec![
                20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
            ],
            vec![
                01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
            ],
        ],
    };

    let result = row_iterators(&grid)
        .map(Either::Left)
        .chain(column_iterators(&grid).map(Either::Right))
        .map(|iter| max_product(iter, 4))
        .max()
        .unwrap();
    println!("{:?}", result);
}

fn row_iterators(grid: &Grid) -> impl Iterator<Item: Iterator<Item = usize> + '_> {
    (0..grid.num_rows()).flat_map(|row| {
        [
            Either::Left(grid.right(row)),
            Either::Right(Either::Left(grid.diagonal_from_first_column(row))),
            Either::Right(Either::Right(grid.bidiagonal_from_last_column(row))),
        ]
    })
}

fn column_iterators(grid: &Grid) -> impl Iterator<Item: Iterator<Item = usize> + '_> {
    (0..grid.num_columns()).flat_map(|column| {
        [
            Either::Left(grid.down(column)),
            Either::Right(Either::Left(grid.diagonal_from_first_row(column))),
            Either::Right(Either::Right(grid.bidiagonal_from_first_row(column))),
        ]
    })
}

#[derive(Debug)]
struct Result {
    #[allow(dead_code)]
    sequence: LinkedList<usize>,
    product: usize,
}

impl PartialEq for Result {
    fn eq(&self, other: &Self) -> bool {
        self.product == other.product
    }
}

impl PartialOrd for Result {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.product.cmp(&other.product))
    }
}

impl Eq for Result {}

impl Ord for Result {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.product.cmp(&other.product)
    }
}

fn max_product(mut iter: impl Iterator<Item = usize>, n: usize) -> Option<Result> {
    let mut sequence = LinkedList::new();
    let mut product = 1;
    for _ in 0..n {
        let next = iter.next()?;
        sequence.push_back(next);
        product *= next;
    }
    let mut result = Result {
        sequence: sequence.clone(),
        product,
    };
    for next in iter {
        let first = sequence.pop_front().unwrap();
        sequence.push_back(next);
        product = if first > 0 {
            product / first * next
        } else {
            sequence.iter().product()
        };
        if product > result.product {
            result.sequence = sequence.clone();
            result.product = product;
        }
    }
    Some(result)
}
