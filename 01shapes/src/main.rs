struct ShapeRect {
    width:f32,
    height:f32
}

impl ShapeRect {
    fn area(&self) -> f32{
        self.width * self.height
    }
}

fn main(){
    let shape = ShapeRect {
        width:12.0,
        height:12.0
    };

    println!("Area of the Shape is :{}",shape.area())
}