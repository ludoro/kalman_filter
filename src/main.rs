mod surface;
mod functions;

fn main() {

    //Small example to check that everything is working nicely: I have a plane which has already been
    // traslated to the origin and rotated such that it is parallel to the xy plane. Then I can call
    // my checking function to see if my point is in the bounded region.

    let point3D = surface::Point3D::new(1.0,1.0,0.0);
    let my_rect = surface::Rectangle::new(0.0,0.0,3.4,0.0,5.0,3.0);
    let result = functions::is_point_on_rectangle(point3D,my_rect);


    //Another example involving the rotation matrix:
    let to_be_rotated = surface::Rectangle::new(1.0,2.0,3.0,0.0,3.0,7.0);
    let rotation_matrix = functions::rotation_to_xy_plane(to_be_rotated);

    //The inverse is the tranpose
    let inverse = rotation_matrix.transpose();
    

}
