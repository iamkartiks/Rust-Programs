struct Student{
    name : String,
    gender : String,
}

impl Student{
    fn get_data(&self){
        println!("Name is {}",self.name);
        println!("Gender is {}",self.gender);
    }
}

fn main()
{
    let a1 = Student{
        name : String::from("Student1"),
        gender : String::from("Male"),
    };

    a1.get_data();
}