mod surface;
mod functions;

fn main() {

    //Small example to check that everything is woring nice: I have a plane which has already been
    // traslated to the origin and rotated such that it is parallel to the xy plane. Then I can call
    // my checking function to see if my point is in the bounded region.


    let point3D = surface::Point3D::new(1.0,1.0,0.0);
    let my_rect = surface::Rectangle::new(0.0,0.0,3.4,0.0,5.0,3.0);
    let result = functions::is_point_on_rectangle(point3D,my_rect);
    println!("{}", result);
    println!("Everything working fine.");



}
