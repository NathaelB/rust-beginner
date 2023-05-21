pub(crate) mod courses;
pub mod seccotine;

#[cfg(test)]
mod tests {
  use crate::matrix::Matrix;
  use crate::programmation_dynamique::courses::{Course, Courses};
  use crate::programmation_dynamique::seccotine::Seccotine;

  #[test]
  fn filter_courses () {
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

    assert_eq!(courses.data[0].end, 8);
    assert_eq!(courses.data[1].end, 15);
    assert_eq!(courses.data[2].end, 17);
    assert_eq!(courses.data[3].end, 20);
    assert_eq!(courses.data[4].end, 28);
    assert_eq!(courses.data[5].end, 45);
    assert_eq!(courses.data[6].end, 45);
    assert_eq!(courses.data[7].end, 78);
  }

  #[test]
  pub fn test_seccotine () {
    let vec2 = vec![
      vec![1, 2, 8],
      vec![1, 1, 1],
      vec![10, 1, 1],
    ];
    let matrix2 = Matrix::new(vec2);

    let mut seccotine = Seccotine::new(matrix2.clone());
    let path = seccotine.find_optimal_path();

    assert_eq!(path.len(), 5);
    assert_eq!(path[0], (0, 0));
    assert_eq!(path[1], (1, 0));
    assert_eq!(path[2], (2, 0));
    assert_eq!(path[3], (2, 1));
    assert_eq!(path[4], (2, 2));
  }

  #[test]
  pub fn pred_courses () {
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

    let mut pred = courses.calcul_pred();

    assert_eq!(pred[0], -1);
    assert_eq!(pred[1], 0);
    assert_eq!(pred[2], 0);
    assert_eq!(pred[3], 0);
    assert_eq!(pred[4], 2);
    assert_eq!(pred[5], 4);
    assert_eq!(pred[6], 4);
    assert_eq!(pred[7], 5);
  }
}