use specs::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub points: [[f32; 2]; 3],
}
impl Triangle {
    pub fn points_flat(self) -> [f32; 6] {
        let [[a, b], [c, d], [e, f]] = self.points;
        [a, b, c, d, e, f]
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Triangle {
            points: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0] ]
        }
    }
}

impl Component for Triangle {
    type Storage = VecStorage<Self>;
}