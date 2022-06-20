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

#[derive(Debug)]
pub struct School {
  // !TODO
  students: HashMap<String, u32>
}

impl School {
  pub fn new() -> School {
      School { students: HashMap::new() }
  }

  pub fn add(&mut self, grade: u32, student: &str) {
      self.students.insert(student.to_string(), grade);
  }

  pub fn grades(&self) -> Vec<u32> {
    let mut grades: Vec<u32> = Vec::new();

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


  pub fn grade(&self, grade: u32) -> Vec<String> {
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
