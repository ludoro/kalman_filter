mod surface;
mod functions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_is_point_on_rectangle_true() {
        let my_point = surface::Point3D::new(2.3,1.4,0.0);
        let my_rect = surface::Rectangle::new(0.0,0.0,1.0,0.0,3.0,3.0);
        let value = functions::is_point_on_rectangle(my_point,my_rect);
        assert!(value);
    }

    #[test]
    fn testing_is_point_on_rectangle_false_bound(){
        let my_point = surface::Point3D::new(2.3,1.4,0.0);
        let my_rect = surface::Rectangle::new(0.0,0.0,1.0,0.0,1.0,1.0);
        let value = functions::is_point_on_rectangle(my_point,my_rect);
        assert!(!value);
    }

    #[test]
    fn testing_is_point_on_rectangle_false_plane(){
        let my_point = surface::Point3D::new(2.3,1.4,0.0);
        let my_rect = surface::Rectangle::new(1.0,0.0,1.0,0.0,3.0,3.0);
        let value = functions::is_point_on_rectangle(my_point,my_rect);
        assert!(!value);
    }

    #[test]
    fn testing_global_to_local(){
        let my_point = surface::Point3D::new(2.3,1.4,0.0);
        let expected = surface::Point2D::new(2.3,1.4);
        let returned = functions::global_to_local(my_point);
        assert!( ((expected.x-returned.x).abs() < functions::EPS) &&
                 ((expected.y-returned.y).abs() < functions::EPS));
    }

    #[test]
    fn testing_local_to_global(){
        let my_point = surface::Point2D::new(2.3,1.4);
        let expected = surface::Point3D::new(2.3,1.4,0.0);
        let returned = functions::local_to_global(my_point);
        assert!( ((expected.x-returned.x).abs() < functions::EPS) &&
                 ((expected.y-returned.y).abs() < functions::EPS) &&
                 ((expected.z-returned.z).abs() < functions::EPS));
    }

    #[test]
    fn testing_traslation_to_origin(){
        let my_rect = surface::Rectangle::new(2.3,3.1,3.4,3.4,3.0,3.0);
        let my_point = surface::Point3D::new(2.3,1.4,3.4);
        let my_rect_expected = surface::Rectangle::new(2.3,3.1,3.4,0.0,3.0,3.0);
        let my_point_expected = surface::Point3D::new(2.3,1.4,2.4);
        let translation_expected = 1.0;
        let (my_rect_returned,my_point_returned, translation) =
                                        functions::translation_to_origin(my_rect,my_point);

        assert!( ((my_rect_expected.plane.d).abs() < functions::EPS) &&
                 ((my_point_expected.z-my_point_returned.z).abs() < functions::EPS) &&
                 ((translation_expected-translation).abs() < functions::EPS));
    }


    #[test]
    fn testing_traslation_to_origin_plane_already_in_origin(){
        let my_rect = surface::Rectangle::new(2.3,3.1,3.4,0.0,3.0,3.0);
        let my_point = surface::Point3D::new(2.3,1.4,3.4);
        let my_rect_expected = surface::Rectangle::new(2.3,3.1,3.4,0.0,3.0,3.0);
        let my_point_expected = surface::Point3D::new(2.3,1.4,3.4);
        let (my_rect_returned,my_point_returned, translation) =
                                        functions::translation_to_origin(my_rect,my_point);
        assert!( ((my_rect_expected.plane.d).abs() < functions::EPS) &&
                 ((my_point_expected.z - my_point_returned.z).abs() < functions::EPS));

    }

    #[test]
    fn testing_translation_from_origin(){
        let my_rect = surface::Rectangle::new(2.3,3.1,3.4,0.0,3.0,3.0);
        let my_point = surface::Point3D::new(2.3,1.4,3.4);
        let translation_value = 1.0;
        let d_value_expected = translation_value*my_rect.plane.c;
        let z_expected = my_point.z + translation_value;
        let (my_rect_returned, my_point_returned) =
                    functions::translation_from_origin(my_rect,my_point,translation_value);

        assert!(((my_rect_returned.plane.d- d_value_expected).abs() < functions::EPS) &&
                 ((z_expected - my_point_returned.z).abs() < functions::EPS));

    }
}
