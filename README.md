rust-geos
=========

[![Build Status](https://travis-ci.org/mthh/rust-geos.svg?branch=master)](https://travis-ci.org/mthh/rust-geos)  

Rust bindings for [GEOS](https://trac.osgeo.org/geos/) C API.  
Work in progress (currently it's probably poorly designed, incomplete and containing beginners errors)  


#### Usage example
##### Constructing geometries from WKT :
```rust
extern crate geos;
use geos::GGeom;

fn main() {
    let gg1 = GGeom::new("POLYGON ((0 0, 0 5, 6 6, 6 0, 0 0))");
    let gg2 = GGeom::new("POLYGON ((1 1, 1 3, 5 5, 5 1, 1 1))");
	let gg3 = gg1.difference(&gg2);
	println!("{:?}", gg3.to_wkt());
}
```

##### Constructing geometries from coordinates :
```rust
extern crate geos;
// Theses convenience methods returns the same GGeom instances as in the previous example :
use geos::types_geom::{Point, LineString, Polygon};

fn main(){
    let pt = Point::new((22.33, 44.55));
    println!("{:?}", pt.to_wkt());

    let l_geom = LineString::new(&[(12.78, 78.08), (55.77, 77.55), (22.77, 88.99)]);
    println!("GeosGeom Linestring from coordinates : {:?}", l_geom.to_wkt());

    let exterior_ring = Ring::new(&[(0.0, 0.0), (0.0, 8.0), (8.0, 8.0), (8.0, 0.0), (0.0, 0.0)]);
    let interior = Ring::new(&[(1.0, 1.0), (4.0, 1.0), (4.0, 4.0), (1.0, 4.0), (1.0, 1.0)]);
    let poly_geom = Polygon::new(&exterior_ring, &[interior]);
    println!("GeosGeom Polygon from ring coordinates : {:?}", poly_geom.to_wkt());

	assert!(!poly_geom.contains(&pt));
	assert!(!l_geom.intersects(&poly_geom));

	// The underlying CoordinateSequence of point(s) can also be fetched :
    let coord_seq = pt.get_coord_seq().unwrap();
    let mut x = coord_seq.get_x(0);
    let mut y = coord_seq.get_y(0);
    assert_eq!(x, 22.33);
    assert_eq!(y, 44.55);
}

```

##### "Preparing" the geometries for faster predicates (intersects, contains, etc.) computation on repetitive calls :
```rust
extern crate geos;
use geos::{version, GGeom, PreparedGGeom};

fn main() {
    let g1 = GGeom::new("POLYGON ((0 0, 0 5, 5 5, 5 0, 0 0))");
    let g2 = GGeom::new("POLYGON ((1 1, 1 3, 5 5, 5 0, 1 1))");


    let pg1 = PreparedGGeom::new(&g1);
    let result = pg1.intersects(&g2);
	assert_eq!(result, true);

    let vec_geoms = vec![
        GGeom::new("POINT (1.3 2.4)"),
        GGeom::new("POINT (2.1 0.3)"),
        GGeom::new("POINT (3.1 4.7)"),
        GGeom::new("POINT (0.4 4.1)")
        ];
    for geom in &vec_geoms {
        if pg1.intersects(&geom) {
			...
			...
		}
    }
}
```


