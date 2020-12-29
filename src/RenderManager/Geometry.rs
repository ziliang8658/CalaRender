pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Vec3f {
            x: x,
            y: y,
            z: z,
        };
    }
}