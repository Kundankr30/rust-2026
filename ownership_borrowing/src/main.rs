fn main() {
    // println!("Hello, world!");

    // first_rule();
    // first_rule_break();
    // second_rule();
    // second_rule_breaks();
    // third_rule();
    third_rule_back();
    ownership_with_integers();
    // impl_string_copy();
    // first_rule_break_test();
}


// what is ownership?
// ownership is the rust's way of memory management. C, python , java they all have different memory management methods.
// c has malloc and free, python has garbage collector.
// so other language allow you to claim some memory and give you method to 
// clean up manually or  they clean it automatically.
// Rust's compiler tracks who owns what piece  of data and cleanup automaticlaly- during compile no runtime cost at all.

// when my program is compiled , the compiler makes sure all the values or all the data we have that should have owner,
// and when the owner or the task is done the data should be cleaned up.


// three rules of ownership
// 1. Each value has a single owner
fn first_rule(){
    // s is the owner of hello string
    // exactly one valriable owns one value
    let s = String::from("hello"); // String
    println!("{}", s);
}



fn first_rule_break(){
    // s1 is the owner of hello
    let s1 = String::from("hello");
    // here i am trying to assign value of s1 which is hello to s2
    // i am moving the ownership of hello from s1 to s2
    let s2 = s1;

    // s1 is no longer a valid variable

    // println!("{}", s1); 
    // hello is owned by s2 so s2 will print hello
    println!("{}", s2);
}

// rule 2: When owne goes out of the scope, the value will be dropped
fn second_rule(){
    {
        // this is another scope inside the scope of second rule function
        // create a value hello with an owner S
        let s = String::from("hello");
        // this will print hello cause s still owns hello
        println!("{}", s);
    }
}


// fn second_rule_breaks() {  
//     {
//         // this is another scope inside the scope of second rule function
//         // create a value hello with an owner S
//         let s = String::from("hello");
//         // this will print hello cause s still owns hello
//         println!("{}", s);
//     }
//     // here is s in the inner scope and this scope has no variable S
//     println!("{}", s); // this will give error because s is not valid here
// }


//rule 3: ownership can be transferred to another variable but only one variable can own a single value at a time
fn take_string(x: String){
    println!("ownership of {} is taken by take_ownership function", x);
}

// fn extra_func(y: String){
//     println!("ownership of {} is taken by extra_func function", y);
// }

// fn third_rule(){
//     // define a string hello and s is the owner of hello
//     let s = String::from("hello");
//     // here the value hello is moved to the func take_ownership and s is no nolonger the owner of hello
//     take_string(s);
//     // if i pass s again to any other function
//     extra_func(s); 
//     // println!("{}", s); 
// }


fn take_ownership_and_back(x: String) -> String {
    println!("ownership of {} is taken by take_ownership_and_back function", x);
    // here we are returning the value x which is hello and the ownership of hello is moved back to the caller

    // we create a string with coancatenation of x and world
    let y = x + " world";
    // if we modify the value x
    y
}

//hello was moved

fn third_rule_back(){
    let mut s = String::from("h");
    // here the value hello is moved to the func take_ownership_and_back and s is no nolonger the owner of hello
    if s.len() > 2 {
        take_string(s);
        // println!("value of s is {}", s);// should throw error
    } else {
        println!("{} is too short", s);
    }
    // s = take_ownership_and_back(s);
    // println!("value of x is {}", s);
    // after this there is value hello and s is the owner of hello again
    // println!("{}", s); 
}



// int is a data type which implements copy trait
// string was stored in heap memory without any specific size defined
// but inter is stored stack memory and easy to copy  cause we know the fixed size
// so copy types are those whoc implement copy trait, e.g. int, float, bool

// "hello"
// i8 ranges between -128 to 127
fn ownership_with_integers(){
    // let say we define a a value 5 and x is the owner
    let mut x =5;
    // ownership of 5 is moved Y - wrong
    //  the value 5 is copied to Y
    //now x and y are two different variable stored in stack
    // they eaach own a value which is 5
    let y = x;

    // we will increment x by 10
    x += 10;
    // x is no longer valid so x should not print
    println!("x: {}", x);
    println!("y: {}", y);
}


// how to copy a string so that we can pass it and still use it after passing to a function
// fn impl_string_copy(){
//     let s1 = String::from("hello");
//     // there are now two places in memory where is stored means hello is stored 2 times
//     // and s1 owns one hello and s2 owns another hello
//     let s2 = s1.clone();
//     take_string(s1);
//     // what if i need s1 here after this

//     // now we can copy the of s1 to  a variable s2 using clone method
//     // let s2 = s1.clone();
//     // now s1 is moved to take_ownership and s1 is no longer valid

//     println!(" trying to access s1 {}", s1); 
//     // println!(" trying to access s2 {}", s2); 
// }




// fn first_rule_break_test(){
//     // s1 is the owner of hello
//     let s1 = String::from("hello");
//     {
//         let s2 = s1;
//         println!("s2: {}", s2);
//     }
   
//     println!("s1: {}", s1);

//     // s1 is no longer a valid variable
//     // println!("{}", s1); 
//     // hello is owned by s2 so s2 will print hello
//     // println!("{}", s2);
// }