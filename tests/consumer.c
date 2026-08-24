// Compiles against geoarrow_rs.h with a plain C compiler, links the static
// library, and drives the entry points from C.

#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "../geoarrow_rs.h"

// The Rust-side layouts_match_arrow_rs test pins every field offset of the
// Rust redeclarations to arrow-rs; these pin the C text to the same ABI.
_Static_assert(sizeof(struct ArrowSchema) == 72, "ArrowSchema size");
_Static_assert(offsetof(struct ArrowSchema, release) == 56, "ArrowSchema.release");
_Static_assert(sizeof(struct ArrowArray) == 80, "ArrowArray size");
_Static_assert(offsetof(struct ArrowArray, release) == 64, "ArrowArray.release");

// The Rust side pins the same layout and width.
_Static_assert(sizeof(struct GeoArrowError) == 1024, "GeoArrowError size");
_Static_assert(sizeof(GeoArrowType) == 4, "GeoArrowType width");

// The Rust error constants are these numerals; the platform must agree.
_Static_assert(EIO == 5 && ENOMEM == 12 && EINVAL == 22, "errno values");

// No Arrow header precedes geoarrow_rs.h here, so its guard emits the C Data
// Interface definitions, the stream struct included: a later header that
// skips its own Arrow block on that guard (nanoarrow.h) still needs
// struct ArrowArrayStream.
static int use_stream(struct ArrowArrayStream *stream, struct ArrowSchema *schema,
                      struct ArrowArray *array) {
  int rc = stream->get_schema(stream, schema);
  if (rc != 0) {
    return rc;
  }
  return stream->get_next(stream, array);
}
int (*stream_probe)(struct ArrowArrayStream *, struct ArrowSchema *,
                    struct ArrowArray *) = use_stream;

static int failures = 0;

static void check(int condition, const char *what) {
  if (!condition) {
    fprintf(stderr, "FAIL: %s\n", what);
    failures++;
  }
}

static void noop_release(struct ArrowSchema *schema) { (void)schema; }

// C Data Interface metadata: an int32 entry count, then length-prefixed key
// and value bytes, all native endian.
static size_t put_int32(char *buf, size_t off, int32_t value) {
  memcpy(buf + off, &value, sizeof(value));
  return off + sizeof(value);
}

static size_t put_string(char *buf, size_t off, const char *s) {
  int32_t len = (int32_t)strlen(s);
  off = put_int32(buf, off, len);
  memcpy(buf + off, s, (size_t)len);
  return off + (size_t)len;
}

int main(void) {
  const char *version = GeoArrowRsVersion();
  check(version != NULL && strlen(version) > 0, "version is a readable string");

  // Null contracts: releases ignore null, SchemaType reports EINVAL.
  GeoArrowRsArrayRelease(NULL);
  GeoArrowRsSchemaRelease(NULL);

  struct GeoArrowError error;
  memset(&error, 0, sizeof(error));
  GeoArrowType type = GEOARROW_TYPE_UNINITIALIZED;
  GeoArrowErrorCode rc = GeoArrowRsSchemaType(NULL, &type, &error);
  check(rc == EINVAL, "null schema returns EINVAL");
  check(strlen(error.message) > 0, "error message reaches the sink");
  check(type == GEOARROW_TYPE_UNINITIALIZED, "out_type stays untouched on failure");

  // The success path, through a schema built in C: a binary column tagged
  // with the geoarrow.wkb extension name. This drives the real ABI: struct
  // layout, metadata parsing, and the out-parameter write.
  char metadata[64];
  size_t off = put_int32(metadata, 0, 1);
  off = put_string(metadata, off, "ARROW:extension:name");
  off = put_string(metadata, off, "geoarrow.wkb");
  check(off <= sizeof(metadata), "metadata blob fits its buffer");

  struct ArrowSchema schema;
  memset(&schema, 0, sizeof(schema));
  schema.format = "z";
  schema.name = "geometry";
  schema.metadata = metadata;
  schema.release = noop_release;

  rc = GeoArrowRsSchemaType(&schema, &type, &error);
  check(rc == GEOARROW_OK, "wkb schema returns GEOARROW_OK");
  check(type == GEOARROW_TYPE_WKB, "wkb schema reports GEOARROW_TYPE_WKB");

  // Init -> Type round trip: a schema this library builds must read back as
  // the same code, and the release entry point must run its callback.
  struct ArrowSchema built;
  memset(&built, 0, sizeof(built));
  rc = GeoArrowRsSchemaInit(&built, GEOARROW_TYPE_INTERLEAVED_POINT_Z,
                            GEOARROW_EDGE_TYPE_SPHERICAL, NULL, &error);
  check(rc == GEOARROW_OK, "SchemaInit builds a spherical interleaved point z");
  type = GEOARROW_TYPE_UNINITIALIZED;
  rc = GeoArrowRsSchemaType(&built, &type, &error);
  check(rc == GEOARROW_OK, "built schema reads back");
  check(type == GEOARROW_TYPE_INTERLEAVED_POINT_Z, "round trip preserves the code");
  GeoArrowRsSchemaRelease(&built);
  check(built.release == NULL, "release runs the embedded callback");

  rc = GeoArrowRsSchemaInit(&built, 99999, 0, NULL, &error);
  check(rc == EINVAL, "unknown type code returns EINVAL");

  if (failures == 0) {
    printf("consumer ok: geoarrow-ffi %s\n", version);
  }
  return failures;
}
