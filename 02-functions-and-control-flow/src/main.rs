fn main() {
    // println!("Hello, world!");
    // let sum = add(3,2);



    // //simple if statement
    // let x = 5;
    // // only if
    // if x < 5 {
    //     println!("x is less than 5");
    // }



    // // if and else both
    // if x < 5 {
    //     println!("x is less than 5");
    // } else {
    //     println!("x is greater than or equal to 5");
    // }


    // // if and else chain
    // if x < 5 {
    //     println!("x is less than 5");
    // } else if x == 5 {
    //     println!("x is equal to 5");
    // } else  if x>5{
    //     println!("x is greater than 5");
    // }


    // // if as an expression
    // let y = if x<5 {0} else {1};


    //match

    // match example 
    let n = 2;

    // if n == 0 {
    //     println!("The word for {} is zero", n);
    // } else if n == 1 {
    //     println!("The word for {} is one", n);
    // } else {
    //     println!("The word for {} is default", n);
    // }


    // _ is called catch all pattern

    // let word = match n {
    //     0 => "zero",
    //     1 => "one",
    //     _ => "default"
    // };
    // println!("The word for {} is {}", n, word);



    // match n {
    //     0 => println!("The word for {} is zero or two", n),
    //     1 => println!("The word for {} is one", n),
    //     _ => println!("The word for {} is default", n)
    // };


    // let x = match n {
    //     0 => "zero",
    //     1 => 1,
    //     _ => "default"
    // };
    // println!("The word for {} is {}", n, x);



    // match n {
    //     0 => {
    //         println!("The word for {} is zero", n);
    //         println!("This is the end of the match arm");
    //     },
    //     1 => println!("The word for {} is one", n),
    //     _ => println!("The word for {} is default", n)
    // };


    //or pattern
    // match n {
    //     0 | 2 => println!("The word for {} is zero or two", n),
    //     1 => println!("The word for {} is one", n),
    //     _ => println!("The word for {} is default", n)
    // };


    // ranges
    // match n {
    //     0..=5 => println!("The word for {} is between 0 and 5", n),
    //     _ => println!("The word for {} is default", n)
    // };

    //(1, 4)
    //1..4
    //1..=4
    //0..4
    //0..=4

    //loops
    // types of loops : loop, while, for

    // for loop

    // for i in 0..5{
    //     println!("i is {}", i); 
    // }

    // for i in 0..=5{
    //     println!("i is {}", i);
    // }

    // //0, 2, 4, 6, 8
    // for i in (0..10).step_by(2){
    //     println!("i is {}", i);
    // }

    // //9, 8, 7, 6, 5, 4, 3, 2, 1, 0
    // for i in (0..10).rev().step_by(2){
    //     println!("i is {}", i);
    // }

    // // break and continue in for loop

    // for i in 0..10 {
    //     if i == 5 {
    //         break;
    //     }
    //     println!("i is {}", i);
    // }

    // for i in 0..10 {
    //     if i == 5 {
    //         continue;
    //     }

    //     // 10 line
    //     println!("i is {}", i);
    // }


    // while loop

    // while n<5{
    //     println!("n is {}", n);
    //     n+=1;
    //     // n = n + 1;
    //     // n += 1;
    //     // n++;
    // }


    //loop

    // loop{
    //     println!("This is an infinite loop");
    // }

    // return values using break in loop
    let mut c = 0;
    let result = loop {
        c+=1;
        if c == 5 {
            break;
        }
    };
    // println!("The result is {}", result);

    // either your compiler can define certainf ucntion that will work for specific types

    // // apple, banana, cherry
    // let items = vec!["apple", "banana", "cherry"];
    // for item in items.iter() {
    //     println!("item is {}", item);
    // }


    // // 0, apple  , 1, banana , 2, cherry
    // for (i, item) in items.iter().enumerate() {
    //     println!("item {} is {}", i, item);
    // }

}


//writw a custom function
fn add(a: i32, b:i32) -> i32 {
    // // early return 
    // if a>5 {
    //     return 0;
    // }
    let x = 5;
     a+b
}

