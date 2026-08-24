// The geoarrow-c half of cross.c. This translation unit includes geoarrow.h,
// which cross.c cannot: the two libraries define the same vocabulary types.
// Types cross as plain int, schemas as C Data Interface structs.

#include <string.h>

#include "geoarrow/geoarrow.h"

int c_schema_init(struct ArrowSchema *out, int type) {
  return GeoArrowSchemaInitExtension(out, (enum GeoArrowType)type);
}

int c_schema_read(const struct ArrowSchema *schema, int *out_type,
                  int *out_edge_type, char *crs, int crs_size) {
  struct GeoArrowError error;
  struct GeoArrowSchemaView view;
  int rc = GeoArrowSchemaViewInit(&view, schema, &error);
  if (rc != GEOARROW_OK) {
    return rc;
  }
  *out_type = (int)view.type;

  struct GeoArrowMetadataView metadata;
  rc = GeoArrowMetadataViewInit(&metadata, view.extension_metadata, &error);
  if (rc != GEOARROW_OK) {
    return rc;
  }
  *out_edge_type = (int)metadata.edge_type;
  int64_t len = metadata.crs.size_bytes;
  if (len > crs_size - 1) {
    len = crs_size - 1;
  }
  if (len > 0) {
    memcpy(crs, metadata.crs.data, (size_t)len);
  }
  crs[len > 0 ? len : 0] = '\0';
  return GEOARROW_OK;
}

void c_schema_release(struct ArrowSchema *schema) {
  schema->release(schema);
}
