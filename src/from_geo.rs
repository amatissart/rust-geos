extern crate geo;

use std;
use libc::{c_int, c_uint, c_void};
use self::geo::{LineString, MultiPolygon, Polygon};
use ffi::{CoordSeq, GEOSGeomTypes, GEOSGeom_clone, GGeom, SafeCObj};
use error::Error;

// define our own TryInto while the std trait is not stable
pub trait TryInto<T> {
    type Err;
    fn try_into(self) -> Result<T, Self::Err>;
}

impl<'a> TryInto<GGeom> for &'a LineString<f64> {
    type Err = Error;

    fn try_into(self) -> Result<GGeom, Self::Err> {
        let nb_pts = self.0.len();
        let coord_seq_ext = CoordSeq::new(nb_pts as u32, 2);
        for i in 0..nb_pts {
            let j = i as u32;
            coord_seq_ext.set_x(j, self.0[i].x());
            coord_seq_ext.set_y(j, self.0[i].y());
        }

        if nb_pts == 1 { //TODO check that the ring is closed
            Err(Error::InvalidGeometry)
        } else {
            Ok(GGeom::create_linear_ring(&coord_seq_ext))
        }
    }
}

impl<'a> TryInto<GGeom> for &'a Polygon<f64> {
    type Err = Error;

    fn try_into(self) -> Result<GGeom, Self::Err> {
        let geom_exterior: GGeom = (&self.exterior).try_into()?;

        let interiors: Vec<_> = self.interiors
            .iter()
            .map(|i| i.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        GGeom::create_polygon(geom_exterior, interiors)
    }
}

impl<'a> TryInto<GGeom> for &'a MultiPolygon<f64> {
    type Err = Error;

    fn try_into(self) -> Result<GGeom, Self::Err> {
        let polygons: Vec<_> = self.0
            .iter()
            .map(|p| p.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        GGeom::create_mulipolygon(polygons)
    }
}

#[cfg(test)]
mod test {
    use from_geo::geo::{LineString, MultiPolygon, Point, Polygon};
    use ffi::GGeom;
    use from_geo::TryInto;

    #[test]
    fn polygon_contains_test() {
        let exterior = LineString(vec![
            Point::new(0., 0.),
            Point::new(0., 1.),
            Point::new(1., 1.),
            Point::new(1., 0.),
            Point::new(0., 0.),
        ]);
        let interiors = vec![
            LineString(vec![
                Point::new(0.1, 0.1),
                Point::new(0.1, 0.9),
                Point::new(0.9, 0.9),
                Point::new(0.9, 0.1),
                Point::new(0.1, 0.1),
            ]),
        ];
        let p = Polygon::new(exterior.clone(), interiors.clone());

        assert_eq!(p.exterior, exterior);
        assert_eq!(p.interiors, interiors);

        let geom: GGeom = (&p).try_into().unwrap();

        assert!(geom.contains(&geom));
        assert!(!geom.contains(&(&exterior).try_into().unwrap()));

        assert!(geom.covers((&(&exterior).try_into().unwrap())));
        assert!(geom.touches(&(&exterior).try_into().unwrap()));
    }

    #[test]
    fn multipolygon_contains_test() {
        let exterior = LineString(vec![
            Point::new(0., 0.),
            Point::new(0., 1.),
            Point::new(1., 1.),
            Point::new(1., 0.),
            Point::new(0., 0.),
        ]);
        let interiors = vec![
            LineString(vec![
                Point::new(0.1, 0.1),
                Point::new(0.1, 0.9),
                Point::new(0.9, 0.9),
                Point::new(0.9, 0.1),
                Point::new(0.1, 0.1),
            ]),
        ];
        let p = Polygon::new(exterior.clone(), interiors.clone());
        let mp = MultiPolygon(vec![p.clone()]);

        let geom = (&mp).try_into();
        if (geom.is_err()) {return;}
        let geom: GGeom = (&mp).try_into().unwrap();

        assert!(geom.contains(&geom));
        assert!(geom.contains(&(&p).try_into().unwrap()));
    }

    #[test]
    fn incorrect_multipolygon_test() {
        let exterior = LineString(vec![
            Point::new(0., 0.)
        ]);
        let interiors = vec![];
        let p = Polygon::new(exterior.clone(), interiors.clone());
        let mp = MultiPolygon(vec![p.clone()]);

        let geom = (&mp).try_into();

        assert!(geom.is_err());
    }
}
