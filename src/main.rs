use std::collections::HashMap;

fn main() {
  let mut school = School::new();

  // Test case 1
  assert!(school.students.len() == 0);

  // Test case 2
  school.add(2, "Lee");
  let grades_1 = school.grades();
  assert!(grades_1 == [2]);

  school.add(3, "Nancy");
  let grades_2 = school.grades();
  assert!(grades_2 == [2, 3]);

  // Test case 3
  school.students.clear();
  school.add(4, "Bob");
  school.add(4, "Alice");
  school.add(5, "Tom");
  let match_students = school.grade(4);

  assert!(match_students == ["Alice", "Bob"]);
}

// fn main() {
//   let mut school = School::new();

//   // Test case 1
//   assert!(school.students.len() == 0);

//   // Test case 2
//   school.add("A", "Lee");
//   let grades_1 = school.grades();
//   assert!(grades_1 == ["A"]);

//   school.add("B+", "Nancy");
//   let grades_2 = school.grades();
//   assert!(grades_2 == ["A", "B+"]);

//   // Test case 3
//   school.students.clear();
//   school.add("A+", "Bob");
//   school.add("A+", "Alice");
//   school.add("B", "Tom");
//   let match_students = school.grade("A+");

//   assert!(match_students == ["Alice", "Bob"]);

//   println!("{:?}", school);
// }

#[derive(Debug)]
pub struct School<T> {
  // !TODO
  students: HashMap<String, T>
}

impl<T: PartialOrd + Ord + Copy> School<T> {
  pub fn new() -> School<T> {
      School { students: HashMap::new() }
  }

  pub fn add(&mut self, grade: T, student: &str) {
      self.students.insert(student.to_string(), grade);
  }

  pub fn grades(&self) -> Vec<T> {
    let mut grades: Vec<T> = Vec::new();

    for (_, &current_grade) in &(self.students) {
      let mut is_duplicate = false;
      for &grade in &grades {
        if current_grade == grade {
          is_duplicate = true;
          break;
        }
      }
      if !is_duplicate {
        grades.push(current_grade);
      }
    }
    grades.sort();

    grades
  }


  pub fn grade(&self, grade: T) -> Vec<String> {
    let mut students: Vec<String> = Vec::new();

    for (name, &current_grade) in &(self.students) {
      if current_grade == grade {
        students.push(name.to_string());
      }
    }
    students.sort();

    students
  }
}
