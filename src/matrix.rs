#![allow(dead_code)]

use std::ops::{Add, AddAssign, Index, Mul, MulAssign};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix {
    cells: Vec<Vec<isize>>,
}

impl Matrix {
    //create
    pub fn new(cells: Vec<Vec<isize>>) -> Self {
        let no_of_cols = cells[0].len();

        if !cells.iter().all(|col| col.len() == no_of_cols) {
            panic!(
                "rows must be of the same length, expected length {}",
                no_of_cols
            )
        }

        Matrix { cells }
    }

    pub fn zeroes(rows: usize, cols: usize) -> Self {
        Matrix {
            cells: vec![vec![0; cols]; rows],
        }
    }

    pub fn from_list(rows: usize, cols: usize, list: Vec<isize>) -> Self {
        let size = rows * cols;

        if list.len() != size {
            panic!("Error: creating Matrix `from_list` input vector does not match, desired matrix length")
        }

        let mut mat = Self::zeroes(rows, cols);
        let chunks = list.chunks(cols);

        for (row, chunk) in chunks.enumerate() {
            // since the size of the vector is checked in the step above there's no reason unwrap shouldn't work
            mat.cells[row] = <&[isize] as TryInto<Vec<isize>>>::try_into(chunk).unwrap();
        }

        mat
    }

    pub fn from_matrix(rows: usize, cols: usize, mat: &Self) -> Self {
        let mut out = mat.clone();

        let upsize_rows = rows >= mat.rows();
        let upsize_cols = cols >= mat.cols();

        match (upsize_rows, upsize_cols) {
            (true, true) => {
                out.upsize(rows, cols);
            }
            (true, false) => {
                out.upsize(rows, out.cols());
                out.downsize(out.rows(), cols);
            }
            (false, true) => {
                out.downsize(rows, out.cols());
                out.upsize(out.rows(), cols);
            }
            (false, false) => {
                out.downsize(rows, cols);
            }
        }

        out
    }
}

impl Matrix {
    //internal methods

    fn upsize(&mut self, new_rows: usize, new_cols: usize) -> () {
        let row_diff = new_rows as isize - self.cells.len() as isize;

        if row_diff < 0 {
            panic!("Number of rows must be bigger to upsize, current matrix is {} trying to size to {}", self.cells.len(), new_rows)
        }

        let col_diff = new_cols as isize - self.cells[0].len() as isize;
        if col_diff < 0 {
            panic!("Number of columns must be bigger to upsize, current matrix is {} trying to size to {}", self.cells[0].len(), new_cols)
        }

        let col_extension = vec![0; col_diff as usize];
        let row_extension = vec![vec![0isize; new_cols]; row_diff as usize];

        for row in self.cells.iter_mut() {
            row.extend(col_extension.clone());
        }

        self.cells.extend(row_extension);
    }

    fn downsize(&mut self, new_rows: usize, new_cols: usize) -> () {
        let row_diff = self.cells.len() as isize - new_rows as isize;

        if row_diff < 0 {
            panic!("Number of rows must be smaller to downsize, current matrix is {} trying to size to {}", self.cells.len(), new_rows)
        }

        let col_diff = self.cells[0].len() as isize - new_cols as isize;
        if col_diff < 0 {
            panic!("Number of columns must be smaller to downsize, current matrix is {} trying to size to {}", self.cells[0].len(), new_cols)
        }

        self.cells.truncate(new_rows);
        for row in self.cells.iter_mut() {
            row.truncate(new_cols)
        }
    }

    fn rows(&self) -> usize {
        self.cells.len()
    }

    fn cols(&self) -> usize {
        self.cells[0].len()
    }
}

impl Matrix {
    // strass algo

    pub fn strass(&self, b: &Self) -> Self {
        if self.rows() == b.rows()
            && self.rows() == 2
            && self.cols() == b.cols()
            && self.cols() == 2
        {
            return self.strass_inner(&b);
        }

        let (out_dim_rows, out_dim_cols) = (self.rows(), b.cols());

        // find the closest multiple of two
        let size = self.rows().max(self.cols()).max(b.rows()).max(b.cols());
        let size = if size % 2 == 0 { size } else { size + 1 };

        let a = Matrix::from_matrix(size, size, self);
        let b = Matrix::from_matrix(size, size, b);

        let a = Blocks::from_matrix(a);
        let b = Blocks::from_matrix(b);

        let block = a.strass(&b);
        let res = block.to_matrix();

        Matrix::from_matrix(out_dim_rows, out_dim_cols, &res)
    }

    pub fn strass_inner(&self, b: &Self) -> Self {
        // perform strass_inner algorithm on 2x2 matrix
        let m1 = (self[[1, 1]] + self[[2, 2]]) * (b[[1, 1]] + b[[2, 2]]);
        let m2 = (self[[2, 1]] + self[[2, 2]]) * b[[1, 1]];
        let m3 = self[[1, 1]] * (b[[1, 2]] - b[[2, 2]]);
        let m4 = self[[2, 2]] * (b[[2, 1]] - b[[1, 1]]);
        let m5 = (self[[1, 1]] + self[[1, 2]]) * b[[2, 2]];
        let m6 = (self[[2, 1]] - self[[1, 1]]) * (b[[1, 1]] + b[[1, 2]]);
        let m7 = (self[[1, 2]] - self[[2, 2]]) * (b[[2, 1]] + b[[2, 2]]);

        Matrix::new(vec![
            vec![(m1 + m4 - m5 + m7), (m3 + m5)],
            vec![(m2 + m4), (m1 - m2 + m3 + m6)],
        ])
    }
}

impl Index<[usize; 2]> for Matrix {
    type Output = isize;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let (row, col) = (index[0], index[1]);

        &self.cells[row - 1][col - 1]
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, b: Self) -> Self::Output {
        if self.cols() != b.rows() {
            panic!("Matrix multiplaicaton only allowed for r x m and m x c matrices to form r x c matrix.\n Found {} x {} and {} x {} matrices", self.rows(), self.cols(), b.rows(), b.cols())
        }

        let mut out = Matrix::zeroes(self.rows(), b.cols());

        for (r, row) in out.cells.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                for idx in 1..=self.cols() {
                    *cell += self[[r + 1, idx]] * b[[idx, c + 1]];
                }
            }
        }

        out
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, b: Self) {
        let a = self.clone();
        *self = a * b;
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, b: Self) -> Self::Output {
        if self.rows() != b.cols() || self.cols() != b.cols() {
            panic!(
                "Matrices must be of the smae size to add. Found {} x {} + {} x {}",
                self.rows(),
                self.cols(),
                b.rows(),
                b.cols()
            );
        }

        let mut out = Self::zeroes(self.rows(), self.cols());

        for (r, row) in out.cells.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                *cell = self[[r + 1, c + 1]] + b[[r + 1, c + 1]];
            }
        }

        out
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, b: Self) {
        let a = self.clone();
        *self = a + b;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Blocks {
    // struct to handle Matrix Block Multiplication
    mats: Vec<Vec<Matrix>>,
}

impl Blocks {
    fn zeroes(rows: usize, cols: usize) -> Self {
        Blocks {
            mats: vec![vec![Matrix::zeroes(2, 2); cols]; rows],
        }
    }

    fn rows(&self) -> usize {
        self.mats.len()
    }

    fn cols(&self) -> usize {
        self.mats[0].len()
    }

    pub fn from_matrix(mat: Matrix) -> Self {
        //this assumes the matrix is of dimensions 2n x 2m

        let mut out = Blocks::zeroes(mat.rows() / 2, mat.cols() / 2);

        for (r, row) in out.mats.iter_mut().enumerate() {
            let mat_row_index = 2 * (r + 1);

            for (c, cell) in row.iter_mut().enumerate() {
                let mat_col_index = 2 * (c + 1);

                *cell = Matrix::new(vec![
                    vec![
                        mat[[mat_row_index - 1, mat_col_index - 1]],
                        mat[[mat_row_index - 1, mat_col_index]],
                    ],
                    vec![
                        mat[[mat_row_index, mat_col_index - 1]],
                        mat[[mat_row_index, mat_col_index]],
                    ],
                ])
            }
        }

        out
    }
}

impl Blocks {
    pub fn to_matrix(self) -> Matrix {
        let (out_rows, out_cols) = (self.rows() * 2, self.cols() * 2);

        let mut out = Matrix::zeroes(out_rows, out_cols);

        for (r, row) in out.cells.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                let mat_row_idx = if (r) % 2 == 0 { 1 } else { 2 };
                let mat_col_idx = if (c) % 2 == 0 { 1 } else { 2 };

                *cell = self[[(r / 2) + 1, (c / 2) + 1]][[mat_row_idx, mat_col_idx]];
            }
        }
        out
    }

    pub fn strass(&self, b: &Self) -> Self {
        let mut out = Blocks::zeroes(self.rows(), self.cols());

        for (r, row) in out.mats.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                for idx in 1..=self.cols() {
                    *cell += self[[r + 1, idx]].strass_inner(&b[[idx, c + 1]]);
                }
            }
        }

        out
    }
}

impl Index<[usize; 2]> for Blocks {
    type Output = Matrix;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let (row, col) = (index[0], index[1]);

        &self.mats[row - 1][col - 1]
    }
}

#[cfg(test)]
#[path = "./_tests/matrix.rs"]
mod tests;
