////FIRST PART WITH PROBLEMS
//Does not work now, Fix it with part2 and part3

/*
//struct --- course
struct Student<'a>{ //lifetime is shown with 'a
    name:String,
    courses:Vec<&'a Course<'a>>
}
impl<'a> Student<'a>
{
    fn new(name: &str) ->Student<'a>
    {
        Student{
            name:name.into(), //&str->String
            courses:Vec::new()
        }
    }
}

struct Course<'a>
{
    name:String,
    students: Vec<&'a Student<'a>>
}
impl<'a> Course<'a>
{
    fn new(name: &str)-> Course<'a>{
        Course{
            name:name.into(),
            students:Vec::new()
        }
    }
    fn add_student(&'a mut self,student:&'a mut Student<'a>){
        student.courses.push(self);
        self.students.push(student);//it is not allowed for normal case.
        //RefCell
    }
}

fn main(){
    let john=Student::new("John");
    let course=Course::new("Rust Course");
    course.add_student(john); //Rc
}
*/


////SECOND PART WITH SOLVED PROBLEMS
// with Rc- RefCell

/*
use std::rc::Rc;
use std::cell::RefCell;

struct Student{
    name: String,
    courses:Vec<Rc<RefCell<Course>>>
}

impl Student
{
    fn new(name: &str) ->Student
    {
        Student{
            name:name.into(), //&str->String
            courses:Vec::new()
        }
    }
}

struct Course
{
    name:String,
    students: Vec<Rc<RefCell<Student>>>
}
impl Course
{
    fn new(name: &str)-> Course{
        Course{
            name:name.into(),
            students:Vec::new()
        }
    }
    fn add_student(course:Rc<RefCell<Course>>,student:Rc<RefCell<Student>>){
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}

fn main(){
    let john=Rc::new(RefCell::new(Student::new("John")));
    let jane=Rc::new(RefCell::new(Student::new("Jane")));

    let course=Course::new("Rust Course");
    let magic_course=Rc::new(RefCell::new(course));
    
    //course.add_student(john); //Rc
    Course::add_student(magic_course.clone(),john);//add clone not to move for next line
    Course::add_student(magic_course,jane);
}
*/



//THIRD PART- WITH "ENROLLMENT Struct"
students-course-Vec<Enrollment{course,student}>

struct Student{
    name:String
}
impl Student{
    //shows the courses which is taken by student
    fn courses(&self,platform:Platform)->Vec<String>{
        platform.enrollments.iter() //e=enrollment
            .filter(|&e| e.student.name==self.name)//filter the student
            .map(|e| e.course.name.clone())//take each course
            .collect()//put everything in vector
    }
}

struct Course{
    name:String
}
struct Enrollment<'a>{  //Put a lifetime because all together are meaningfull.
    student:&'a Student, //Put a reference not to "move"
    course:&'a Course 
}
impl<'a> Enrollment<'a>{
    fn new(student: &'a Student,course: &'a Course)->Enrollment<'a>{
        Enrollment{student,course}
    }
}

struct Platform<'a>{ //table of enrollments
    enrollments: Vec<Enrollment<'a>>
}
impl<'a> Platform<'a>{
    fn new()->Platform<'a>{
        Platform{ enrollments: Vec::new() }
    }
    fn enroll(&mut self,student: &'a Student,course: &'a Course){
        self.enrollments.push(Enrollment::new(student,course));
    }
}

fn main(){
    let john=Student{name:"John".into()};
    let course=Course{name:"Intro to Rust".into()};
    let mut p=Platform::new();

    p.enroll(&john,&course);

    for c in john.courses(p){
        println!("John takes {}",c);
    }
}
