mod shape;

use std::f64::consts::PI;
use crate::shape::{area, Circle, Square, Triangle};
// //矩形结构体
// struct Rectangle<u32> {
//     width: u32,
//     height: u32
// }
// //圆形
// struct Circle<T>{
//     radius: T
// }
//
// //定义计算图形面积的特性
// trait Area {
//     fn CalculateArea(&self) -> f64;
// }
//
// impl Area for Rectangle<u32> {
//     fn CalculateArea(&self) -> f64 {
//         (&self.width * &self.height) as f64
//     }
// }
// impl Area for Circle<f64>{
//     fn CalculateArea(&self) -> f64 {
//         (PI*&self.radius*&self.radius) as f64
//     }
// }

fn main() {
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50
    // };
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.CalculateArea()
    // );
    // let cir1 = Circle{
    //     radius:2.0
    // };
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     cir1.CalculateArea()
    // );
    //
    //

    let circle = Circle { r: 10u8 };
    println!("{:#?} \n area {}\n", circle, area(&circle));

    let triangle = Triangle {
        a: 3u32,
        b: 4.0f32,
        c: 5.00000000000001f64,
    };
    println!("{:#?} \n area {}\n", triangle, area(&triangle));

    let square = Square { a: 10u32 };
    println!("{:#?} \n area {}\n", square, area(&square));
}





