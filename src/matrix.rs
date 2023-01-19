#[derive(Debug, PartialEq, Eq)]
pub struct Matrix {
    cells: Vec<Vec<isize>>,
}

impl Matrix { //create
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

impl Matrix { //internal methods 
    
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
}
