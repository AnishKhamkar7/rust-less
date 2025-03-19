
fn create_string(){ 
    let s1 = String::from("new string");
    let s2 = s1;

    println!("new owner:{}",s1)
}

fn do_something(s:String){
    println!("{}",s)
}
//s1 is moveed to s2 ... so you cannot access s1 after the value is moved to s2
fn main() {
    println!("Hello, world!");

    let a = String::from("new");

    do_something(a);

    print!("{}",a)  
}
