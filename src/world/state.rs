use std::ops::{Add};

#[derive(Debug, PartialEq, Clone)]
struct State<T: Add + Clone> {
    ammo: u32,
    position: Point2d<T>,
}

#[derive(Debug, PartialEq, Clone)]
enum Command<T: Add> {
    Translate(Point2d<T>),
    Shoot,
}


impl <T: Clone + Add<Output=T>> State <T> {

    fn next_state(&self, command: Command<T>) -> State<T> {

        match command {
            Command::Shoot => {
                State {
                    ammo: self.ammo.clone() - 1,
                    position: self.position.clone(),
                }
            },
            Command::Translate(p) => {
                State {
                    ammo: self.ammo.clone(),
                    position: self.position.clone() + p,
                }
            }
        }
    }

    fn final_state (self, commands: Vec<Command<T>>) -> State<T> {

        let mut state = self.clone();

        for command in commands.iter() {
            state = state.next_state(command.clone());
        }

        state

    }
}


#[derive(Debug, PartialEq, Clone)]
struct Point2d<T: Add> {
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

impl<T> Point2d<T> where T: Add<Output = T> {

    fn new (x: T, y: T) -> Point2d<T> {
        Point2d {
            x: x,
            y: y,
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_points() {
        let p1 = Point2d {
            x:2,
            y:3,
        };
        let p2 = Point2d {
            x:1,
            y:4,
        };
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
                x: -1.4,
                y: 3.5,
            }
        };

        let c1 = Command::Translate(Point2d {x: -1.4, y: 3.5});

        let new_state = s1.next_state(c1);
        assert_eq!(new_state, s2 )
    }

    #[test]
    fn test_vec_update_state() {

        let mut cmds = Vec::new();
        cmds.push(Command::Translate(Point2d::new(1.2, 4.3)));
        cmds.push(Command::Translate(Point2d::new(5.3, 2.2)));

        let mut s = State {
            ammo: 0,
            position: Point2d {
                x: 0.0,
                y: 0.0,
            }
        };

        let f = State {
            ammo: 0,
            position: Point2d {
                x: 6.5,
                y: 6.5,
            }
        };

        for cmd in cmds.iter() {
            s = s.next_state(cmd.clone());
        }

        assert_eq! {s, f};

    }





}
