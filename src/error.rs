use std;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Invalid geometry")]
    InvalidGeometry,
    #[fail(display = "Invalid geometry, {}", _0)]
    InvalidGeometryDetail(String),
    #[fail(display = "impossible to build a geometry from a nullptr")]
    NoConstructionFromNullPtr,
}

pub type Result<T> = std::result::Result<T, Error>;