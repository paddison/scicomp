pub mod helpers;

use std::fmt::Display;

// Represents a n*n matrix, organized as a single vector array.
pub struct Matrix {
    vals: Vec<f64>,
    dimension: usize,
}

fn solve(mut m: Matrix, mut b: Vec<f64>) -> Option<Vec<f64>> {
    if m.dimension != b.len() {
        return None;
    }
    let mut result: Vec<f64> = vec![0.; m.dimension];
    for i in 0..m.dimension {
        // normalize row so that m(i,i) == 1
        let one = m.vals[m.dimension * i + i]; // get m(i, i);
        for j in i..m.dimension {
            m.vals[m.dimension * i + j] /= one;   // divide all elements of current row by m(i, i)
        }

        // don't forget to divide the vector, too
        b[i] /= one;

        // subtract from other rows (starting from current row + 1) and respective indices in vector
        // i denotes the row we use to subtract, j the current row that gets subtracted from
        for j in (i + 1)..m.dimension {
            let val_to_multiply = m.vals[m.dimension * j + i];
            for k in i..m.dimension {
                // multiply row with the value we need to set the ith element in current row to zero
                let value_to_subtract = m.vals[m.dimension * i + k] * val_to_multiply;
                m.vals[m.dimension * j + k] -= value_to_subtract;
            }
            b[j] -= b[i] * val_to_multiply;

        }
        // maybe this can be done at the beginning of the function, so we don't need if???
        // swap rows so the one with the highest next element is next, in order to increase precision
        if i < m.dimension - 1 {
            m.pivot_rows(i + 1, &mut b); 
        }
    }

    // next, "go upwards" and solve for the rest
    // here i denotes the current solved row, and j all the other rows that are not yet solved
    for i in (0..(m.dimension)).rev() {
        // the last value in b is the z value of the solution (if 3 dimensional)
        // if b[i] is NaN it means we probably divided by 0, which means there are no solutions
        if b[i].is_nan() {
            return None;
        }
        result[i] = b[i];

        // we need to subtract element m(i, i) from other rows
        // loop through all rows above the current one
        for j in (0..i).rev() {
            let val_to_multiply = m.vals[m.dimension * j + i];
            m.vals[m.dimension * j + i] -= m.vals[m.dimension * i + i] * val_to_multiply;
            b[j] -= b[i] * val_to_multiply;
        }
    }
    if !m.is_identity_form() {
        return None;
    }
    Some(result)
}   

impl Matrix {
    pub fn new(vals: Vec<f64>) -> Option<Matrix> {
        let dimension = (vals.len() as f64).sqrt() as usize;
        if dimension * dimension == vals.len() {
            Some(Matrix { vals, dimension })
        } else {
            None
        }
    }

    // todo switch rows with highest value (pivot)
    // check for errors (rows with more than one solution, or no solution (eg 0 = 2))
    pub fn solve(&mut self, mut b: Vec<f64>) -> Option<Vec<f64>> {
        if self.dimension != b.len() {
            return None;
        }
        let mut result: Vec<f64> = vec![0.; self.dimension];
        for i in 0..self.dimension {
            // normalize row so that m(i,i) == 1
            let one = self.vals[self.dimension * i + i]; // get m(i, i);
            for j in i..self.dimension {
                self.vals[self.dimension * i + j] /= one;   // divide all elements of current row by m(i, i)
            }

            // don't forget to divide the vector, too
            b[i] /= one;

            // subtract from other rows (starting from current row + 1) and respective indices in vector
            // i denotes the row we use to subtract, j the current row that gets subtracted from
            for j in (i + 1)..self.dimension {
                let val_to_multiply = self.vals[self.dimension * j + i];
                for k in i..self.dimension {
                    // multiply row with the value we need to set the ith element in current row to zero
                    let value_to_subtract = self.vals[self.dimension * i + k] * val_to_multiply;
                    self.vals[self.dimension * j + k] -= value_to_subtract;
                }
                b[j] -= b[i] * val_to_multiply;

            }
            // maybe this can be done at the beginning of the function, so we don't need if???
            // swap rows so the one with the highest next element is next, in order to increase precision
            if i < self.dimension - 1 {
                self.pivot_rows(i + 1, &mut b); 
            }
        }

        // next, "go upwards" and solve for the rest
        // here i denotes the current solved row, and j all the other rows that are not yet solved
        for i in (0..(self.dimension)).rev() {
            // the last value in b is the z value of the solution (if 3 dimensional)
            // if b[i] is NaN it means we probably divided by 0, which means there are no solutions
            if b[i].is_nan() {
                return None;
            }
            result[i] = b[i];

            // we need to subtract element m(i, i) from other rows
            // loop through all rows above the current one
            for j in (0..i).rev() {
                let val_to_multiply = self.vals[self.dimension * j + i];
                self.vals[self.dimension * j + i] -= self.vals[self.dimension * i + i] * val_to_multiply;
                b[j] -= b[i] * val_to_multiply;
            }
        }
        if !self.is_identity_form() {
            return None;
        }
        Some(result)
    }

    fn pivot_rows(&mut self, start_index: usize, b: &mut Vec<f64>) {
        let mut pivot = self.vals[self.dimension * start_index + start_index];
        let mut swap_row_index = start_index;
        for i in (start_index + 1)..self.dimension {
            let current_element = self.vals[self.dimension * i + start_index];
            if current_element < pivot {
                pivot = current_element;
                swap_row_index = i;
            }
        }
        if swap_row_index != start_index {
            for i in start_index..self.dimension {
                self.vals.swap(self.dimension * start_index + i, self.dimension * swap_row_index + i);
            }
            b.swap(swap_row_index, start_index);
        }
    }

    fn is_identity_form(&self) -> bool {
        // i denotes rows, j columns
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                let current_value = self.vals[self.dimension * i + j];
                if i == j && current_value != 1. {
                    return false;
                } else if i != j && current_value != 0. {
                    return false;
                }
            }
        }
        true 
    }
}

impl From<Vec<f64>> for Matrix {  
    fn from(vals: Vec<f64>) -> Self {
        match Self::new(vals) {
            Some(m) => m,
            None => panic!("Vector needs to have a square length"),
        }
    }
}

impl From<&[f64]> for Matrix {
    fn from(vals: &[f64]) -> Self {
        let mut raw_matrix = vec![];
        raw_matrix.extend_from_slice(vals);
        match Self::new(raw_matrix) {
            Some(m) => m,
            None => panic!("Vector needs to have a squared length"),
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("[");
        for (i, val) in self.vals.iter().enumerate() {
            if i % self.dimension == 0 && i != 0 {
                string += "\n";
            }
            if i != self.vals.len() - 1 {
                string += &format!("{}, ", val); 
            } else { 
                string += &format!("{}", val); 
            }

        }
        string += "]";
        write!(f, "{}", string)
    }
}

#[test]
fn new_matrix_some() {
    let m = Matrix::new(vec![1., 2., 3., 4.]);
    assert!(m.is_some());
    assert_eq!(m.unwrap().dimension, 2);
}

#[test]
fn new_matrix_none() {
    let m = Matrix::new(vec![1., 2., 3.]);
    assert!(m.is_none());
}

#[test]
fn solve_3_dim() {
    let mut m = Matrix::new(vec![1., 1., -2., 3., -1., 1., 2., 3., 5.]).unwrap();
    assert_eq!(vec![2., 3., -1.], m.solve(vec![7., 2., 8.]).unwrap());
}  

#[test]
fn solve_4_dim() {
    let mut m = Matrix::new(vec![
        1., 2., -3., -1., 
        0., -3., 2., 6.,
        -3., -1., 3., 1.,
        2., 3., 2., -1.]).unwrap();
    let result = m.solve(vec![0., -8., 0., -8.]).unwrap();
    assert_eq!(result, vec![-1., -2., -1., -2.]);
}

#[test]
fn pivot_swap() {
    let mut m = Matrix::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]).unwrap();
    m.pivot_rows(1, &mut vec![1., 2., 3.]);
    assert_eq!(m.vals, vec![1., 2., 3., 4., 8., 9., 7., 5., 6.]);
}

#[test]
fn pivot_dont_swap() {
    let mut m = Matrix::new(vec![1., 2., 3., 4., 5., 6., 1., 2., 3.]).unwrap();
    m.pivot_rows(1, &mut vec![1., 2., 3.]);
    assert_eq!(m.vals, vec![1., 2., 3., 4., 5., 6., 1., 2., 3.]);
}

#[test]
fn no_solution() {
    let mut m = Matrix::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]).unwrap();
    let result = m.solve(vec![2., 2., 2.]);
    println!("{:?}", result);
}

#[test]
fn is_identity_form() {
    let m = Matrix::new(vec![1., 0., 0., 0., 1., 0., 0., 0., 1.]).unwrap();
    assert!(m.is_identity_form());
}

#[test]
fn is_not_identity_form() {
    let m = Matrix::new(vec![1., 0., 0., 0., 1., 0., 0., 2., 1.]).unwrap();
    assert!(!m.is_identity_form());
}

