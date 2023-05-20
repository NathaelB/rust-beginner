pub(crate) mod courses;

#[cfg(test)]
mod tests {
  use crate::programmation_dynamique::courses::{Course, Courses};

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
}