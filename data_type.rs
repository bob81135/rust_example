fn main() {
    let x:i32 = 5;
    let y:f32 = 5.2;
    let z:bool = false;
    let tup:(i32,f32,bool) = (x,y,z);
    let list:[i32;5] = [3;5];
    println!("int32 x={}", x);
    println!("float32 y={}", y);
    println!("bool z={}", z);
    println!("tuple tup={},{},{}", tup.0,tup.1,tup.2);
    println!("list list={}", list[0]);
}