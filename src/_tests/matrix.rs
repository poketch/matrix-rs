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
fn from_mat_2x2_3x3() {


    let a = Matrix::from_list(2, 2, vec![1, 2, 3,4]);

    let mat = Matrix::from_list(3, 3, vec![1,2, 0, 3, 4,0,0,0,0]);

    let result = Matrix::from_matrix(3, 3, a);

    assert_eq!(mat, result)
}

#[test]
fn from_mat_3x3_2x2() {


    let a = Matrix::from_list(3, 3, vec![1,2, 0, 3, 4,0,0,0,0]);
    let mat = Matrix::from_list(2, 2, vec![1, 2, 3,4]);


    let result = Matrix::from_matrix(2, 2, a);

    assert_eq!(mat, result)
}

#[test]
fn from_mat_2x2_2x1() {


    let a = Matrix::from_list(2, 2, vec![1, 2, 3,4]);

    let mat = Matrix::from_list(2,1, vec![1,3]);

    let result = Matrix::from_matrix(2, 1, a);

    assert_eq!(mat, result)
}

#[test]
fn from_mat_2x2_1x2() {


    let a = Matrix::from_list(2, 2, vec![1, 2, 3, 4]);

    let mat = Matrix::from_list(1,2, vec![1,2]);

    let result = Matrix::from_matrix(1, 2, a);

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
fn strass_ident_mul() {
    let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
    let b = Matrix::new(vec![vec![1, 0], vec![0, 1]]);

    let mat = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    assert_eq!(mat, a.strass(&b));
}

#[test]
fn mul_2x2() {
    let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
    let b = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    let mat = Matrix::new(vec![vec![7, 10], vec![15, 22]]);
    assert_eq!(mat, a * b);
}

#[test]
fn mul_3x3() {
    let a = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    let b = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

    let mat = Matrix::from_list(3, 3, vec![30, 36, 42, 66, 81, 96, 102, 126, 150]);
    assert_eq!(mat, a * b);
}

#[test]
fn asym_1x2_2x1_mul() {
    let a = Matrix::new(vec![vec![1, 2]]);
    let b = Matrix::new(vec![vec![3], vec![4]]);

    let mat = Matrix::new(vec![vec![11]]);
    assert_eq!(mat, a * b);
}

#[test]
fn asym_2x1_1x2_mul() {
    let a = Matrix::new(vec![vec![1], vec![2]]);
    let b = Matrix::new(vec![vec![3, 4]]);

    let mat = Matrix::new(vec![vec![3, 4], vec![6, 8]]);
    assert_eq!(mat, a * b);
}

#[test]
fn asym_3x1_1x3_mul() {
    let a = Matrix::new(vec![vec![1], vec![2], vec![3]]);
    let b = Matrix::new(vec![vec![4, 5, 6]]);

    let mat = Matrix::new(vec![vec![4, 5, 6], vec![8, 10, 12], vec![12, 15, 18]]);
    assert_eq!(mat, a * b);
}

#[test]
fn ident_mul() {
    let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
    let b = Matrix::new(vec![vec![1, 0], vec![0, 1]]);

    let mat = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    assert_eq!(mat, a * b);
}

#[test]
fn mul_assign() {
    let mut a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
    let b = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
    a *= b;

    let mat = Matrix::new(vec![vec![7, 10], vec![15, 22]]);

    assert_eq!(mat, a);
}

#[test]
fn indent_mul_assign() {
    let mut a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
    let b = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    a *= b;

    let mat = Matrix::new(vec![vec![1, 2], vec![3, 4]]);

    assert_eq!(mat, a);
}
