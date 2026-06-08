use std::{collections::{HashMap, VecDeque}, hash::Hash};
// Traits
// Trait is a set of behaviours/features/functionalities that is defined/implemented for types who wish to 
// incorporate the said behaviours/features/functionalities.

// trait keyword

// trait TRAIT_NAME
trait Greet {
    fn hello(); // associated fun
    fn hi(); // associated fun
    // fn idk(&self); // method

    // default implementation
    fn default() {
        println!("I AM A DEFAULT")
    }
}

struct Human;
struct Robot;

// impl TRAIT_NAME for TYPE_NAME
impl Greet for Human {
    fn hello() {
        println!("I say Hello");
    }

    fn hi() {
        println!("I say HI");
    }
}

impl Greet for Robot {
    fn hello() {
        println!("Beep Boop");
    }

    fn hi() {
        println!("I am Debi");
    }
}

trait Size {
    fn max_size(&self) -> usize;
}

// i32
impl Size for i32 {
    fn max_size(&self) -> usize {
        32
    }
}

// i8
impl Size for i8 {
    fn max_size(&self) -> usize {
        8
    }
}

impl Size for HashMap<i32, String> {

    fn max_size(&self) -> usize {
        700
    }
}




// Orphan Rule - You can only implement a trait for a type if either the trait or the type is defined by you in your crate.

trait Shape {
    fn shape(&self) -> String {
        String::from("Shape")
        // // "Shape".to_owned() // converts an &str to String 
        // "Shape".into()
        // to_string() -> Converts anything that implements the display trait(anything that can be printed) into a String
    }

    fn has_sides(&self) -> bool;
    fn num_sides() -> u8;
    fn compare_perimeter(&self, other_shape: &Self);
}

struct Circle(f64);
struct Square;
struct Pentagon;
struct Triangle;

impl Shape for Circle {
    fn shape(&self) -> String {
        String::from("Circle")
    }

    fn has_sides(&self) -> bool {
        false
    }

    fn num_sides() -> u8 {
        0
    }

    fn compare_perimeter(&self, other_shape: &Self) {
        let per_1 = 3.14 * self.0 * 2.0;
        let per_2 = 3.14 * other_shape.0 * 2.0;

        if per_1 > per_2 {
            println!("CIRCLE ONE HAS GREATER PERIMETER - {}", per_1);
        } else {
            println!("CIRCLE TWO HAS GREATER PERIMETER - {}", per_2);
        }
    }
}

// let num: i32 = 400;


impl Greet for Circle {
    fn hello() {
        println!("I AM A CIRCLE")
    }

    fn hi() {
        
    }
}

trait Abc {
    fn whatever();
}

trait Xyz {
    fn whatever();
}

struct ScapeGoat;

impl Abc for ScapeGoat {
    fn whatever() {
        println!("I AM FROM TRAIT ABC");
    }
}

impl Xyz for ScapeGoat {
    fn whatever() {
        println!("I AM FROM TRAIT XYZ");
    }
}

fn main() {
    <ScapeGoat as Abc>::whatever(); // -> I am calling whatever() on a type that is an implementaion of the trait Abc;
    <ScapeGoat as Xyz>::whatever();


    // println!("{}", num.max_size());
    // let circle_1 = Circle(5.0);
    // let circle_2 = Circle(15.0);
    
    // println!("Shape is - {}", circle_1.shape());
    // println!("Does a {} have sides? - {}", circle_1.shape(), circle_1.has_sides());
    
    // println!("Number of sides in a {} - {}", circle_1.shape(), Circle::num_sides());
    // circle_1.compare_perimeter(&circle_2);
    
    // Circle::hello();
    
}