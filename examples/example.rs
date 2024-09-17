fn main() {
    let a = plates::PlateCode::encode("hello world123", true);
    println!("{}", a);
    println!("{}", plates::PlateCode::decode(&a));

    let b = plates::PlateCode::encode("hello world456", false);
    println!("{}", b);
    println!("{}", plates::PlateCode::decode(&b));

    println!("{}", plates::PlateCode::hash("test", true));
    println!("{}", plates::PlateCode::hash("test", false));
}
