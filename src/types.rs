//! The geoarrow-c type vocabulary.
//!
//! Names and values are copied from geoarrow-c's `geoarrow_type.h`, so one
//! set of constants works with both libraries. Functions take these as plain
//! `int32_t`: an out-of-range discriminant is undefined behavior in Rust but
//! only a wrong integer in C, so codes are validated on entry.
//!
//! The `GEOMETRYCOLLECTION` and `geoarrow.geometry` values have no geoarrow-c
//! counterpart. GEOMETRYCOLLECTION follows geoarrow-c's encoding,
//! `(coord_type - 1) * 10000 + (dimensions - 1) * 1000 + geometry_type`, with
//! geometry_type 7. The `geoarrow.geometry` values take 991 and 10991, next
//! to BOX at 990, with the dimension slot left at its XY position: the type
//! mixes dimensions per row, which the encoding cannot express.

use std::os::raw::c_char;
use std::sync::Arc;

use geoarrow_schema::{
    BoxType, CoordType, Crs, Dimension, Edges, GeoArrowType as RsType, GeometryCollectionType,
    GeometryType, LineStringType, Metadata, MultiLineStringType, MultiPointType, MultiPolygonType,
    PointType, PolygonType, WkbType, WktType,
};

use crate::error::Error;

#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum GeoArrowDimensions {
    GEOARROW_DIMENSIONS_UNKNOWN = 0,
    GEOARROW_DIMENSIONS_XY = 1,
    GEOARROW_DIMENSIONS_XYZ = 2,
    GEOARROW_DIMENSIONS_XYM = 3,
    GEOARROW_DIMENSIONS_XYZM = 4,
}

#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum GeoArrowCoordType {
    GEOARROW_COORD_TYPE_UNKNOWN = 0,
    GEOARROW_COORD_TYPE_SEPARATE = 1,
    GEOARROW_COORD_TYPE_INTERLEAVED = 2,
}

#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum GeoArrowEdgeType {
    GEOARROW_EDGE_TYPE_PLANAR = 0,
    GEOARROW_EDGE_TYPE_SPHERICAL = 1,
    GEOARROW_EDGE_TYPE_VINCENTY = 2,
    GEOARROW_EDGE_TYPE_THOMAS = 3,
    GEOARROW_EDGE_TYPE_ANDOYER = 4,
    GEOARROW_EDGE_TYPE_KARNEY = 5,
}

#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum GeoArrowType {
    GEOARROW_TYPE_UNINITIALIZED = 0,

    GEOARROW_TYPE_WKB = 100001,
    GEOARROW_TYPE_LARGE_WKB = 100002,
    GEOARROW_TYPE_WKT = 100003,
    GEOARROW_TYPE_LARGE_WKT = 100004,
    GEOARROW_TYPE_WKB_VIEW = 100005,
    GEOARROW_TYPE_WKT_VIEW = 100006,

    GEOARROW_TYPE_BOX = 990,
    GEOARROW_TYPE_BOX_Z = 1990,
    GEOARROW_TYPE_BOX_M = 2990,
    GEOARROW_TYPE_BOX_ZM = 3990,

    GEOARROW_TYPE_GEOMETRY = 991,
    GEOARROW_TYPE_INTERLEAVED_GEOMETRY = 10991,

    GEOARROW_TYPE_POINT = 1,
    GEOARROW_TYPE_LINESTRING = 2,
    GEOARROW_TYPE_POLYGON = 3,
    GEOARROW_TYPE_MULTIPOINT = 4,
    GEOARROW_TYPE_MULTILINESTRING = 5,
    GEOARROW_TYPE_MULTIPOLYGON = 6,
    GEOARROW_TYPE_GEOMETRYCOLLECTION = 7,

    GEOARROW_TYPE_POINT_Z = 1001,
    GEOARROW_TYPE_LINESTRING_Z = 1002,
    GEOARROW_TYPE_POLYGON_Z = 1003,
    GEOARROW_TYPE_MULTIPOINT_Z = 1004,
    GEOARROW_TYPE_MULTILINESTRING_Z = 1005,
    GEOARROW_TYPE_MULTIPOLYGON_Z = 1006,
    GEOARROW_TYPE_GEOMETRYCOLLECTION_Z = 1007,

    GEOARROW_TYPE_POINT_M = 2001,
    GEOARROW_TYPE_LINESTRING_M = 2002,
    GEOARROW_TYPE_POLYGON_M = 2003,
    GEOARROW_TYPE_MULTIPOINT_M = 2004,
    GEOARROW_TYPE_MULTILINESTRING_M = 2005,
    GEOARROW_TYPE_MULTIPOLYGON_M = 2006,
    GEOARROW_TYPE_GEOMETRYCOLLECTION_M = 2007,

    GEOARROW_TYPE_POINT_ZM = 3001,
    GEOARROW_TYPE_LINESTRING_ZM = 3002,
    GEOARROW_TYPE_POLYGON_ZM = 3003,
    GEOARROW_TYPE_MULTIPOINT_ZM = 3004,
    GEOARROW_TYPE_MULTILINESTRING_ZM = 3005,
    GEOARROW_TYPE_MULTIPOLYGON_ZM = 3006,
    GEOARROW_TYPE_GEOMETRYCOLLECTION_ZM = 3007,

    GEOARROW_TYPE_INTERLEAVED_POINT = 10001,
    GEOARROW_TYPE_INTERLEAVED_LINESTRING = 10002,
    GEOARROW_TYPE_INTERLEAVED_POLYGON = 10003,
    GEOARROW_TYPE_INTERLEAVED_MULTIPOINT = 10004,
    GEOARROW_TYPE_INTERLEAVED_MULTILINESTRING = 10005,
    GEOARROW_TYPE_INTERLEAVED_MULTIPOLYGON = 10006,
    GEOARROW_TYPE_INTERLEAVED_GEOMETRYCOLLECTION = 10007,

    GEOARROW_TYPE_INTERLEAVED_POINT_Z = 11001,
    GEOARROW_TYPE_INTERLEAVED_LINESTRING_Z = 11002,
    GEOARROW_TYPE_INTERLEAVED_POLYGON_Z = 11003,
    GEOARROW_TYPE_INTERLEAVED_MULTIPOINT_Z = 11004,
    GEOARROW_TYPE_INTERLEAVED_MULTILINESTRING_Z = 11005,
    GEOARROW_TYPE_INTERLEAVED_MULTIPOLYGON_Z = 11006,
    GEOARROW_TYPE_INTERLEAVED_GEOMETRYCOLLECTION_Z = 11007,

    GEOARROW_TYPE_INTERLEAVED_POINT_M = 12001,
    GEOARROW_TYPE_INTERLEAVED_LINESTRING_M = 12002,
    GEOARROW_TYPE_INTERLEAVED_POLYGON_M = 12003,
    GEOARROW_TYPE_INTERLEAVED_MULTIPOINT_M = 12004,
    GEOARROW_TYPE_INTERLEAVED_MULTILINESTRING_M = 12005,
    GEOARROW_TYPE_INTERLEAVED_MULTIPOLYGON_M = 12006,
    GEOARROW_TYPE_INTERLEAVED_GEOMETRYCOLLECTION_M = 12007,

    GEOARROW_TYPE_INTERLEAVED_POINT_ZM = 13001,
    GEOARROW_TYPE_INTERLEAVED_LINESTRING_ZM = 13002,
    GEOARROW_TYPE_INTERLEAVED_POLYGON_ZM = 13003,
    GEOARROW_TYPE_INTERLEAVED_MULTIPOINT_ZM = 13004,
    GEOARROW_TYPE_INTERLEAVED_MULTILINESTRING_ZM = 13005,
    GEOARROW_TYPE_INTERLEAVED_MULTIPOLYGON_ZM = 13006,
    GEOARROW_TYPE_INTERLEAVED_GEOMETRYCOLLECTION_ZM = 13007,
}

const GEOMETRY: i32 = 991;
const BOX: i32 = 990;

fn dimension_code(dimension: Dimension) -> i32 {
    match dimension {
        Dimension::XY => 0,
        Dimension::XYZ => 1,
        Dimension::XYM => 2,
        Dimension::XYZM => 3,
    }
}

fn coord_code(coords: CoordType) -> i32 {
    match coords {
        CoordType::Separated => 0,
        CoordType::Interleaved => 1,
    }
}

fn native(coords: CoordType, dimension: Dimension, geometry: i32) -> i32 {
    coord_code(coords) * 10000 + dimension_code(dimension) * 1000 + geometry
}

/// Encode a geoarrow-rs type as its `GeoArrowType` value.
pub(crate) fn type_code(data_type: &RsType) -> i32 {
    match data_type {
        RsType::Wkb(_) => 100001,
        RsType::LargeWkb(_) => 100002,
        RsType::Wkt(_) => 100003,
        RsType::LargeWkt(_) => 100004,
        RsType::WkbView(_) => 100005,
        RsType::WktView(_) => 100006,
        // geoarrow.box is always a struct of doubles; no interleaved form.
        RsType::Rect(t) => dimension_code(t.dimension()) * 1000 + BOX,
        RsType::Geometry(t) => coord_code(t.coord_type()) * 10000 + GEOMETRY,
        RsType::Point(t) => native(t.coord_type(), t.dimension(), 1),
        RsType::LineString(t) => native(t.coord_type(), t.dimension(), 2),
        RsType::Polygon(t) => native(t.coord_type(), t.dimension(), 3),
        RsType::MultiPoint(t) => native(t.coord_type(), t.dimension(), 4),
        RsType::MultiLineString(t) => native(t.coord_type(), t.dimension(), 5),
        RsType::MultiPolygon(t) => native(t.coord_type(), t.dimension(), 6),
        RsType::GeometryCollection(t) => native(t.coord_type(), t.dimension(), 7),
    }
}

/// `Planar` is the GeoArrow default and is encoded as the absence of an
/// `edges` metadata key, hence `None` rather than a variant.
fn edges(code: i32) -> Result<Option<Edges>, Error> {
    match code {
        0 => Ok(None),
        1 => Ok(Some(Edges::Spherical)),
        2 => Ok(Some(Edges::Vincenty)),
        3 => Ok(Some(Edges::Thomas)),
        4 => Ok(Some(Edges::Andoyer)),
        5 => Ok(Some(Edges::Karney)),
        _ => Err(Error::invalid(format!(
            "unsupported GeoArrowEdgeType: {code}"
        ))),
    }
}

fn dimension(code: i32) -> Result<Dimension, Error> {
    match code {
        0 => Ok(Dimension::XY),
        1 => Ok(Dimension::XYZ),
        2 => Ok(Dimension::XYM),
        3 => Ok(Dimension::XYZM),
        _ => Err(Error::invalid(format!(
            "unsupported dimensions in GeoArrowType: {code}"
        ))),
    }
}

/// # Safety
/// `crs_projjson` must be null or a NUL-terminated UTF-8 string.
pub(crate) unsafe fn metadata(
    crs_projjson: *const c_char,
    edge_type: i32,
) -> Result<Arc<Metadata>, Error> {
    let crs = match unsafe { crs_projjson.as_ref() } {
        None => Crs::default(),
        Some(_) => {
            let text = unsafe { std::ffi::CStr::from_ptr(crs_projjson) }
                .to_str()
                .map_err(|e| Error::invalid(format!("crs is not valid UTF-8: {e}")))?;
            let json: serde_json::Value = serde_json::from_str(text)
                .map_err(|e| Error::invalid(format!("crs is not valid PROJJSON: {e}")))?;
            Crs::from_projjson(json)
        }
    };
    Ok(Arc::new(Metadata::new(crs, edges(edge_type)?)))
}

/// Decode a `GeoArrowType` value into the geoarrow-rs type it names.
///
/// geoarrow-c encodes the coordinate layout into the type value itself
/// (`GEOARROW_TYPE_INTERLEAVED_POINT`), so no separate coord_type argument
/// exists to disagree with it.
pub(crate) fn data_type(code: i32, metadata: Arc<Metadata>) -> Result<RsType, Error> {
    if let Some(serialized) = serialized_type(code, &metadata) {
        return Ok(serialized);
    }
    let interleaved = code / 10000;
    let coords = match interleaved {
        0 => CoordType::Separated,
        1 => CoordType::Interleaved,
        _ => return Err(Error::invalid(format!("unsupported GeoArrowType: {code}"))),
    };
    let rest = code % 10000;

    if rest == GEOMETRY {
        return Ok(RsType::Geometry(
            GeometryType::new(metadata).with_coord_type(coords),
        ));
    }
    let dim = dimension(rest / 1000)?;
    if rest % 1000 == BOX {
        // geoarrow.box is always a struct of doubles, so an interleaved
        // spelling of it does not exist.
        if interleaved == 1 {
            return Err(Error::invalid(format!(
                "geoarrow.box has no interleaved form: {code}"
            )));
        }
        return Ok(RsType::Rect(BoxType::new(dim, metadata)));
    }
    Ok(match rest % 1000 {
        1 => RsType::Point(PointType::new(dim, metadata).with_coord_type(coords)),
        2 => RsType::LineString(LineStringType::new(dim, metadata).with_coord_type(coords)),
        3 => RsType::Polygon(PolygonType::new(dim, metadata).with_coord_type(coords)),
        4 => RsType::MultiPoint(MultiPointType::new(dim, metadata).with_coord_type(coords)),
        5 => {
            RsType::MultiLineString(MultiLineStringType::new(dim, metadata).with_coord_type(coords))
        }
        6 => RsType::MultiPolygon(MultiPolygonType::new(dim, metadata).with_coord_type(coords)),
        7 => RsType::GeometryCollection(
            GeometryCollectionType::new(dim, metadata).with_coord_type(coords),
        ),
        _ => return Err(Error::invalid(format!("unsupported GeoArrowType: {code}"))),
    })
}

fn serialized_type(code: i32, metadata: &Arc<Metadata>) -> Option<RsType> {
    let wkb = || WkbType::new(metadata.clone());
    let wkt = || WktType::new(metadata.clone());
    match code {
        100001 => Some(RsType::Wkb(wkb())),
        100002 => Some(RsType::LargeWkb(wkb())),
        100003 => Some(RsType::Wkt(wkt())),
        100004 => Some(RsType::LargeWkt(wkt())),
        100005 => Some(RsType::WkbView(wkb())),
        100006 => Some(RsType::WktView(wkt())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use geoarrow_schema::CoordType::{Interleaved, Separated};
    use geoarrow_schema::Dimension::{XY, XYM, XYZ, XYZM};

    use super::GeoArrowType::*;
    use super::*;

    fn meta() -> Arc<Metadata> {
        Arc::new(Metadata::default())
    }

    fn point(d: Dimension, c: CoordType) -> RsType {
        RsType::Point(PointType::new(d, meta()).with_coord_type(c))
    }
    fn linestring(d: Dimension, c: CoordType) -> RsType {
        RsType::LineString(LineStringType::new(d, meta()).with_coord_type(c))
    }
    fn polygon(d: Dimension, c: CoordType) -> RsType {
        RsType::Polygon(PolygonType::new(d, meta()).with_coord_type(c))
    }
    fn multipoint(d: Dimension, c: CoordType) -> RsType {
        RsType::MultiPoint(MultiPointType::new(d, meta()).with_coord_type(c))
    }
    fn multilinestring(d: Dimension, c: CoordType) -> RsType {
        RsType::MultiLineString(MultiLineStringType::new(d, meta()).with_coord_type(c))
    }
    fn multipolygon(d: Dimension, c: CoordType) -> RsType {
        RsType::MultiPolygon(MultiPolygonType::new(d, meta()).with_coord_type(c))
    }
    fn collection(d: Dimension, c: CoordType) -> RsType {
        RsType::GeometryCollection(GeometryCollectionType::new(d, meta()).with_coord_type(c))
    }
    fn geometry(c: CoordType) -> RsType {
        RsType::Geometry(GeometryType::new(meta()).with_coord_type(c))
    }
    fn rect(d: Dimension) -> RsType {
        RsType::Rect(BoxType::new(d, meta()))
    }

    /// Every `type_code` arm against the exported enum: a swapped geometry
    /// literal or multiplier fails here, and the parity test pins the enum to
    /// geoarrow-c. The GEOMETRY and GEOMETRYCOLLECTION rows carry the
    /// invented values, which no parity test covers, so all of them appear.
    /// Each row also decodes its value back, so `data_type` and `type_code`
    /// stay inverse over the whole vocabulary.
    #[test]
    fn type_code_matches_the_exported_enum() {
        fn code_is(built: RsType, expected: GeoArrowType) {
            let expected = expected as i32;
            assert_eq!(type_code(&built), expected, "for {expected}");
            let decoded = data_type(expected, meta()).unwrap();
            assert_eq!(type_code(&decoded), expected, "decode for {expected}");
        }

        code_is(point(XY, Separated), GEOARROW_TYPE_POINT);
        code_is(linestring(XY, Separated), GEOARROW_TYPE_LINESTRING);
        code_is(polygon(XY, Separated), GEOARROW_TYPE_POLYGON);
        code_is(multipoint(XY, Separated), GEOARROW_TYPE_MULTIPOINT);
        code_is(
            multilinestring(XY, Separated),
            GEOARROW_TYPE_MULTILINESTRING,
        );
        code_is(multipolygon(XY, Separated), GEOARROW_TYPE_MULTIPOLYGON);
        code_is(point(XYZ, Separated), GEOARROW_TYPE_POINT_Z);
        code_is(point(XYM, Separated), GEOARROW_TYPE_POINT_M);
        code_is(point(XYZM, Separated), GEOARROW_TYPE_POINT_ZM);
        code_is(point(XY, Interleaved), GEOARROW_TYPE_INTERLEAVED_POINT);
        code_is(
            linestring(XYZM, Interleaved),
            GEOARROW_TYPE_INTERLEAVED_LINESTRING_ZM,
        );
        code_is(
            polygon(XYZ, Interleaved),
            GEOARROW_TYPE_INTERLEAVED_POLYGON_Z,
        );
        code_is(
            multipolygon(XYM, Interleaved),
            GEOARROW_TYPE_INTERLEAVED_MULTIPOLYGON_M,
        );
        code_is(collection(XY, Separated), GEOARROW_TYPE_GEOMETRYCOLLECTION);
        code_is(
            collection(XYZ, Separated),
            GEOARROW_TYPE_GEOMETRYCOLLECTION_Z,
        );
        code_is(
            collection(XYM, Separated),
            GEOARROW_TYPE_GEOMETRYCOLLECTION_M,
        );
        code_is(
            collection(XYZM, Separated),
            GEOARROW_TYPE_GEOMETRYCOLLECTION_ZM,
        );
        code_is(
            collection(XY, Interleaved),
            GEOARROW_TYPE_INTERLEAVED_GEOMETRYCOLLECTION,
        );
        code_is(
            collection(XYZ, Interleaved),
            GEOARROW_TYPE_INTERLEAVED_GEOMETRYCOLLECTION_Z,
        );
        code_is(
            collection(XYM, Interleaved),
            GEOARROW_TYPE_INTERLEAVED_GEOMETRYCOLLECTION_M,
        );
        code_is(
            collection(XYZM, Interleaved),
            GEOARROW_TYPE_INTERLEAVED_GEOMETRYCOLLECTION_ZM,
        );
        code_is(geometry(Separated), GEOARROW_TYPE_GEOMETRY);
        code_is(geometry(Interleaved), GEOARROW_TYPE_INTERLEAVED_GEOMETRY);
        code_is(rect(XY), GEOARROW_TYPE_BOX);
        code_is(rect(XYZ), GEOARROW_TYPE_BOX_Z);
        code_is(rect(XYM), GEOARROW_TYPE_BOX_M);
        code_is(rect(XYZM), GEOARROW_TYPE_BOX_ZM);
        code_is(RsType::Wkb(WkbType::new(meta())), GEOARROW_TYPE_WKB);
        code_is(
            RsType::LargeWkb(WkbType::new(meta())),
            GEOARROW_TYPE_LARGE_WKB,
        );
        code_is(RsType::Wkt(WktType::new(meta())), GEOARROW_TYPE_WKT);
        code_is(
            RsType::LargeWkt(WktType::new(meta())),
            GEOARROW_TYPE_LARGE_WKT,
        );
        code_is(
            RsType::WkbView(WkbType::new(meta())),
            GEOARROW_TYPE_WKB_VIEW,
        );
        code_is(
            RsType::WktView(WktType::new(meta())),
            GEOARROW_TYPE_WKT_VIEW,
        );
    }

    /// The value-to-name mapping, which the roundtrip in `code_is` cannot
    /// see: a decoder wrong in the same way as `type_code` would still
    /// roundtrip.
    #[test]
    fn decodes_the_extension_name_a_value_names() {
        fn extension_name(code: i32) -> String {
            data_type(code, meta())
                .unwrap()
                .to_field("geometry", true)
                .metadata()["ARROW:extension:name"]
                .clone()
        }
        for (code, expected) in [
            (1, "geoarrow.point"),
            (2, "geoarrow.linestring"),
            (3, "geoarrow.polygon"),
            (4, "geoarrow.multipoint"),
            (5, "geoarrow.multilinestring"),
            (6, "geoarrow.multipolygon"),
            (7, "geoarrow.geometrycollection"),
            (990, "geoarrow.box"),
            (991, "geoarrow.geometry"),
            (100001, "geoarrow.wkb"),
            (100003, "geoarrow.wkt"),
        ] {
            assert_eq!(extension_name(code), expected, "for {code}");
        }
    }

    #[test]
    fn rejects_codes_outside_the_vocabulary() {
        // 10990 is the interleaved spelling of BOX, which does not exist.
        for code in [0, -1, 8, 999, 4001, 10990, 20001, 100007] {
            assert!(data_type(code, meta()).is_err(), "for {code}");
        }
    }

    #[test]
    fn rejects_edge_codes_outside_the_vocabulary() {
        assert_eq!(edges(0).unwrap(), None, "planar is the absent default");
        assert_eq!(edges(5).unwrap(), Some(Edges::Karney));
        for code in [6, -1] {
            assert!(edges(code).is_err(), "for {code}");
        }
    }
}
