struct Rectangle{
    height:i32,
    width:i32,
}


fn main(){
    let rect1 = Rectangle{
        height : 30,
        width : 40,
    };

    // area_of_rect = area(&rect1);

    println!("Area is {}",area(&rect1));
}

fn area(rectangle:&Rectangle)->i32{
    rectangle.height * rectangle.width
}
