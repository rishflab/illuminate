use std::ops::{Add, Sub};

fn square(x: i32) -> i32 {
    x*x
}


//enum Command<T> {
//    translate(Point2d<T>),
//    shoot
//}



//
//#[derive(Debug, PartialEq)]
//struct State<T> {
//    ammo: u32,
//    position: Point2d<T>,
//}

#[derive(Debug, PartialEq)]
struct Point2d<T> where
    T: Add {
    x: T,
    y: T,
}

impl <T> Add for Point2d<T> where T: Add<Output = T> {

    type Output = Point2d<T>;

    fn add(self, other: Point2d<T>) -> Point2d<T> {
        Point2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(2), 4);
    }

    #[test]
    fn test_add_points() {
        let p1 = Point2d{x:2,y:3};
        let p2 = Point2d{x:1,y:4};
        let p3 = p1 + p2;
        assert_eq!(p3, Point2d {x: 3, y: 7});
    }

//
//    #[test]
//    fn calculate_new_state(state: State, commands: Vec<Command>) -> State {
//        for command in commands.iter() {
//            match command {
//                &Command::shoot => {state.ammo -= 1},
//                &Command::translate(x,y) => {state.position.x = },
//            }
//        }
//    }


}
