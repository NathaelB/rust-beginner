use crate::matrix::Matrix;

pub struct Seccotine {
  m: Matrix
}

impl Seccotine {
  pub fn new (m: Matrix) -> Self {
    Seccotine {
      m
    }
  }

  pub fn get_matrix (self) -> Matrix {
    self.m
  }

  pub fn glouton (&self) -> Option<u32> {
    if !self.m.is_square() {
      return None
    }

    let mut x = 0;
    let mut y = 0;
    let n = self.m.get_size();
    let mut v = self.m.get_value(x,y);

    for _ in 0..2*n-2 {
      if x== n-1 {
        y += 1;
        v += self.m.get_value(x,y);
      } else if y == n-1 {
        x += 1;
        v += self.m.get_value(x,y);
      } else if self.m.get_value(x+1,y) > self.m.get_value(x,y+1) {
        x += 1;
        v += self.m.get_value(x,y);
      } else {
        y += 1;
        v += self.m.get_value(x,y);
      }
    }

    Some(v)
  }

  /// Donne la valeur la plus grande sur une matrice carré d'ordre n.
  /// Les propriétés donné à l'algorithme sont interdiction de remonter ou
  /// d'aller à gauche.
  ///
  /// L'objectif est de récupérer le maximum de pièce sachant
  /// que le nombre de pièce est élevé à la valeur d'une case de la matrice.
  ///
  /// Cette fonction renvoie une nouvelle matrice où chaque correspond
  /// à la somme des pièces sur les cases parcourus
  ///
  /// ```
  /// let mut matrix = Matrix::new(vec![
  ///   vec![1, 2, 8],
  ///   vec![1, 1, 1],
  ///   vec![10, 1, 1]
  /// ]);
  /// let seccotine = Seccotine::new(matrix.clone());
  /// seccotine.dyn_big_value().print();
  /// ```
  pub fn dyn_big_value (&mut self) -> Matrix {
    if !self.m.is_square() {
      panic!("La matrice n'est pas carrée");
    }

    let n = self.m.get_size();
    let mut v = Matrix::create_zero_matrix(n, self.m.get_value(0,0));

    for i in 1..n {
      v.update_value(i,0, v.get_value(i-1, 0) + self.m.get_value(i, 0));
      v.update_value(0,i, v.get_value(0, i-1)+ self.m.get_value(0,i));
    }

    for i in 1..n {
      for j in 1..n {
        v.update_value(
          i,j,
          std::cmp::max(
            v.get_value(i-1, j) + self.m.get_value(i,j),
            v.get_value(i, j-1) + self.m.get_value(i,j)
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
  /// let seccotine = Seccotine::new(matrix.clone());
  ///
  /// let mut seccotine = Seccotine::new(matrix2.clone());
  /// let path = seccotine.find_optimal_path();
  ///
  /// println!("Optimal Path:");
  ///
  /// let mut val = 0;
  /// for (x, y) in path {
  ///   let value = seccotine.get_matrix().get_value(x, y);
  ///   val += value;
  ///   println!("({}, {}): {}", x, y, value);
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
      path.push((i, j));
    }

    path.reverse();
    path
  }
}