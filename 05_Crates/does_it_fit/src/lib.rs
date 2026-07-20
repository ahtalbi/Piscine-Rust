mod areas_volumes;

pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};

use areas_volumes::{
    square_area,
    rectangle_area,
    triangle_area,
    circle_area,
    cube_volume,
    sphere_volume,
    cone_volume,
    triangular_pyramid_volume,
    parallelepiped_volume,
};

pub fn area_fit((x, y): (usize, usize), kind: GeometricalShapes, times: usize, (a, b): (usize, usize)) -> bool {
    let sa = match kind {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };

    sa * times as f64 <= (x * y) as f64
}

pub fn volume_fit( (x, y, z): (usize, usize, usize), kind: GeometricalVolumes, times: usize, (a, b, c): (usize, usize, usize)) -> bool {
    let sv = match kind {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::TriangularPyramid => {
            triangular_pyramid_volume(triangle_area(a, b), c)
        }
        GeometricalVolumes::Parallelepiped => {
            parallelepiped_volume(a, b, c) as f64
        }
    };

    sv * times as f64 <= (x * y * z) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            area_fit((2, 4), GeometricalShapes::Rectangle, 100, (2, 1)),
            false
        );

        assert_eq!(
            area_fit((5, 5), GeometricalShapes::Triangle, 3, (5, 3)),
            true
        );

        assert_eq!(
            volume_fit((5, 5, 5), GeometricalVolumes::Sphere, 3, (2, 0, 0)),
            true
        );

        assert_eq!(
            volume_fit(
                (5, 7, 5),
                GeometricalVolumes::Parallelepiped,
                1,
                (6, 7, 4)
            ),
            true
        );
    }
}
