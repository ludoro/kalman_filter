pub struct Plane{
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

pub struct Point3D{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Point2D{
    pub x: f64,
    pub y: f64,
}

pub struct Rectangle{
    pub plane: Plane,
    pub x_bound: f64,
    pub y_bound: f64,
}


impl Plane{
    pub fn new(a: f64,b: f64,c: f64,d: f64) -> Self{
        Plane { a: a, b:b, c:c, d:d}
    }
}

impl Point3D{
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Point3D{x:x, y:y, z:z}
    }
}

impl Point2D{
    pub fn new(x: f64, y: f64) -> Self{
        Point2D{x:x, y:y}
    }
}

impl Rectangle{
    pub fn new(a: f64,b: f64,c: f64,d: f64, x_bound: f64, y_bound:f64) -> Self{
        Rectangle { plane: Plane::new(a,b,c,d), x_bound: x_bound, y_bound: y_bound}
    }
}
