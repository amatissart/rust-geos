use libc::{atexit, c_char, c_double, c_int, c_uint, c_void, size_t};
use std::sync::{Once, ONCE_INIT};
use std::ffi::{CStr, CString};
use std::{ptr, str};
use error::{Error, Result as GeosResult};
use std;

#[link(name = "geos_c")]
extern "C" {
    fn initGEOS() -> *mut c_void;
    fn GEOSversion() -> *const c_char;
    fn finishGEOS() -> *mut c_void;

    // API for reading WKT :
    fn GEOSWKTReader_create() -> *mut GEOSWKTReader;
    fn GEOSWKTReader_destroy(reader: *mut GEOSWKTReader);
    fn GEOSWKTReader_read(reader: *mut GEOSWKTReader, wkt: *const c_char) -> *mut c_void;

    // API for writing WKT :
    fn GEOSWKTWriter_create() -> *mut GEOSWKTWriter;
    fn GEOSWKTWriter_destroy(writer: *mut GEOSWKTWriter);
    fn GEOSWKTWriter_write(writer: *mut GEOSWKTWriter, g: *const c_void) -> *const c_char;
    fn GEOSWKTWriter_setRoundingPrecision(writer: *mut GEOSWKTWriter, precision: c_int);

    fn GEOSFree(buffer: *mut c_void);

    fn GEOSPrepare(g: *const c_void) -> *mut GEOSPreparedGeometry;
    fn GEOSGeom_destroy(g: *mut c_void);
    pub fn GEOSGeom_clone(g: *const c_void) -> *mut c_void;

    fn GEOSCoordSeq_create(size: c_uint, dims: c_uint) -> *mut GEOSCoordSequence;
    fn GEOSCoordSeq_destroy(s: *mut GEOSCoordSequence);
    fn GEOSCoordSeq_clone(s: *const GEOSCoordSequence) -> *mut GEOSCoordSequence;
    fn GEOSCoordSeq_setX(s: *mut GEOSCoordSequence, idx: c_uint, val: c_double) -> c_int;
    fn GEOSCoordSeq_setY(s: *mut GEOSCoordSequence, idx: c_uint, val: c_double) -> c_int;
    fn GEOSCoordSeq_setZ(s: *mut GEOSCoordSequence, idx: c_uint, val: c_double) -> c_int;
    fn GEOSCoordSeq_getX(s: *const GEOSCoordSequence, idx: c_uint, val: *mut c_double) -> c_int;
    fn GEOSCoordSeq_getY(s: *const GEOSCoordSequence, idx: c_uint, val: *mut c_double) -> c_int;
    fn GEOSCoordSeq_getZ(s: *const GEOSCoordSequence, idx: c_uint, val: *mut c_double) -> c_int;

    // Geometry must be a LineString, LinearRing or Point :
    fn GEOSGeom_getCoordSeq(g: *const c_void) -> *mut GEOSCoordSequence;

    // Geometry constructor :
    pub fn GEOSGeom_createPoint(s: *const GEOSCoordSequence) -> *mut c_void;
    pub fn GEOSGeom_createLineString(s: *const GEOSCoordSequence) -> *mut c_void;
    pub fn GEOSGeom_createLinearRing(s: *const GEOSCoordSequence) -> *mut c_void;
    fn GEOSGeom_createPolygon(
        shell: *mut c_void,
        holes: *mut *mut c_void,
        nholes: c_uint,
    ) -> *mut c_void;
    fn GEOSGeom_createCollection(
        t: c_int,
        geoms: *mut *mut c_void,
        ngeoms: c_uint,
    ) -> *mut c_void;

    // Functions acting on GEOSGeometry :
    fn GEOSisEmpty(g: *const c_void) -> c_int;
    fn GEOSisSimple(g: *const c_void) -> c_int;
    fn GEOSisRing(g: *const c_void) -> c_int;
    fn GEOSHasZ(g: *const c_void) -> c_int;
    fn GEOSisClosed(g: *const c_void) -> c_int;
    fn GEOSisValid(g: *const c_void) -> c_int;

    fn GEOSGeomToWKT(g: *const c_void) -> *const c_char;
    fn GEOSGeomFromWKB_buf(wkb: *const u8, size: size_t) -> *mut c_void;
    fn GEOSGeomToWKB_buf(g: *const c_void, size: *mut size_t) -> *mut u8;
    fn GEOSGeomTypeId(g: *const c_void) -> c_int;
    fn GEOSArea(g: *const c_void, area: *mut c_double) -> c_int;
    fn GEOSLength(g: *const c_void, distance: *mut c_double) -> c_int;
    fn GEOSDisjoint(g1: *const c_void, g2: *const c_void) -> c_int;
    fn GEOSTouches(g1: *const c_void, g2: *const c_void) -> c_int;
    fn GEOSIntersects(g1: *const c_void, g2: *const c_void) -> c_int;
    fn GEOSCrosses(g1: *const c_void, g2: *const c_void) -> c_int;
    fn GEOSWithin(g1: *const c_void, g2: *const c_void) -> c_int;
    fn GEOSContains(g1: *const c_void, g2: *const c_void) -> c_int;
    fn GEOSOverlaps(g1: *const c_void, g2: *const c_void) -> c_int;
    fn GEOSEquals(g1: *const c_void, g2: *const c_void) -> c_int;
    fn GEOSEqualsExact(g1: *const c_void, g2: *const c_void, tolerance: c_double) -> c_int;
    fn GEOSCovers(g1: *const c_void, g2: *const c_void) -> c_int;
    fn GEOSCoveredBy(g1: *const c_void, g2: *const c_void) -> c_int;

    fn GEOSBuffer(g: *const c_void, width: c_double, quadsegs: c_int) -> *mut c_void;
    fn GEOSEnvelope(g: *const c_void) -> *mut c_void;
    fn GEOSConvexHull(g: *const c_void) -> *mut c_void;
    fn GEOSBoundary(g: *const c_void) -> *mut c_void;
    fn GEOSGetCentroid(g: *const c_void) -> *mut c_void;
    fn GEOSSymDifference(g1: *const c_void, g2: *const c_void) -> *mut c_void;
    fn GEOSDifference(g1: *const c_void, g2: *const c_void) -> *mut c_void;
    fn GEOSClipByRect(
        g: *const c_void,
        xmin: c_double,
        ymin: c_double,
        xmax: c_double,
        ymax: c_double,
    ) -> *mut c_void;
    fn GEOSSnap(g1: *const c_void, g2: *const c_void, tolerance: c_double) -> *mut c_void;
    fn GEOSGeom_extractUniquePoints(g: *const c_void) -> *mut c_void;

    // Functions acting on GEOSPreparedGeometry :
    fn GEOSPreparedContains(pg1: *const GEOSPreparedGeometry, g2: *const c_void) -> c_int;
    fn GEOSPreparedContainsProperly(pg1: *const GEOSPreparedGeometry, g2: *const c_void) -> c_int;
    fn GEOSPreparedCoveredBy(pg1: *const GEOSPreparedGeometry, g2: *const c_void) -> c_int;
    fn GEOSPreparedCovers(pg1: *const GEOSPreparedGeometry, g2: *const c_void) -> c_int;
    fn GEOSPreparedCrosses(pg1: *const GEOSPreparedGeometry, g2: *const c_void) -> c_int;
    fn GEOSPreparedDisjoint(pg1: *const GEOSPreparedGeometry, g2: *const c_void) -> c_int;
    fn GEOSPreparedIntersects(pg1: *const GEOSPreparedGeometry, g2: *const c_void) -> c_int;
    fn GEOSPreparedOverlaps(pg1: *const GEOSPreparedGeometry, g2: *const c_void) -> c_int;
    fn GEOSPreparedTouches(pg1: *const GEOSPreparedGeometry, g2: *const c_void) -> c_int;
    fn GEOSPreparedWithin(pg1: *const GEOSPreparedGeometry, g2: *const c_void) -> c_int;
    fn GEOSPreparedGeom_destroy(g: *mut GEOSPreparedGeometry);
}

pub enum GEOSWKTReader {}
pub enum GEOSWKTWriter {}
pub enum GEOSPreparedGeometry {}
pub enum GEOSCoordSequence {}

#[derive(Debug)]
#[repr(C)]
#[allow(dead_code)]
pub enum GEOSGeomTypes {
    GEOS_POINT = 0,
    GEOS_LINESTRING,
    GEOS_LINEARRING,
    GEOS_POLYGON,
    GEOS_MULTIPOINT,
    GEOS_MULTILINESTRING,
    GEOS_MULTIPOLYGON,
    GEOS_GEOMETRYCOLLECTION,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GeosError {
    pub desc: &'static str,
}

// We need to cleanup only the char* from geos, the const char* are not to be freed.
// this has to be checked method by method in geos
// so we provide 2 method to wrap a char* to a string, one that manage (and thus free) the underlying char*
// and one that does not free it
fn _string(raw_ptr: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(raw_ptr) };
    let s = str::from_utf8(c_str.to_bytes()).unwrap().to_string();
    s
}

fn managed_string(raw_ptr: *const c_char) -> String {
    let s = _string(raw_ptr);
    unsafe { GEOSFree(raw_ptr as *mut c_void) };
    s
}

#[allow(dead_code)]
pub fn snap(g1: &GGeom, g2: &GGeom, tolerance: f64) -> GGeom {
    GGeom::new_from_c_obj(unsafe { GEOSSnap(g1.c_obj, g2.c_obj, tolerance as c_double) })
}

#[allow(dead_code)]
pub fn clip_by_rect(g: &GGeom, xmin: f64, ymin: f64, xmax: f64, ymax: f64) -> GGeom {
    GGeom::new_from_c_obj(unsafe {
        GEOSClipByRect(
            g.c_obj,
            xmin as c_double,
            ymin as c_double,
            xmax as c_double,
            ymax as c_double,
        )
    })
}

pub fn version() -> String {
    unsafe { _string(GEOSversion()) }
}

fn initialize() {
    static INIT: Once = ONCE_INIT;
    INIT.call_once(|| unsafe {
        initGEOS();
        assert_eq!(atexit(cleanup), 0);
    });

    extern "C" fn cleanup() {
        unsafe {
            finishGEOS();
        }
    }
}

pub struct CoordSeq(*mut GEOSCoordSequence);

impl Drop for CoordSeq {
    fn drop(&mut self) {
        unsafe { GEOSCoordSeq_destroy(self.0 as *mut GEOSCoordSequence) };
        self.0 = ptr::null_mut();
    }
}

impl Clone for CoordSeq {
    fn clone(&self) -> CoordSeq {
        CoordSeq(unsafe { GEOSCoordSeq_clone(self.0 as *const GEOSCoordSequence) })
    }
}

impl CoordSeq {
    pub fn new(size: u32, dims: u32) -> CoordSeq {
        initialize();
        CoordSeq(unsafe { GEOSCoordSeq_create(size as c_uint, dims as c_uint) })
    }

    pub fn new_from_c_obj(c_obj: *mut GEOSCoordSequence) -> CoordSeq {
        CoordSeq(c_obj)
    }
    pub fn set_x(&self, idx: u32, val: f64) -> i32 {
        let ret_val = unsafe {
            GEOSCoordSeq_setX(
                self.0 as *mut GEOSCoordSequence,
                idx as c_uint,
                val as c_double,
            )
        };
        return ret_val;
    }
    pub fn set_y(&self, idx: u32, val: f64) -> i32 {
        let ret_val = unsafe {
            GEOSCoordSeq_setY(
                self.0 as *mut GEOSCoordSequence,
                idx as c_uint,
                val as c_double,
            )
        };
        return ret_val;
    }
    pub fn set_z(&self, idx: u32, val: f64) -> i32 {
        let ret_val = unsafe {
            GEOSCoordSeq_setZ(
                self.0 as *mut GEOSCoordSequence,
                idx as c_uint,
                val as c_double,
            )
        };
        return ret_val;
    }

    pub fn get_x(&self, idx: u32) -> f64 {
        let n_mut_ref = &mut 0.0;
        let ret_val = unsafe {
            GEOSCoordSeq_getX(
                self.0 as *const GEOSCoordSequence,
                idx as c_uint,
                n_mut_ref as *mut c_double,
            )
        };
        if ret_val == 0 {
            panic!("Error when getting coordinates from CoordSeq");
        }
        return *n_mut_ref;
    }

    pub fn get_y(&self, idx: u32) -> f64 {
        let n_mut_ref = &mut 0.0;
        let ret_val = unsafe {
            GEOSCoordSeq_getY(
                self.0 as *const GEOSCoordSequence,
                idx as c_uint,
                n_mut_ref as *mut c_double,
            )
        };
        if ret_val == 0 {
            panic!("Error when getting coordinates from CoordSeq");
        }
        return *n_mut_ref;
    }

    pub fn get_z(&self, idx: u32) -> f64 {
        let n_mut_ref = &mut 0.0;
        let ret_val = unsafe {
            GEOSCoordSeq_getZ(
                self.0 as *const GEOSCoordSequence,
                idx as c_uint,
                n_mut_ref as *mut c_double,
            )
        };
        if ret_val == 0 {
            panic!("Error when getting coordinates from CoordSeq");
        }
        return *n_mut_ref;
    }
    pub fn release(mut self) -> *mut GEOSCoordSequence {
        std::mem::replace(&mut self.0, std::ptr::null_mut())
    }
}

pub struct SafeCObj {
    c_obj: *mut c_void,
}

impl Drop for SafeCObj {
    fn drop(&mut self) {
        cleanup_ptr(self.c_obj);
        self.c_obj = ptr::null_mut();
    }
}
impl SafeCObj {
    pub fn release(mut self) -> *mut c_void {
        std::mem::replace(&mut self.c_obj, std::ptr::null_mut())
    }

    pub fn steal_from(mut geom: GGeom) -> Self {
        SafeCObj {
            c_obj: std::mem::replace(&mut geom.c_obj, std::ptr::null_mut())
        }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.c_obj
    }
}

pub struct GGeom {
    pub c_obj: *mut c_void,
    pub area: f64,
    pub _type: i32,
}

fn cleanup_ptr(c_obj: *mut c_void) {
    unsafe { GEOSGeom_destroy(c_obj as *mut c_void) };
}

impl Drop for GGeom {
    fn drop(&mut self) {
        cleanup_ptr(self.c_obj);
        self.c_obj = ptr::null_mut();
    }
}

impl Clone for GGeom {
    fn clone(&self) -> GGeom {
        let n_obj = unsafe { GEOSGeom_clone(self.c_obj as *const c_void) };
        GGeom {
            c_obj: n_obj,
            area: self.area,
            _type: self._type,
        }
    }
}

impl GGeom {
    pub fn new(wkt: &str) -> GGeom {
        initialize();
        let c_str = CString::new(wkt).unwrap();
        let reader = unsafe { GEOSWKTReader_create() };
        let obj = unsafe { GEOSWKTReader_read(reader, c_str.as_ptr()) };
        if obj.is_null() {
            panic!("Invalid geometry");
        }
        unsafe { GEOSWKTReader_destroy(reader) };
        GGeom::new_from_c_obj(obj)
    }

    pub fn new_from_wkb(wkb: &Vec<u8>) -> GGeom {
        initialize();
        // let strr = CString::new(wkb).unwrap();
        // let t = strr.as_bytes();
        let obj = unsafe { GEOSGeomFromWKB_buf(wkb.as_ptr(), wkb.len()) };
        GGeom::new_from_c_obj(obj)
    }

    pub fn new_from_c_obj(g: *mut c_void) -> GGeom {
        if g.is_null() {
            panic!("Invalid geometry");
        }
        let area = GGeom::_area(g as *const c_void);
        let type_geom = unsafe { GEOSGeomTypeId(g as *const c_void) as i32 };
        GGeom {
            c_obj: g,
            area: area,
            _type: type_geom,
        }
    }

    pub fn is_valid(&self) -> bool {
        let rv = unsafe { GEOSisValid(self.c_obj as *const c_void) };
        return if rv == 1 { true } else { false };
    }

    pub fn get_coord_seq(&self) -> Result<CoordSeq, &'static str> {
        match self._type {
            0 | 1 | 2 => {
                let t = unsafe { GEOSGeom_getCoordSeq(self.c_obj as *const c_void) };
                if t.is_null() {
                    return Err("Error retrieving Coordinates Sequence");
                }
                Ok(CoordSeq::new_from_c_obj(t as *mut GEOSCoordSequence))
            }
            _ => Err("Not implemented - Geometry must be a Point, LineString or LinearRing"),
        }
    }

    fn _area(obj: *const c_void) -> f64 {
        let n_mut_ref = &mut 0.0;
        let ret_val = unsafe { GEOSArea(obj, n_mut_ref as *mut c_double) };
        assert!(ret_val != 0);
        return *n_mut_ref;
    }

    pub fn to_wkt(&self) -> String {
        unsafe { managed_string(GEOSGeomToWKT(self.c_obj as *const c_void)) }
    }

    pub fn to_wkt_precison(&self, precision: Option<u32>) -> String {
        let writer = unsafe { GEOSWKTWriter_create() };
        if let Some(x) = precision {
            unsafe { GEOSWKTWriter_setRoundingPrecision(writer, x as c_int) }
        };
        let c_result = unsafe { GEOSWKTWriter_write(writer, self.c_obj as *const c_void) };
        let result = managed_string(c_result);
        unsafe { GEOSWKTWriter_destroy(writer) };
        result
    }

    pub fn is_ring(&self) -> bool {
        let rv = unsafe { GEOSisRing(self.c_obj as *const c_void) };
        return if rv == 1 { true } else { false };
    }

    pub fn intersects(&self, g2: &GGeom) -> bool {
        let ret_val =
            unsafe { GEOSIntersects(self.c_obj as *const c_void, g2.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn crosses(&self, g2: &GGeom) -> bool {
        let ret_val =
            unsafe { GEOSCrosses(self.c_obj as *const c_void, g2.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn disjoint(&self, g2: &GGeom) -> bool {
        let ret_val =
            unsafe { GEOSDisjoint(self.c_obj as *const c_void, g2.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn touches(&self, g2: &GGeom) -> bool {
        let ret_val =
            unsafe { GEOSTouches(self.c_obj as *const c_void, g2.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn overlaps(&self, g2: &GGeom) -> bool {
        let ret_val =
            unsafe { GEOSOverlaps(self.c_obj as *const c_void, g2.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn within(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe { GEOSWithin(self.c_obj as *const c_void, g2.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn equals(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe { GEOSEquals(self.c_obj as *const c_void, g2.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn equals_exact(&self, g2: &GGeom, precision: f64) -> bool {
        let ret_val = unsafe {
            GEOSEqualsExact(
                self.c_obj as *const c_void,
                g2.c_obj as *const c_void,
                precision as c_double,
            )
        };
        return ret_val == 1;
    }

    pub fn covers(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe { GEOSCovers(self.c_obj as *const c_void, g2.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn covered_by(&self, g2: &GGeom) -> bool {
        let ret_val =
            unsafe { GEOSCoveredBy(self.c_obj as *const c_void, g2.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn contains(&self, g2: &GGeom) -> bool {
        let ret_val =
            unsafe { GEOSContains(self.c_obj as *const c_void, g2.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn buffer(&self, width: f64, quadsegs: i32) -> GGeom {
        GGeom::new_from_c_obj(unsafe {
            GEOSBuffer(
                self.c_obj as *const c_void,
                width as c_double,
                quadsegs as c_int,
            )
        })
    }

    pub fn is_empty(&self) -> bool {
        let ret_val = unsafe { GEOSisEmpty(self.c_obj as *const c_void) };
        return ret_val == 1;
    }

    pub fn is_simple(&self) -> bool {
        let ret_val = unsafe { GEOSisSimple(self.c_obj as *const c_void) };
        return ret_val == 1;
    }
    pub fn difference(&self, g2: &GGeom) -> GGeom {
        GGeom::new_from_c_obj(unsafe {
            GEOSDifference(self.c_obj as *const c_void, g2.c_obj as *const c_void)
        })
    }

    pub fn envelope(&self) -> GGeom {
        GGeom::new_from_c_obj(unsafe { GEOSEnvelope(self.c_obj as *const c_void) })
    }

    pub fn sym_difference(&self, g2: &GGeom) -> GGeom {
        GGeom::new_from_c_obj(unsafe {
            GEOSSymDifference(self.c_obj as *const c_void, g2.c_obj as *const c_void)
        })
    }

    pub fn get_centroid(&self) -> GGeom {
        GGeom::new_from_c_obj(unsafe { GEOSGetCentroid(self.c_obj as *const c_void) })
    }

    pub fn create_polygon(exterior: GGeom, interiors: Vec<GGeom>) -> GeosResult<GGeom> {
        let nb_interiors = interiors.len();

        let interiors_ptr: Vec<_> = interiors
            .into_iter()
            .map(|g| SafeCObj::steal_from(g))
            .collect();

        let external_c_obj = SafeCObj::steal_from(exterior);

        let t = unsafe {
            GEOSGeom_createPolygon(
                external_c_obj.as_ptr(),
                interiors_ptr[..].as_ptr() as *mut *mut c_void,
                nb_interiors as c_uint,
            )
        };
        if t.is_null() {
            Err(Error::InvalidGeometry)
        } else {
            // we'll transfert the ownership of the ptr to the new GGeom,
            // so the old one needs to forget their c ptr to avoid double cleanup
            external_c_obj.release();
            for mut i in interiors_ptr {
                i.release();
            }
            Ok(GGeom::new_from_c_obj(t))
        }
    }

    pub fn create_mulipolygon(polygons: Vec<GGeom>) -> GeosResult<GGeom> {
        let nb_polygons = polygons.len();
        let polygons: Vec<_> = polygons
            .into_iter()
            .map(|g| SafeCObj::steal_from(g))
            .collect();

        let t = unsafe {
            GEOSGeom_createCollection(
                GEOSGeomTypes::GEOS_MULTIPOLYGON as c_int,
                polygons[..].as_ptr() as *mut *mut c_void,
                nb_polygons as c_uint,
            )
        };

        if t.is_null() {
            Err(Error::InvalidGeometry)
        } else {
            for mut p in polygons {
                p.release();
            }
            Ok(GGeom::new_from_c_obj(t))
        }
    }

    pub fn create_point(s: &CoordSeq) -> GGeom {
        GGeom::new_from_c_obj(unsafe {
            GEOSGeom_createPoint(GEOSCoordSeq_clone(s.0 as *const GEOSCoordSequence))
        })
    }

    pub fn create_line_string(s: CoordSeq) -> GGeom {
        let obj = GGeom::new_from_c_obj(unsafe {
            GEOSGeom_createLineString(s.0 as *const GEOSCoordSequence)
        });
        s.release();
        obj
    }

    pub fn create_linear_ring(s: CoordSeq) -> GGeom {
        let obj = GGeom::new_from_c_obj(unsafe {
            GEOSGeom_createLinearRing(s.0 as *const GEOSCoordSequence)
        });
        s.release();
        obj
    }
}

pub struct PreparedGGeom(*mut GEOSPreparedGeometry);

impl Clone for PreparedGGeom {
    fn clone(&self) -> PreparedGGeom {
        PreparedGGeom(self.0)
    }
}

impl Drop for PreparedGGeom {
    fn drop(&mut self) {
        if self.0.is_null() {
            return;
        }
        unsafe { GEOSPreparedGeom_destroy(self.0) };
        self.0 = ptr::null_mut();
    }
}

impl PreparedGGeom {
    pub fn new(g: &GGeom) -> PreparedGGeom {
        PreparedGGeom(unsafe { GEOSPrepare(g.c_obj) })
    }
    pub fn contains(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe {
            GEOSPreparedContains(
                self.0 as *const GEOSPreparedGeometry,
                g2.c_obj as *const c_void,
            )
        };
        return ret_val == 1;
    }
    pub fn contains_properly(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe {
            GEOSPreparedContainsProperly(
                self.0 as *const GEOSPreparedGeometry,
                g2.c_obj as *const c_void,
            )
        };
        return ret_val == 1;
    }
    pub fn covered_by(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe {
            GEOSPreparedCoveredBy(
                self.0 as *const GEOSPreparedGeometry,
                g2.c_obj as *const c_void,
            )
        };
        return ret_val == 1;
    }
    pub fn covers(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe {
            GEOSPreparedCovers(
                self.0 as *const GEOSPreparedGeometry,
                g2.c_obj as *const c_void,
            )
        };
        return ret_val == 1;
    }
    pub fn crosses(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe {
            GEOSPreparedCrosses(
                self.0 as *const GEOSPreparedGeometry,
                g2.c_obj as *const c_void,
            )
        };
        return ret_val == 1;
    }
    pub fn disjoint(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe {
            GEOSPreparedDisjoint(
                self.0 as *const GEOSPreparedGeometry,
                g2.c_obj as *const c_void,
            )
        };
        return ret_val == 1;
    }
    pub fn intersects(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe {
            GEOSPreparedIntersects(
                self.0 as *const GEOSPreparedGeometry,
                g2.c_obj as *const c_void,
            )
        };
        return ret_val == 1;
    }
    pub fn overlaps(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe {
            GEOSPreparedOverlaps(
                self.0 as *const GEOSPreparedGeometry,
                g2.c_obj as *const c_void,
            )
        };
        return ret_val == 1;
    }
    pub fn touches(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe {
            GEOSPreparedTouches(
                self.0 as *const GEOSPreparedGeometry,
                g2.c_obj as *const c_void,
            )
        };
        return ret_val == 1;
    }
    pub fn within(&self, g2: &GGeom) -> bool {
        let ret_val = unsafe {
            GEOSPreparedWithin(
                self.0 as *const GEOSPreparedGeometry,
                g2.c_obj as *const c_void,
            )
        };
        return ret_val == 1;
    }
}
