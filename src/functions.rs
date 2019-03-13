extern crate nalgebra as na;
extern crate num_traits;
use num_traits::pow;
use na::{Matrix3};
use super::surface;


// The problem is numerical: so I define a small epsilon which acts
// as the tollerance. When checking equality we will insteaf check if it is within tollerance.
// I just set it global as 10^-6, there can be further discussion about what the value of eps
// should be.
pub static EPS: f64 = 1.0/1000000.0;


//This is assuming the plane has already been traslated to the origin and rotated such that it is
// equivalent to xy plane.
pub fn is_point_on_rectangle(point3D: surface::Point3D, rectangle: surface::Rectangle) -> bool {

    // First of all, I check if the point is on the plane at all, to save up time in case it is not.
    let planarity = (rectangle.plane.a * point3D.x) + (rectangle.plane.b*point3D.y)
                + (rectangle.plane.c*point3D.z);
    if planarity.abs() < EPS{
        //It is on the plane, then I check if it is in the boundary region:
        if point3D.x.abs() < rectangle.x_bound && point3D.y.abs() < rectangle.x_bound {
            return true;
        }
        else {
            return false;
        }
    } else {
        return false;
    }
}

// These two are used when we are sure that the point is on the plane
pub fn global_to_local(point3D: surface::Point3D) -> surface::Point2D {
    let point2D = surface::Point2D::new(point3D.x,point3D.y);
    return point2D
}

pub fn local_to_global(point2D: surface::Point2D) -> surface::Point3D {
    let point3D = surface::Point3D::new(point2D.x,point2D.y, 0.0);
    return point3D;

}

// First of all, I traslate the plane to the origin. The translatio has to act on the 3D point as well
// to remain consistent. This is effectively the same as translating the axis.
pub fn translation_to_origin(rectangle: surface::Rectangle, point3D: surface::Point3D) -> (surface::Rectangle,surface::Point3D,f64) {
    if rectangle.plane.d.abs() < EPS {
        // the plane already passes through the origin.
        return (rectangle,point3D,0.0);
    } else {
        // If I have a plane ax + by + cz + d = 0, it intersects the z-axis at (0,0,-d/c), so
        // the translation t:(𝑥,𝑦,𝑧)→(𝑥,𝑦,𝑧−𝑑/𝑐) gives me the plane ax + by +cz = 0
        // that passes through the origin.
        let translation_value = rectangle.plane.d/rectangle.plane.c;
        let new_rectangle = surface::Rectangle::new(rectangle.plane.a,
                               rectangle.plane.b,rectangle.plane.c,0.0,rectangle.x_bound,rectangle.y_bound);
        let new_point = surface::Point3D::new(point3D.x,point3D.y,point3D.z - translation_value);

        // I need to return the translation value, it will be used for the inverse transformation.
        return(new_rectangle,new_point,translation_value);

    }

}

pub fn translation_from_origin(rectangle: surface::Rectangle, point3D: surface::Point3D, translation_value: f64) -> (surface::Rectangle,surface::Point3D) {
    //the invere is just (x,y,z) -> (x,y,z+d/c)
    let new_rectangle = surface::Rectangle::new(rectangle.plane.a,
                           rectangle.plane.b,rectangle.plane.c,translation_value*rectangle.plane.c
                           ,rectangle.x_bound,rectangle.y_bound);
    let new_point = surface::Point3D::new(point3D.x,point3D.y,point3D.z + translation_value);

    return (new_rectangle,new_point);
}


// I will now calcute the rotation matrix that makes the plane parallel to xy plane. Note that this method
// has to be applied when the term d is equal to 0, that means that we have already traslated it.
// The matrix that is the output of this function must then be multiplicated to the vector (a,b,c)
// containing the values of the coefficient of the plane and the (x,y,z) vector containing the coordinates of the point.


pub fn rotation_to_xy_plane(rectangle: surface::Rectangle) -> Matrix3<f64> {

    //After some math calculations, we find that the rotation matrix is the folling:
    // I took heavy ispiration from this post on stack:
    // https://math.stackexchange.com/questions/1167717/transform-a-plane-to-the-xy-plane
    let sum_squared = (pow(rectangle.plane.a,2) +
                       pow(rectangle.plane.b,2) + pow(rectangle.plane.c,2)).sqrt();
    let cos = rectangle.plane.c / sum_squared;
    let sin = ((pow(rectangle.plane.a,2) + pow(rectangle.plane.b,2)) / (pow(rectangle.plane.a,2) +
                       pow(rectangle.plane.b,2) + pow(rectangle.plane.c,2)) ).sqrt();
    let u1 = rectangle.plane.b / sum_squared;
    let u2 = - rectangle.plane.a / sum_squared;

    let m = Matrix3::new(cos+pow(u1,2)*(1.0-cos), u1*u2*(1.0-cos), u2*sin,
                         u1*u2*(1.0-cos),cos + pow(u2,2)*(1.0-cos),-u1*sin,
                         -u2*sin,u1*sin,cos);

    return m;
}

// The inverse of a rotation matrix is just its transpose.
//The library nalgebra has a function to calculate the transpose of a matrix called .transpose()
// Note: it also has a function for rotations given the angle. In our case the angle between plane
// and xy plane is: cos(theta) =  c / sqrt(a^2 + b^2 + c^2);
// I found a lot of different linear algebra libraries for Rust, however aparta from nalgebra
// the documentation was really poor so I have not had time to do a proper investigation of which
// one to use.
