mod surface;
mod functions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_1() {
        let my_point = surface::Point3D::new(2.3,1.4,0.0);
        let my_rect = surface::Rectangle::new(0.0,0.0,1.0,0.0,3.0,3.0);
        let value = functions::is_point_on_rectangle(my_point,my_rect);
        assert!(value);
    }

    #[test]
    fn testing_2(){
        assert!(true);
    }
}
