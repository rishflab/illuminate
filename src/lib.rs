use std::ops::{Add};

fn square(x: i32) -> i32 {
    x*x
}

#[derive(Debug, PartialEq, Clone)]
enum Command<T: Add> {
    Translate(Point2d<T>),
    Shoot,
}


#[derive(Debug, PartialEq, Clone)]
struct State<T: Add> {
    ammo: u32,
    position: Point2d<T>,
}

#[derive(Debug, PartialEq, Clone)]
struct Point2d<T: Add> {
    x: T,
    y: T,
}

impl <T> std::ops::Add for Point2d<T> where T: Add<Output = T> {

    type Output = Point2d<T>;

    fn add(self, other: Point2d<T>) -> Point2d<T> {
        Point2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


fn update_state<T:Add<Output = T>>(state: State<T>, command: Command<T>) -> State<T> {

    match command {
        Command::Shoot => {
            State {
                ammo: state.ammo - 1,
                position: state.position,
            }
        },
        Command::Translate(p) => {
            State {
                ammo: state.ammo,
                position: state.position + p,
            }
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

    #[test]
    fn test_add_point_to_state() {
        let s1 = State {
            ammo: 10,
            position: Point2d {
                x: 0.0,
                y: 0.0,
            }
        };

        let p1 = Point2d{x:2.1, y:3.2};

        let s2 = State {
            ammo: 10,
            position: p1 + s1.position,
        };
        assert_eq!(s2, State {ammo: 10, position: Point2d {x: 2.1, y: 3.2}});
    }

    #[test]
    fn test_update_state() {

        let s1 = State {
            ammo: 10,
            position: Point2d {
                x: 0.0,
                y: 0.0,
            }
        };

        let s2 = State {
            ammo: 10,
            position: Point2d {
                x: -1.4, y: 3.5
            }
        };

        let c1 = Command::Translate(Point2d {x: -1.4, y: 3.5});

        let new_state = update_state(s1, c1);
        assert_eq!(new_state, s2 )
    }





}
