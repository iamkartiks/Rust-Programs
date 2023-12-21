#[derive(Debug)]
enum Staff{
    Professor(String),
    Janitor(String),
}

fn main(){
    let j1 = Staff::Janitor(String::from("Kartik"));
    let p1 = Staff::Professor(String::from("Anoneyaah"));

    println!("Janitor is {:?}", j1);
    println!("Profesor is {:?}", p1);
}