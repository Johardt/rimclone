#[derive(Default, Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    matrix: Vec<T>,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            matrix: vec![T::default(); rows * cols],
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.matrix[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), &'static str> {
        if row < self.rows && col < self.cols {
            self.matrix[row * self.cols + col] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.matrix.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.matrix.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let matrix: Matrix<i32> = Matrix::new(3, 4);
        assert_eq!(matrix.rows(), 3);
        assert_eq!(matrix.columns(), 4);
    }

    #[test]
    fn test_matrix_get_set() {
        let mut matrix: Matrix<i32> = Matrix::new(2, 2);

        assert!(matrix.set(0, 0, 1).is_ok());
        assert!(matrix.set(0, 1, 2).is_ok());
        assert!(matrix.set(1, 0, 3).is_ok());
        assert!(matrix.set(1, 1, 4).is_ok());

        assert_eq!(matrix.get(0, 0), Some(&1));
        assert_eq!(matrix.get(0, 1), Some(&2));
        assert_eq!(matrix.get(1, 0), Some(&3));
        assert_eq!(matrix.get(1, 1), Some(&4));
    }

    #[test]
    fn test_out_of_bounds() {
        let mut matrix: Matrix<i32> = Matrix::new(2, 2);

        assert!(matrix.set(2, 0, 1).is_err());
        assert!(matrix.set(0, 2, 1).is_err());

        assert_eq!(matrix.get(2, 0), None);
        assert_eq!(matrix.get(0, 2), None);
    }
}
