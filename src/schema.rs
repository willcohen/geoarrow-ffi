//! Reading the geoarrow type of an Arrow schema.

use arrow_schema::Field;
use arrow_schema::ffi::FFI_ArrowSchema;
use geoarrow_schema::GeoArrowType as RsType;

use crate::ArrowSchema;
use crate::error::{Error, GeoArrowError, GeoArrowErrorCode, catching, finish};
use crate::types::{GeoArrowType, type_code};

/// A schema tree deeper than this is rejected rather than recursed, so a
/// cyclic children graph cannot hang or overflow the caller's stack.
const MAX_DEPTH: usize = 64;

/// Reject the inputs arrow-rs asserts on instead of returning errors, so bad
/// input reports EINVAL, not a contained panic. `Field::try_from` walks the
/// whole tree, so the checks recurse through children and dictionaries too.
fn validate(raw: &ArrowSchema, depth: usize) -> Result<(), Error> {
    if depth > MAX_DEPTH {
        return Err(Error::invalid("schema nests deeper than 64 levels"));
    }
    if raw.release.is_none() {
        return Err(Error::invalid("schema is released or zero-initialized"));
    }
    if raw.format.is_null() {
        return Err(Error::invalid("schema format is null"));
    }
    if unsafe { std::ffi::CStr::from_ptr(raw.format) }
        .to_str()
        .is_err()
    {
        return Err(Error::invalid("schema format is not valid UTF-8"));
    }
    if !raw.name.is_null()
        && unsafe { std::ffi::CStr::from_ptr(raw.name) }
            .to_str()
            .is_err()
    {
        return Err(Error::invalid("schema name is not valid UTF-8"));
    }
    if raw.n_children < 0 {
        return Err(Error::invalid("schema n_children is negative"));
    }
    if raw.n_children > 0 {
        if raw.children.is_null() {
            return Err(Error::invalid("schema children pointer is null"));
        }
        for i in 0..raw.n_children as usize {
            let child = unsafe { *raw.children.add(i) };
            if child.is_null() {
                return Err(Error::invalid(format!("schema child {i} is null")));
            }
            validate(unsafe { &*child }, depth + 1)?;
        }
    }
    if !raw.dictionary.is_null() {
        validate(unsafe { &*raw.dictionary }, depth + 1)?;
    }
    Ok(())
}

/// Write the `GeoArrowType` value describing `schema` into `out_type`. The
/// schema is borrowed, not consumed. `error` may be null. On failure nothing
/// is written to `out_type`.
///
/// # Safety
/// `schema` must be null or address a readable [`ArrowSchema`] tree whose
/// non-null `metadata` pointers hold well-formed C Data Interface metadata
/// blocks; `out_type` must address a writable `GeoArrowType`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GeoArrowRsSchemaType(
    schema: *const ArrowSchema,
    out_type: *mut GeoArrowType,
    error: *mut GeoArrowError,
) -> GeoArrowErrorCode {
    let result = catching(|| {
        if schema.is_null() {
            return Err(Error::invalid("schema pointer is null"));
        }
        if out_type.is_null() {
            return Err(Error::invalid("out_type pointer is null"));
        }
        validate(unsafe { &*schema }, 0)?;
        let ffi = unsafe { &*(schema as *const FFI_ArrowSchema) };
        let field = Field::try_from(ffi)?;
        let code = type_code(&RsType::try_from(&field)?);
        // Every code type_code returns is a declared GeoArrowType value, so
        // the write through the enum pointer stays in range.
        unsafe { *(out_type as *mut i32) = code };
        Ok(())
    });
    finish(result, error)
}

#[cfg(test)]
mod tests {
    use std::os::raw::c_char;
    use std::sync::Arc;

    use arrow_schema::DataType;
    use geoarrow_schema::{
        BoxType, CoordType, Dimension, GeometryType, Metadata, PointType, PolygonType, WkbType,
    };

    use super::*;
    use crate::error::{EINVAL, GEOARROW_OK};

    fn run(schema: *const ArrowSchema) -> (GeoArrowErrorCode, i32, String) {
        let mut code = GeoArrowType::GEOARROW_TYPE_UNINITIALIZED;
        let mut sink = GeoArrowError { message: [0; 1024] };
        let rc = unsafe { GeoArrowRsSchemaType(schema, &mut code, &mut sink) };
        let message = unsafe { std::ffi::CStr::from_ptr(sink.message.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        (rc, code as i32, message)
    }

    fn code_of(field: &Field) -> Result<i32, GeoArrowErrorCode> {
        let ffi = FFI_ArrowSchema::try_from(field).unwrap();
        let (rc, code, _) = run(&ffi as *const FFI_ArrowSchema as *const ArrowSchema);
        match rc {
            GEOARROW_OK => Ok(code),
            _ => Err(rc),
        }
    }

    fn metadata() -> Arc<Metadata> {
        Arc::new(Metadata::default())
    }

    unsafe extern "C" fn noop_release(_: *mut ArrowSchema) {}

    /// A raw schema whose fields default to a valid childless int32.
    fn raw_schema(format: &'static [u8], name: Option<&'static [u8]>) -> ArrowSchema {
        ArrowSchema {
            format: format.as_ptr() as *const c_char,
            name: name.map_or(std::ptr::null(), |n| n.as_ptr() as *const c_char),
            metadata: std::ptr::null(),
            flags: 0,
            n_children: 0,
            children: std::ptr::null_mut(),
            dictionary: std::ptr::null_mut(),
            release: Some(noop_release),
            private_data: std::ptr::null_mut(),
        }
    }

    /// EINVAL with a message that is not a contained panic: arrow-rs asserts
    /// on malformed input, so a deleted guard shifts its case to a panic.
    fn expect_invalid(schema: &ArrowSchema) -> String {
        let (rc, _, message) = run(schema);
        assert_eq!(rc, EINVAL);
        assert!(!message.starts_with("panic:"), "got {message:?}");
        message
    }

    #[test]
    fn reports_the_code_a_geoarrow_field_describes() {
        let cases: Vec<(RsType, i32)> = vec![
            (RsType::Point(PointType::new(Dimension::XY, metadata())), 1),
            (
                RsType::Polygon(
                    PolygonType::new(Dimension::XYZM, metadata())
                        .with_coord_type(CoordType::Interleaved),
                ),
                13003,
            ),
            (RsType::Rect(BoxType::new(Dimension::XY, metadata())), 990),
            (RsType::Geometry(GeometryType::new(metadata())), 991),
            (RsType::Wkb(WkbType::new(metadata())), 100001),
        ];
        for (data_type, expected) in cases {
            let field = data_type.to_field("geometry", true);
            assert_eq!(code_of(&field).unwrap(), expected, "for {expected}");
        }
    }

    /// The message must reach the caller's sink; there is no thread-local
    /// fallback.
    #[test]
    fn rejects_a_field_that_is_not_geoarrow() {
        let field = Field::new("plain", DataType::Int32, true);
        let ffi = FFI_ArrowSchema::try_from(&field).unwrap();
        let (rc, _, message) = run(&ffi as *const FFI_ArrowSchema as *const ArrowSchema);
        assert_eq!(rc, EINVAL);
        assert!(!message.is_empty());
    }

    /// A geoarrow extension name on the wrong storage type must be EINVAL,
    /// not a contained panic.
    #[test]
    fn rejects_a_geoarrow_name_on_the_wrong_storage() {
        for extension in ["geoarrow.point", "geoarrow.box"] {
            let field = Field::new("geometry", DataType::Int32, true).with_metadata(
                [("ARROW:extension:name".to_string(), extension.to_string())].into(),
            );
            let ffi = FFI_ArrowSchema::try_from(&field).unwrap();
            let (rc, _, message) = run(&ffi as *const FFI_ArrowSchema as *const ArrowSchema);
            assert_eq!(rc, EINVAL, "for {extension}");
            assert!(!message.starts_with("panic:"), "got {message:?}");
        }
    }

    #[test]
    fn rejects_null_pointers() {
        let (rc, code, _) = run(std::ptr::null());
        assert_eq!(rc, EINVAL);
        assert_eq!(code, 0, "out_type must stay untouched on failure");

        // A valid schema, so only the null out_type guard can return EINVAL.
        let field = PointType::new(Dimension::XY, metadata()).to_field("geometry", true);
        let ffi = FFI_ArrowSchema::try_from(&field).unwrap();
        let rc = unsafe {
            GeoArrowRsSchemaType(
                &ffi as *const FFI_ArrowSchema as *const ArrowSchema,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        assert_eq!(rc, EINVAL);
    }

    /// One case per validate guard, each rejected by only its guard, so this
    /// also proves no guard shadows another.
    #[test]
    fn rejects_malformed_raw_schemas() {
        // release absent, format present: only the release guard fires.
        let mut released = raw_schema(b"i\0", None);
        released.release = None;
        expect_invalid(&released);

        // A zeroed struct, as sent by a caller that forgot to fill it in.
        let zeroed: ArrowSchema = unsafe { std::mem::zeroed() };
        expect_invalid(&zeroed);

        let invalid_utf8: &'static [u8] = b"\xff\0";
        expect_invalid(&raw_schema(invalid_utf8, None));
        expect_invalid(&raw_schema(b"i\0", Some(invalid_utf8)));

        // An unknown format string fails inside arrow-rs; the From impl must
        // map it to EINVAL.
        expect_invalid(&raw_schema(b"banana\0", Some(b"geometry\0")));

        // Struct format with n_children = 1 but a null children array.
        let mut null_children = raw_schema(b"+s\0", None);
        null_children.n_children = 1;
        expect_invalid(&null_children);

        // A null entry inside the children array.
        let mut entries: [*mut ArrowSchema; 1] = [std::ptr::null_mut()];
        let mut null_child = raw_schema(b"+s\0", None);
        null_child.n_children = 1;
        null_child.children = entries.as_mut_ptr();
        expect_invalid(&null_child);

        // A released child.
        let mut child = raw_schema(b"i\0", None);
        child.release = None;
        let mut child_ptr = &mut child as *mut ArrowSchema;
        let mut parent = raw_schema(b"+s\0", None);
        parent.n_children = 1;
        parent.children = &mut child_ptr;
        expect_invalid(&parent);

        // Self-referential: EINVAL via the depth cap, not a hang or a stack
        // overflow.
        let mut cyclic = raw_schema(b"+s\0", None);
        let mut self_ptr = &mut cyclic as *mut ArrowSchema;
        cyclic.n_children = 1;
        cyclic.children = &mut self_ptr;
        let message = expect_invalid(&cyclic);
        assert!(message.contains("deeper"), "got {message:?}");
    }
}
