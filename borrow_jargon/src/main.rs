fn something(s1:&String){
    println!("{}",s1)
}

fn main() {
    let mut a = String::from("Anish Khamkar");
    // something(&a);
    // println!("{}",a);
    print_something(&mut a);
    println!("{}",a)
}


//then if you want to mutate the borrowed string


fn print_something(s2:&mut String){
    let a = String::from("addedString");

    for i in a.chars(){
        s2.push(i);
    }
}