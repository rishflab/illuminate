use bincode::{deserialize, serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Message {
    Turn(f32),
    Thrust(f32),
    Shoot,
}