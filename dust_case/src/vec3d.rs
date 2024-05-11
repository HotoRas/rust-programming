struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3d {
    pub fn to_tuple(&self) -> (f64, f64, f64) { (x, y, z) }

    pub fn new(_x: f64, _y: f64, _z: f64) -> Self {
        Self {
            x: _x, y: _y, z: _z,
        }
    }

    pub fn add(&self, src: &Vec3d) -> Vec3d {
        self.x += src.x; self.y += src.y; self.z += src.z;
        return self.clone();
    }
}