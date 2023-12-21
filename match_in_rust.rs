enum Staff {
    Janitor,
    Professor,
    Teacher,
    Student,
}

fn staff_category(category: Staff) {
    match category {
        Staff::Janitor => {
            println!("Staff is Janitor");
        }
        Staff::Teacher => {
            println!("Staff is Teacher");
        }
        Staff::Professor => {
            println!("Staff is Professor");
        }
        Staff::Student => {
            println!("Person is not a staff, they are a student!");
        }
    }
}

fn main() {
    let staff = Staff::Teacher;
    staff_category(staff);
}
