//! A C ABI over geoarrow-rs, for callers that cannot link Rust directly.
//!
//! Arrays cross the boundary through the [Arrow C Data Interface], so no
//! geospatial data is copied or reserialized.
//!
//! [Arrow C Data Interface]: https://arrow.apache.org/docs/format/CDataInterface.html

use std::ffi::c_void;
use std::os::raw::c_char;

use arrow_data::ffi::FFI_ArrowArray;
use arrow_schema::ffi::FFI_ArrowSchema;

/// Arrow C Data Interface `ArrowSchema`.
///
/// Redeclared so the C signatures can name it without cbindgen walking into
/// arrow-rs. The header emits the canonical guarded definition from
/// cbindgen.toml; `layouts_match_arrow_rs` pins this copy to arrow-rs's.
#[repr(C)]
pub struct ArrowSchema {
    pub format: *const c_char,
    pub name: *const c_char,
    pub metadata: *const c_char,
    pub flags: i64,
    pub n_children: i64,
    pub children: *mut *mut ArrowSchema,
    pub dictionary: *mut ArrowSchema,
    pub release: Option<unsafe extern "C" fn(*mut ArrowSchema)>,
    pub private_data: *mut c_void,
}

/// Arrow C Data Interface `ArrowArray`. See [`ArrowSchema`].
#[repr(C)]
pub struct ArrowArray {
    pub length: i64,
    pub null_count: i64,
    pub offset: i64,
    pub n_buffers: i64,
    pub n_children: i64,
    pub buffers: *mut *const c_void,
    pub children: *mut *mut ArrowArray,
    pub dictionary: *mut ArrowArray,
    pub release: Option<unsafe extern "C" fn(*mut ArrowArray)>,
    pub private_data: *mut c_void,
}

/// The library version, as a static NUL-terminated string.
#[unsafe(no_mangle)]
pub extern "C" fn GeoArrowRsVersion() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

/// Release an array through its embedded release callback. Null is ignored.
///
/// # Safety
/// `array` must be null or address an array from a conforming Arrow C Data
/// Interface producer that has not already been released. The caller must not
/// use the array afterwards.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GeoArrowRsArrayRelease(array: *mut ArrowArray) {
    if array.is_null() {
        return;
    }
    // Dropping FFI_ArrowArray runs the embedded release callback. A panic
    // must not unwind across the C boundary; there is no return channel, so
    // it is swallowed.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        std::ptr::drop_in_place(array as *mut FFI_ArrowArray);
    }));
}

/// Release a schema through its embedded release callback. Null is ignored.
///
/// # Safety
/// `schema` must be null or address a schema from a conforming Arrow C Data
/// Interface producer that has not already been released. The caller must not
/// use the schema afterwards.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GeoArrowRsSchemaRelease(schema: *mut ArrowSchema) {
    if schema.is_null() {
        return;
    }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        std::ptr::drop_in_place(schema as *mut FFI_ArrowSchema);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_readable_as_a_c_string() {
        let version = unsafe { std::ffi::CStr::from_ptr(GeoArrowRsVersion()) };
        assert_eq!(version.to_str().unwrap(), env!("CARGO_PKG_VERSION"));
    }

    /// Entry points cast between these structs and arrow-rs's. All fields are
    /// pointer-sized, so size checks pass under any field permutation; the
    /// offsets pin the ABI.
    #[test]
    fn layouts_match_arrow_rs() {
        use std::mem::{align_of, offset_of, size_of};
        assert_eq!(size_of::<ArrowArray>(), size_of::<FFI_ArrowArray>());
        assert_eq!(align_of::<ArrowArray>(), align_of::<FFI_ArrowArray>());
        assert_eq!(size_of::<ArrowSchema>(), size_of::<FFI_ArrowSchema>());
        assert_eq!(align_of::<ArrowSchema>(), align_of::<FFI_ArrowSchema>());

        assert_eq!(offset_of!(ArrowSchema, format), 0);
        assert_eq!(offset_of!(ArrowSchema, name), 8);
        assert_eq!(offset_of!(ArrowSchema, metadata), 16);
        assert_eq!(offset_of!(ArrowSchema, flags), 24);
        assert_eq!(offset_of!(ArrowSchema, n_children), 32);
        assert_eq!(offset_of!(ArrowSchema, children), 40);
        assert_eq!(offset_of!(ArrowSchema, dictionary), 48);
        assert_eq!(offset_of!(ArrowSchema, release), 56);
        assert_eq!(offset_of!(ArrowSchema, private_data), 64);

        assert_eq!(offset_of!(ArrowArray, length), 0);
        assert_eq!(offset_of!(ArrowArray, null_count), 8);
        assert_eq!(offset_of!(ArrowArray, offset), 16);
        assert_eq!(offset_of!(ArrowArray, n_buffers), 24);
        assert_eq!(offset_of!(ArrowArray, n_children), 32);
        assert_eq!(offset_of!(ArrowArray, buffers), 40);
        assert_eq!(offset_of!(ArrowArray, children), 48);
        assert_eq!(offset_of!(ArrowArray, dictionary), 56);
        assert_eq!(offset_of!(ArrowArray, release), 64);
        assert_eq!(offset_of!(ArrowArray, private_data), 72);
    }

    /// Release must run the embedded callbacks and ignore null. A conforming
    /// callback nulls the `release` slot, which makes the call observable.
    #[test]
    fn release_runs_the_embedded_callbacks() {
        use std::mem::ManuallyDrop;

        use arrow_array::ffi::to_ffi;
        use arrow_array::{Array, Int32Array};

        let data = Int32Array::from(vec![1, 2, 3]).into_data();
        let (array, schema) = to_ffi(&data).unwrap();
        // The C release calls consume the values; the locals must not drop
        // them again.
        let mut array = ManuallyDrop::new(array);
        let mut schema = ManuallyDrop::new(schema);
        unsafe {
            let array = &mut *array as *mut FFI_ArrowArray as *mut ArrowArray;
            let schema = &mut *schema as *mut FFI_ArrowSchema as *mut ArrowSchema;
            assert!((*array).release.is_some());
            assert!((*schema).release.is_some());
            GeoArrowRsArrayRelease(array);
            GeoArrowRsSchemaRelease(schema);
            assert!((*array).release.is_none(), "array callback did not run");
            assert!((*schema).release.is_none(), "schema callback did not run");
            GeoArrowRsArrayRelease(std::ptr::null_mut());
            GeoArrowRsSchemaRelease(std::ptr::null_mut());
        }
    }
}
