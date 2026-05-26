const PI: f32 = 3.14;

fn main() {
    // // integers
    // let a = 200; // immutable i32
    // let mut b = 100; // mutable i32

    // let c: u8 = 30;
    // let d: u128 = 10000000000000;

    // // let e: u8 = 300;

    // let f = -1;

    // // floats
    // let g = -1.983;
    // let h = 8.983;

    // let i: f32 = 8.983;

    // // booleans
    // let t = true;
    // let fal = false;
    // let t = !false;

    // // char
    // let char_a = 'A';
    // let fancy = 'ň';
    // let emoji = '😄';

    // let char_a_bytes = "A".as_bytes();
    // let char_fancy_bytes = "ň".as_bytes();
    // let char_emoji_bytes = "😄".as_bytes();

    // println!("{}-{}-{}, {:?}, {:?}", char_a, fancy, emoji, char_a_bytes, char_fancy_bytes);
    // println!("{:?}", char_emoji_bytes);

    // U+00F1
    // U+0041 - A


    // macro
    // println!("{}", PI);



    // type conversions
    // as

    let number = 65;
    let float_number = number as f64;
    // let char_number = number as u8 as char;
    // let char_number = number as char;

    let float = 5.68;
    let float_int = float as i32;

    let int_32 = 10000; // i32
    let int_u8 = int_32 as u8; // u8: 500 - 256 = 244

    // 1000 - 256 = 744
    // 744 - 256 = 488 
    // 488 - 256 = 232 

    // let t = true;
    // let f = false;
    let t = !false; // true
    let int_bool = t as i32;
    // let int_bool = f as i32;

    println!("{}", f32::EPSILON);
    println!("{}", f64::EPSILON);

    let point_one = 0.1;
    let point_two = 0.2;
    let res = point_one + point_two;

    println!("{}", res);

    let neg = -300;
    let cast = neg as u8;

    println!("{}", cast);
}
