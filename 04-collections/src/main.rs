use std::collections::HashMap;

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
    //slic_arr()
    //init_tuple()
    //access_tuple()
    match_tuple()
   // init_map()
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

// Tuples !!

fn init_tuple() {
    let tup = (1,2,3,4,5);

    println!("{:?}",tup);

    let tup1 = (1,2,'c',true,1.68);

    println!("{:?}",tup1);

    let tup2 = ();
    println!("{:?}",tup2);

    let tup3 = (1,[2,5,6],([5,6,7],true,'c',5.7));

    println!("{:?}", tup3);
}

fn access_tuple() {
    let tup = (1,2,5,7,8);

    println!("{}",tup.1);


    let (a,b,c,d,e) = tup;

    println!("{}, {}, {}", a,b,c);

    let tup1 =  (5,6,7,8,9);
    
    let (..,f) = tup1; // a,..,b - a,.. - ..,c

    println!("{}",f);
    println!("{:?}",tup1);

    // multiple return without struct 
    fn return_mul() -> (i32,bool) {
        return (5,true)
    }
}

fn match_tuple() {
    //let p = (0,0);
  
    /* match p {
        (0,0) => println!("origin is here"),
        (1,2) => println!("somethin"),
        (0,y) => println!("here is y-> {}",y),
        (x,0) => println!("here is x-> {}",x),
        (x,y) => println!("both of them {} and {}",x,y),
}
     */

    

    //println!("(i32,bool,f64) -> {} bytes", std::mem::size_of::<(i32,bool,f64)>()); // 4,1,8 = 13

    // byte - 0 1 2 3 4 5 6 7 8 9 10 11
    //        [ i32 ][b][...][ f64 ]

    let tup = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    let (..,a) = tup;
    println!("{}",a);
    //println!("{:?}",tup);
}

// Hash Maps !!!

fn init_map() {
    let mut new_map: HashMap<i32,String> = HashMap::new();

    new_map.insert(1,"biswa".to_string());
    new_map.insert(2,"satyam".to_string());
    new_map.insert(4,"rakesh".to_string());

    

    new_map.insert(2,"kundan".to_string());
    println!("{:?}",new_map);

    let a = new_map.get(&2);
    println!("{:?}",a);

    let key_one = 7;
    let key_two = 2;

    println!("contains {} -> {}",key_one,new_map.contains_key(&key_one));
    
    println!("contains {} -> {}",key_two,new_map.contains_key(&key_two));

    let rem = new_map.remove(&4);

    println!("{:?}",new_map);
    println!("removed -> {:?}",rem);

    for key in new_map.keys() {
        println!("keys -> {}",key);
    }

     for (key,value) in &new_map {
         println!("keys -> {}, value -> {}",key,value);
     }

}
