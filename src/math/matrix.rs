
#[derive(Default, Clone)]
pub struct Matrix<T> {
    pub rows: usize,
    pub columns: usize,
    matrix: Vec<Vec<T>>
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(rows: usize, columns: usize) -> Self {
        let matrix = vec![vec![T::default(); columns]; rows];
        Matrix { rows, columns, matrix }
    }
    
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.columns {
            Some(&self.matrix[row][col])
        } else {
            None
        }
    }
    
    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), &'static str> {
        if row < self.rows && col < self.columns {
            self.matrix[row][col] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.matrix.iter().flat_map(|row| row.iter())
    }
    
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.matrix.iter_mut().flat_map(|row| row.iter_mut())
    }
}
