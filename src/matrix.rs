use std::cmp::max;

/// Création d'une matrice de taille n x m
///
pub struct Matrix {
  data: Vec<Vec<u32>>
}


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

  /// Donne la valeur la plus grande sur une matrice carré d'ordre n.
  /// Les propriétés donné à l'algorithme sont interdiction de remonter ou
  /// d'aller à gauche.
  ///
  /// L'objectif est de récupérer le maximum de pièce sachant
  /// que le nombre de pièce est élevé à la valeur d'une case
  /// de la matrice.
  ///
  /// Cette fonction renvoie une nouvelle matrice où chaque correspond
  /// à la somme des pièces sur les cases parcourus
  ///
  ///```
  ///let mut matrix = Matrix::new(vec![
  ///  vec![1, 2, 8],
  ///  vec![1, 1, 1],
  ///  vec![10, 1, 1]
  ///]);
  ///matrix.dyn_big_value().print();
  ///```
  pub fn dyn_big_value (&mut self) -> Matrix {
    if !self.is_square() {
      panic!("La matrice n'est pas carré");
    }

    let n = self.get_size();
    let mut v = Matrix::create_zero_matrix(n, self.get_value(0,0));

    for i in 1..n {
      v.update_value(i,0, v.get_value(i-1, 0) + self.get_value(i, 0));
      v.update_value(0,i, v.get_value(0, i-1)+ self.get_value(0,i));
    }

    for i in 1..n {
      for j in 1..n {
        v.update_value(
          i,j,
          max(
            v.get_value(i-1, j) + self.get_value(i,j),
            v.get_value(i, j-1) + self.get_value(i,j)
          )
        )
      }
    }
    v
  }

  /// Permet de trouver le chemin optimal pour récupérer le maximum de pièce
  /// sur une matrice carré d'ordre n.
  /// Les propriétés donné à l'algorithme sont interdiction de remonter ou
  /// d'aller à gauche.
  ///
  /// ```
  /// let mut matrix = Matrix::new(vec![
  ///   vec![1, 2, 8],
  ///   vec![1, 1, 1],
  ///   vec![10, 1, 1]
  /// ]);
  ///
  /// let n = matrix.get_size();
  /// let max_value = matrix
  ///   .dynamic_big_value()
  ///   .get_value(n-1, n-1);
  ///
  /// println!("The maximum number of rooms: {}", max_value);
  ///
  /// let path = matrix.find_optimal_path();
  /// let mut val = 0;
  ///
  /// for (x,y) in path {
  ///   let value = matrix.get_value(x,y);
  ///   val += value;
  ///   println!("x: {}, y: {}, value: {}", x, y, value);
  /// }
  ///
  /// println!("Total value: {}", val);
  ///```
  pub fn find_optimal_path (&mut self) -> Vec<(usize, usize)> {
    let value_matrix = self.dyn_big_value();
    let mut path = Vec::new();
    let n = value_matrix.get_size();

    let mut i = n-1;
    let mut j = n-1;
    path.push((i,j));

    while i > 0 && j > 0 {
      if value_matrix.get_value(i-1, j) > value_matrix.get_value(i, j-1) {
        i -= 1;
      } else {
        j -= 1;
      }
      path.push((i,j));
    }

    while i > 0 {
      i -= 1;
      path.push((i,j));
    }

    while j > 0 {
      j -= 1;
      path.push((i,j));
    }
    path.reverse();
    path
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