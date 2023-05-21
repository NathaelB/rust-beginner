mod matrix;
mod programmation_dynamique;

use crate::matrix::Matrix;
use crate::programmation_dynamique::courses::{Course, Courses};
use crate::programmation_dynamique::seccotine::Seccotine;

#[allow(dead_code)]
fn glouton (m: &Matrix) -> Option<u32> {
  if !m.is_square() {
    return None
  }
  let mut x = 0;
  let mut y = 0;
  let n = m.get_size();
  let mut v = m.get_data()[x][y];
  for _ in 0..2*n-2 {
    if x == n-1 {
      y += 1;
      v += m.get_data()[x][y];
    } else if y == n-1 {
      x += 1;
      v += m.get_data()[x][y];
    } else if m.get_data()[x+1][y] > m.get_data()[x][y+1]{
      x += 1;
      v += m.get_data()[x][y];
    } else {
      y += 1;
      v += m.get_data()[x][y]
    }
  }

  return Some(v)
}


fn main() {
  let vec2 = vec![
    vec![1, 2, 8],
    vec![1, 1, 1],
    vec![10, 1, 1],
  ];
  let mut matrix2 = Matrix::new(vec2);

  let mut seccotine = Seccotine::new(matrix2.clone());
  let path = seccotine.find_optimal_path();

  println!("Optimal Path:");
  let mut val = 0;
  for (x, y) in path {
    let value = matrix2.get_value(x, y);
    val += value;
    println!("({}, {}): {}", x, y, value);
  }

  println!("Total value: {}", val);

  println!("-----------------------------------");
  println!("TP 3 PROGRAMMATION DYNAMIQUE COURSES");
  println!("-----------------------------------");


  let mut courses = Courses::new();

  courses
    .push(Course::new(76, 78, 10))
    .push(Course::new(12,17,2))
    .push(Course::new(13,15,1))
    .push(Course::new(19,28,8))
    .push(Course::new(12,20,7))
    .push(Course::new(44,45,9))
    .push(Course::new(43,45,5))
    .push(Course::new(1,8,3));

  courses.sort_courses();

  courses.print();
  let pred = courses.calcul_pred();
  println!("PRED: {:?}", pred);
}
