// Passes schemas between this library and a real geoarrow-c build, in both
// directions. The two headers define the same vocabulary types, so they
// cannot share a translation unit: the geoarrow-c side lives in
// cross_geoarrow_c.c behind plain-int declarations, and the C Data Interface
// structs are the shared ground.

#include <stdio.h>
#include <string.h>

#include "../geoarrow_rs.h"

// Implemented in cross_geoarrow_c.c against geoarrow-c.
int c_schema_init(struct ArrowSchema *out, int type);
int c_schema_read(const struct ArrowSchema *schema, int *out_type,
                  int *out_edge_type, char *crs, int crs_size);
void c_schema_release(struct ArrowSchema *schema);

static int failures = 0;

static void check(int condition, const char *what) {
  if (!condition) {
    fprintf(stderr, "FAIL: %s\n", what);
    failures++;
  }
}

// One code per shared vocabulary family: native separate, native with M,
// interleaved ZM, box, and the two serialized encodings. The values this
// library invents (geoarrow.geometry, GEOMETRYCOLLECTION) have no geoarrow-c
// counterpart, so they cannot appear here.
static const GeoArrowType shared_codes[] = {
    GEOARROW_TYPE_POINT,          GEOARROW_TYPE_MULTILINESTRING_M,
    GEOARROW_TYPE_INTERLEAVED_POLYGON_ZM, GEOARROW_TYPE_BOX,
    GEOARROW_TYPE_WKB,            GEOARROW_TYPE_WKT,
};
static const int n_shared = sizeof(shared_codes) / sizeof(shared_codes[0]);

int main(void) {
  char what[128];
  struct GeoArrowError error;
  memset(&error, 0, sizeof(error));

  // geoarrow-c writes, this library reads. geoarrow-c allocated the schema,
  // and this library's release entry point must free it through the embedded
  // callback: that handoff is the C Data Interface contract.
  for (int i = 0; i < n_shared; i++) {
    int code = (int)shared_codes[i];
    struct ArrowSchema schema;
    snprintf(what, sizeof(what), "geoarrow-c builds %d", code);
    check(c_schema_init(&schema, code) == 0, what);

    GeoArrowType type = GEOARROW_TYPE_UNINITIALIZED;
    GeoArrowErrorCode rc = GeoArrowRsSchemaType(&schema, &type, &error);
    snprintf(what, sizeof(what), "this library reads %d back: %s", code,
             error.message);
    check(rc == GEOARROW_OK && (int)type == code, what);
    GeoArrowRsSchemaRelease(&schema);
    check(schema.release == NULL, "our release runs geoarrow-c's callback");
  }

  // This library writes, geoarrow-c reads.
  for (int i = 0; i < n_shared; i++) {
    int code = (int)shared_codes[i];
    struct ArrowSchema schema;
    memset(&schema, 0, sizeof(schema));
    snprintf(what, sizeof(what), "this library builds %d", code);
    check(GeoArrowRsSchemaInit(&schema, code, GEOARROW_EDGE_TYPE_PLANAR, NULL,
                               &error) == GEOARROW_OK,
          what);

    int type = 0;
    int edge_type = -1;
    char crs[512] = {0};
    int rc = c_schema_read(&schema, &type, &edge_type, crs, sizeof(crs));
    snprintf(what, sizeof(what), "geoarrow-c reads %d back, got %d", code, type);
    check(rc == 0 && type == code, what);
    check(edge_type == (int)GEOARROW_EDGE_TYPE_PLANAR, "planar edges survive");
    c_schema_release(&schema);
  }

  // Edges and CRS must survive the crossing, not only the type code.
  struct ArrowSchema schema;
  memset(&schema, 0, sizeof(schema));
  const char *projjson = "{\"id\":{\"authority\":\"EPSG\",\"code\":4326}}";
  check(GeoArrowRsSchemaInit(&schema, GEOARROW_TYPE_POINT,
                             GEOARROW_EDGE_TYPE_SPHERICAL, projjson,
                             &error) == GEOARROW_OK,
        "this library builds a spherical point with a CRS");
  int type = 0;
  int edge_type = -1;
  char crs[512] = {0};
  check(c_schema_read(&schema, &type, &edge_type, crs, sizeof(crs)) == 0 &&
            type == (int)GEOARROW_TYPE_POINT,
        "geoarrow-c reads the spherical point");
  check(edge_type == (int)GEOARROW_EDGE_TYPE_SPHERICAL,
        "spherical edges survive the crossing");
  snprintf(what, sizeof(what), "the CRS survives the crossing, got %s", crs);
  check(strstr(crs, "4326") != NULL, what);
  c_schema_release(&schema);

  if (failures == 0) {
    printf("cross ok\n");
  }
  return failures;
}
