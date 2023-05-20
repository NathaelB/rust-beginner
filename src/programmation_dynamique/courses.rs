use rand::Rng;

pub struct Course {
  pub start: usize,
  pub end: usize,
  pub value: usize,
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

  pub fn print (&self) {
    println!("start: {}, end: {}, value: {}", self.start, self.end, self.value);
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

  pub fn push (&mut self, course: Course) -> &mut Self {
    self.data.push(course);
    self
  }

  pub fn sort_courses (&mut self) -> &mut Self {
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

    self
  }

  pub fn calcul_pred (&self) -> Vec<isize> {
    let mut pred: Vec<isize> = vec![-1; self.data.len()];

    for i in 0..self.data.len() {
      let current_start = self.data[i].start;
      let mut latest_end: isize = -1;

      for j in 0..self.data.len() {
        if j != i
          && i != 0
          && self.data[j].end < current_start
          && self.data[j].end as isize > latest_end
        {
          latest_end = self.data[j].end as isize;
          pred[i] = j as isize;
        }
      }
    }

    pred
  }

  pub fn print (&self) {
    for course in &self.data {
      println!("start: {}, end: {}, value: {}", course.start, course.end, course.value);
    }
  }
}