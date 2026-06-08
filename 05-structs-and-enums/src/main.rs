// Structs and ENUMs go brrrrrrrrr

fn main() {
    //init_struct();
    //access_and_ops();
    // impl_struct()
     data_enum();
}

fn init_struct() {
    struct Person {
        name: String,
        age: u32,
        email: String,
    }

    let biswash = Person {
        name: String::from("biswash"),
        age: 18,
        email: String::from("cse.25bcsi08@silicon.ac.in"),
    };

     let debi = Person {
        name: String::from("debi"),
        age: 19,
        email: String::from("cse.25bcsc14@silicon.ac.in"),
     };

    println!("name - {} , age - {}",biswash.name,biswash.age);
    println!("name - {} , age - {}",debi.name,debi.age);

}

fn access_and_ops() {
    struct Counter {
        value: i32,
        name: String,
    }

    let mut  c = Counter {
        value: 32,
        name: String::from("time"),
    };

    c.value += 1;
    c.value += 1;
    c.name = "debi".to_string();

    println!("{} {}",c.name,c.value);
}

fn shapes_of_struct() {
    // normal type
    struct Point {
        x: f64,
        y: f64,
    }

    let p = Point {
        x: 12.43,
        y: 15.5,
    };

    // tuple struct
   
    struct Meter(f64);
    struct Feet(f64);

    let height = Meter(10.5);
    let width = Feet(11.11);

    println!("{} {}", height.0,width.0);

    fn climb(h: Meter) {
        println!("debi can climb {} meters height",h.0);
    }

    // climb(width);

    // unit struct 

    struct Empty;

   // let e = Empty;
    
    
}


fn impl_struct() {
    struct Rectangle {
        length: i32,
        width: i32,
    }
    // associated functions for the struct
    // Vec::new -> naya empty vector , String::from("frint")
    
    impl Rectangle {

        // asociated functions 
        fn new(length: i32 , width: i32) -> Self { // Self -> Type
            Self {length , width}
        }

        // methods - this

        fn area(&self) -> i32 {  // struct itself
            self.length * self.width
        }

        fn perimeter(&self) -> i32 {
            2*(self.length+self.width)
        }

        
    }

    let rect = Rectangle::new(3,5);

        println!("area: {}",rect.area());
        println!("perimeter: {}", rect.perimeter());
    
}

fn self_ways() {
    // &self , &mut self, self

    struct Wallet { balance: f64}

    impl Wallet {
        fn balance(&self) -> f64 {
            self.balance
        }
        // wallet , amount 
        fn deposit(&mut self, amount: f64) {
           self.balance += amount;
        }

        fn withdraw(&mut self, amount: f64) {
            self.balance -= amount;
        }

    }
}

fn owned_self() {
    struct Inventory {
        parts: Vec<String>
    }
    impl Inventory {

        fn new() -> Inventory {
            Inventory {parts: Vec::new()}
        }
        fn repair(mut self, part: &str) -> Inventory {
            self.parts.push(part.to_string());
            self
        }       

    }
}

fn multi_impl() {
    struct Point {
        x: f64,//  -> 8 bytes
        y: f64, // -> 8 bytes
    }

    impl Point {
      //  fn new(x: f64, y: f64) -> Self { // Type
        //    Self {x,y}
       // }

      //  fn new(x: f64, y: f64) ->Point { // Type
      //     Point {x,y}
     //   }

         fn new(xcord: f64, ycord: f64) ->Point { // Type
           Point {x: xcord,y: ycord}
        }

        fn origin() -> Self {
            let x_origin = 0.0;
            let y_origin = 0.0;
           Self {x: x_origin,y: y_origin}
        }
    }

    impl Point {
        fn distance_from_origin(&self) -> f64 {
            (self.x*self.x + self.y*self.y).sqrt()
        }

        
    }

    let p = Point { // self 
        x: 5.0,
        y: 8.0,
    };

    let dist = p.distance_from_origin();
    
}

// :: -> associated funcs
// . -> methods

//  Enumerators / ENUMs

fn init_enum() {
    enum TrafficLight {
        Red,
        Green,
        Yellow,
    }
    // enum's value -> action
    let light = TrafficLight::Red;

    let action = match light {
        TrafficLight::Red => "stop",
        TrafficLight::Yellow => "gaadi_start",
        TrafficLight::Green => "bhaago",
    };
   
}

// VAriants can hold data in it as well
fn data_enum() {
    enum Move {
        Quit,
        Walk(i32,i32), // tuple
        SwordColor{r: u8,g: u8, b: u8}, // structs
    }

    let gameplay = vec! [
        Move::Quit,
        Move::Walk(10,20),
        Move::SwordColor { r: 255, g: 100, b: 50},
    ];

    for g in &gameplay {
        match g {
            Move::Quit => println!("QUIT"),
            Move::Walk(x,y) => println!("move to {},{}", x,y),
            Move::SwordColor{r,g,b} => println!("COlor of sword is consist of {},{},{}",r,g,b),
        } 
    }

    enum Shape {
        Circle(f64),
        Square(f64),
        Rectangle(f64,f64),
    }

    fn area(s: &Shape) -> f64 {
        match s {
            Shape::Circle(r) => 3.14*r*r,
            Shape::Square(si) => si*si,
            Shape::Rectangle(w,h) => w*h,
        }
    }

    let shapes = vec! [
        Shape::Circle(3.0),
        Shape::Square(5.0),
        Shape::Rectangle(4.0,6.0),
    ];
    
    for s in &shapes {
        println!("area = {}",area(s))
    }
}

fn impl_enum() {
   enum Shape {
        Circle(f64),
        Square(f64),
        Rectangle(f64,f64),
   }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
             Shape::Circle(r) => 3.14*r*r,
             Shape::Square(si) => si*si,
             Shape::Rectangle(w,h) => w*h,
            }
        }

        fn name(&self) -> &str {
            match self{
             Shape::Circle(_) => "circle",
             Shape::Square(_) => "square",
             Shape::Rectangle(_,_) => "rectangle",
            }
        }

        fn init_circle() -> Self {
            Self::Circle(1.0)
        }
        
    }
}

