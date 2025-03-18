fn isEven(num:u32)->bool{
    if num % 2 == 0{
        return true;
    }
    return false;
}


fn main() {
    let a = 31;

    let res = isEven(a);

    println!("RES:{}",res)
}
