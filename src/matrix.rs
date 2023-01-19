use std::ops::{Index, Mul};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix {
    cells: Vec<Vec<isize>>,
}

impl Matrix {
    //create
    fn new(cells: Vec<Vec<isize>>) -> Self {
        let no_of_cols = cells[0].len();

        if !cells.iter().all(|col| col.len() == no_of_cols) {
            panic!(
                "rows must be of the same length, expected length {}",
                no_of_cols
            )
        }

        Matrix { cells }
    }

    fn zeroes(rows: usize, cols: usize) -> Self {
        Matrix {
            cells: vec![vec![0; cols]; rows],
        }
    }

    fn from_list(rows: usize, cols: usize, list: Vec<isize>) -> Self {
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

    fn from_matrix(mat: Self, rows: usize, cols: usize) -> Self {
        todo!()
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

    fn row_length(&self) -> usize {
        self.cells[0].len()
    }

    fn col_length(&self) -> usize {
        self.cells.len()
    }
}

impl Index<[usize; 2]> for Matrix {
    type Output = isize;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let (row, col) = (index[0], index[1]);

        &self.cells[row - 1][col - 1]
    }
}

impl Matrix {
    // strassen algo

    fn strass(&self, b: &Self) -> Self {
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

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, b: Self) -> Self::Output {
        let mut out = Matrix::zeroes(2, 2);

        for (i, row) in out.cells.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                for idx in 1..=self.row_length() {
                    *cell += self[[i+1, idx]] * b[[idx, j+1]]
                }
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mat = Matrix {
            cells: vec![vec![1, 2], vec![3, 4]],
        };

        let list = vec![vec![1, 2], vec![3, 4]];
        let result = Matrix::new(list);

        assert_eq!(mat, result)
    }
    #[test]
    fn zeroes() {
        let mat = Matrix {
            cells: vec![vec![0isize; 2]; 2],
        };

        let result = Matrix::zeroes(2, 2);
        assert_eq!(mat, result)
    }
    #[test]
    fn from_list() {
        let mat = Matrix {
            cells: vec![vec![1, 2], vec![3, 4]],
        };

        let list = vec![1, 2, 3, 4];
        let result = Matrix::from_list(2, 2, list);

        assert_eq!(mat, result)
    }

    #[test]
    fn upsize() {
        let mat = Matrix {
            cells: vec![vec![1, 2, 0], vec![3, 4, 0], vec![0, 0, 0]],
        };

        let list = vec![1, 2, 3, 4];
        let mut result = Matrix::from_list(2, 2, list);
        result.upsize(3, 3);

        assert_eq!(mat, result)
    }

    #[test]
    fn asymmetric_upsize_cols() {
        let mat = Matrix {
            cells: vec![vec![1, 2, 0], vec![3, 4, 0]],
        };

        let list = vec![1, 2, 3, 4];
        let mut result = Matrix::from_list(2, 2, list);
        result.upsize(2, 3);

        assert_eq!(mat, result)
    }

    #[test]
    fn asymmetric_upsize_rows() {
        let mat = Matrix {
            cells: vec![vec![1, 2], vec![3, 4], vec![0, 0]],
        };

        let list = vec![1, 2, 3, 4];
        let mut result = Matrix::from_list(2, 2, list);
        result.upsize(3, 2);

        assert_eq!(mat, result)
    }

    #[test]
    fn downsize() {
        let mat = Matrix {
            cells: vec![vec![1, 2], vec![3, 4]],
        };

        let list = vec![1, 2, 0, 3, 4, 0, 0, 0, 0];
        let mut result = Matrix::from_list(3, 3, list);
        result.downsize(2, 2);

        assert_eq!(mat, result)
    }

    #[test]
    fn asymmetric_downsize_rows() {
        let mat = Matrix {
            cells: vec![vec![1, 2, 0], vec![3, 4, 0]],
        };

        let list = vec![1, 2, 0, 3, 4, 0, 0, 0, 0];
        let mut result = Matrix::from_list(3, 3, list);
        result.downsize(2, 3);

        assert_eq!(mat, result)
    }

    #[test]
    fn asymmetric_downsize_cols() {
        let mat = Matrix {
            cells: vec![vec![1, 2], vec![3, 4], vec![0, 0]],
        };

        let list = vec![1, 2, 0, 3, 4, 0, 0, 0, 0];
        let mut result = Matrix::from_list(3, 3, list);
        result.downsize(3, 2);

        assert_eq!(mat, result)
    }

    #[test]
    fn index_2x2() {
        let mat = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

        assert_eq!(1, mat[[1, 1]]);
        assert_eq!(2, mat[[1, 2]]);
        assert_eq!(3, mat[[2, 1]]);
        assert_eq!(4, mat[[2, 2]]);
    }

    #[test]
    fn index_3x3() {
        let mat = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        assert_eq!(1, mat[[1, 1]]);
        assert_eq!(2, mat[[1, 2]]);
        assert_eq!(3, mat[[1, 3]]);
        assert_eq!(4, mat[[2, 1]]);
        assert_eq!(5, mat[[2, 2]]);
        assert_eq!(6, mat[[2, 3]]);
        assert_eq!(7, mat[[3, 1]]);
        assert_eq!(8, mat[[3, 2]]);
        assert_eq!(9, mat[[3, 3]]);
    }

    #[test]
    fn strass_mul() {
        let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

        let mat = Matrix::new(vec![vec![7, 10], vec![15, 22]]);
        assert_eq!(mat, a.strass(&b));
    }

    #[test]
    fn strass_indent_mul() {
        let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::new(vec![vec![1, 0], vec![0, 1]]);

        assert_eq!(a, a.strass(&b));
    }

    #[test]
    fn mul() {
        let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

        let mat = Matrix::new(vec![vec![7, 10], vec![15, 22]]);
        assert_eq!(mat, a * b);
    }

    #[test]
    fn indent_mul() {
        let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::new(vec![vec![1, 0], vec![0, 1]]);

        assert_eq!(a.clone(), a * b);
    }
}
