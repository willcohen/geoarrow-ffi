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

use geoarrow_schema::{CoordType, Dimension, GeoArrowType as RsType};

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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use geoarrow_schema::CoordType::{Interleaved, Separated};
    use geoarrow_schema::Dimension::{XY, XYM, XYZ, XYZM};
    use geoarrow_schema::{
        BoxType, GeometryCollectionType, GeometryType, LineStringType, Metadata,
        MultiLineStringType, MultiPointType, MultiPolygonType, PointType, PolygonType, WkbType,
        WktType,
    };

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
    #[test]
    fn type_code_matches_the_exported_enum() {
        fn code_is(data_type: RsType, expected: GeoArrowType) {
            let expected = expected as i32;
            assert_eq!(type_code(&data_type), expected, "for {expected}");
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
}
