enum Direction { // Direction is the name of the enum
    North, // variants of the enum
    South,
    East,
    West // Discriminants
}

// impl TYPE_NAME
impl Direction {
    fn is_vertical(&self) -> bool { // methods
        match self {
            Direction::North | Direction::South => true,
            _ => false
        }
    }

    fn is_vertical_without_self(dir: Direction) -> bool { // associated function
        match dir {
            Direction::North | Direction::South => true,
            _ => false
        }
    }

    fn print_directions(&self, other: &Self, another: Self) {} // ()
}

// fn is_vertical_without_self(dir: Direction) -> bool {
//     match dir {
//         Direction::North | Direction::South => true,
//         _ => false
//     }
// }

// Direction::true

// self -> Instance of a type. instance - any variable of that type
// &self
// &mut self

fn main() {
    let some_direction = Direction::North;
    let another_direction = Direction::East;

    let result = some_direction.is_vertical();
    let result = another_direction.is_vertical();

    let direction = Direction::South;

    let result = Direction::is_vertical_without_self(direction); //variable

    let direction = Direction::South;
    // some_direction.print_directions(&another_direction, direction);
    // a = 200;
    // b = 300;

    // a.gt(b) -> bool
    // a.lt(b) 
    // a.eq(b)

    // a + b;
    // a - b;
    // if a > 300 {}

    // a -> self

    // self + b;
    // self - b;
    // if self > 300 {}

    // println!("Hello, world!");
}



// let abc = String::from("Zaaaaaaaaaa")

// impl String {
//     fn from(string_slice: &str) -> Self {
//         String(string_slice)
//     }
// }