use bincode::{deserialize, serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Message {
    Turn(f32),
    Thrust(f32),
    Shoot,
}


// impl Rpc for UpdatePlayer {
//     pub fn to_bytes(&self) -> &[u8] {
        
//     }

// }