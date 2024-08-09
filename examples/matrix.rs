use concurrecny::Matrix;

fn main() {
    println!("default f32:{}", f32::default());
    println!("default f64:{}", f64::default());
    println!("default i8:{}", i8::default());
    println!("default i16:{}", i16::default());
    println!("default i32:{}", i32::default());
    println!("default usize:{}", usize::default());

    let a = Matrix::new([1, 2, 3, 4], 2, 2);
    let b = Matrix::new([1, 2, 3, 4], 2, 2);
    println!("a * b = {}", a * b);
}
