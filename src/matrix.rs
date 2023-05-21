use std::cmp::max;

#[derive(Debug, Clone)]
pub struct Matrix {
  data: Vec<Vec<u32>>
}

#[allow(dead_code)]
impl Matrix {
  pub fn new (data: Vec<Vec<u32>>) -> Self {
    Matrix {
      data
    }
  }

  pub fn create_zero_matrix (n: usize, value: u32) -> Matrix {
    let data = vec![vec![0; n]; n];
    let mut matrix = Matrix::new(data);
    matrix.update_value(0,0, value);
    matrix
  }

  pub fn update_value (&mut self, x: usize, y: usize, value: u32) {
    if let Some(row) = self.data.get_mut(x) {
      if let Some(cell) = row.get_mut(y) {
        *cell = value;
      }
    }
  }

  pub fn is_square (&self) -> bool {
    if let Some(first_row) = self.data.first() {
      let num_rows = self.data.len();
      let num_cols = first_row.len();
      num_rows == num_cols
    } else {
      false
    }
  }

  pub fn print (&self) {
    for row in &self.data {
      for cell in row {
        print!("{} ", cell);
      }
      println!();
    }
  }

  pub fn get_size (&self) -> usize {
    self.data.len()
  }

  pub fn get_value (&self, x: usize, y: usize) -> u32 {
    self.data[x][y]
  }

  #[allow(dead_code)]
  pub fn get_data (&self) -> Vec<Vec<u32>> {
    self.data.clone()
  }
}