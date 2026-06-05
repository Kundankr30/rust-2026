fn main() {
    //arr_repeated();
    //access_arr();
    // get_len()
    // arr_iter();
    //indices_iter();
    // mutable_iter();
    // slices_arr()
    //new_vec();
    //iter_vec();
    //string_test()
    //string_concat()
    slic_arr()
}

// array

fn init_arr() {
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
}

fn arr_repeated() {
    let arr = [20; 15];
    println!("{:?}", arr);
}

fn access_arr() {
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr[2]);
    // println!("{:?}",arr[100]);
}

fn get_len() {
    let arr = [7, 8, 9, 5];
    let length = arr.len();
    println!("{}", length);
}

// int,char
// let a = 10;
// let b =a;

fn arr_iter() {
    let arr = ['a', 'b', 'c'];

    for element in arr {
        println!("{}", element);
    }

    let string_arr: [String; 2] = [String::from("debi"), String::from("priyanshu")];

    for element in string_arr {
        println!("{}", element);
    }

    // println!("{:?}",string_arr);
}

fn indices_iter() {
    let arr = [1, 2, 3];

    for i in 0..arr.len() {
        // arr[i] = arr[i] * 5;
        println!("index {} -> element {}", i, arr[i]);
    }
}

fn mutable_iter() {
    let mut arr = [1, 2, 3];

    for i in 0..arr.len() {
        arr[i] = arr[i] * 5;
        println!("index {} -> element {}", i, arr[i]);
    }
}

fn slices_arr() {
    let arr = [1, 2, 3];

    let slic = &arr[1..=2];

    println!("{:?}", slic);
}

fn new_vec() {
    // let maccrov = vec![1,4,5,6,7];

    let mut newv: Vec<i32> = Vec::new();

    newv.push(2);
    newv.push(5);
    newv.push(5);
    newv.push(5);
    newv.push(7);

    println!("{:?}", newv);

    //newv.pop();

    //println!("{:?}",newv);

    //newv.remove(0);

    //println!("{:?}",newv);
    // [_,_,_,_,_,_,_,_,_,_]

    // newv.clear();

    //println!("{:?}",newv);

    // newv.swap_remove(2);

    //println!("{:?}",newv);

    //newv.is_empty();
    //get(index)

    newv.insert(1, 10);

    println!("{:?}", newv);

    newv.remove(1);
    println!("{:?}", newv);
}

fn iter_vec() {
    let ivec = vec![1, 2, 3, 4, 5];

    for element in &ivec {
        println!("{:?}", element);
    }

    println!("{:?}", ivec);
}

// Strings !!!!!

fn string_test() {
    //let new_str: &str = "Hello World";
    // let new_new_str = String::from("No hello");

    //println!("{}",new_str);
    //println!("{}",new_new_str);

    // println!("{}",std::mem::size_of::<String>());

    // let new_newstr = "hello".to_string();
    //let indexc = new_newstr[2];
}

fn string_concat() {
    let a = String::from("hello");
    let b = String::from("world");

    let c = a.clone() + &b;

    println!("{}", c);
    println!("{}", a);

    for ele in a.chars() {
        println!("{}", ele);
    }
}

// Slices - ex - &str || &[i32]

fn slic_arr() {
    fn add_ele(s: &[i32]) -> i32 {
        let sum = s.iter().sum();
        return sum;
    }

    let arr = [1, 6, 5, 4, 7, 8, 9];

    // let somepart = &arr[1..];
    // let somemorepart = &arr[..5];

    // println!("{:?}",somepart);
    // println!("{:?}",somemorepart);
    println!("{:?}", arr);

    // println!("added ele of arra = {}",add_ele(&arr));

    // let some_arr =  &arr[1..100];
    // println!("{:?}",some_arr);

    match arr.get(1..100) {
        Some(s) => println!("{:?}", s),
        None => println!("You are actually out of bound !!"),
    }
}
