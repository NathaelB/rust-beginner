use rand::Rng;

pub struct Course {
  start: usize,
  end: usize,
  value: usize,
}

pub struct Courses {
  pub data: Vec<Course>,
}

#[allow(dead_code)]
impl Course {
  pub fn new(start: usize, end: usize, value: usize) -> Self {
    Course {
      start,
      end,
      value,
    }
  }
}

#[allow(dead_code)]
impl Courses {
  pub fn new () -> Self {
    Courses {
      data: Vec::new(),
    }
  }

  pub fn random_courses (&mut self, n: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
      let start = rng.gen_range(1..=80);
      let duration = rng.gen_range(1..=20);
      let value = rng.gen_range(1..=10);
      let course = Course::new(start, start+duration, value);

      self.data.push(course);
    }
  }

  pub fn sort_courses (&mut self) {
    let mut n = self.data.len();
    let mut swapped = true;

    while swapped {
      swapped = false;
      for i in 1..n {
        if self.data[i-1].end > self.data[i].end {
          self.data.swap(i-1, i);
          swapped = true;
        }
      }
      n -= 1;
    }
  }

  pub fn print (&self) {
    for course in &self.data {
      println!("start: {}, end: {}, value: {}", course.start, course.end, course.value);
    }
  }
}