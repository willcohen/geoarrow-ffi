#!/bin/sh
# Compile geoarrow-c from a checkout, link it into the same binary as this
# library, and pass schemas between the two in both directions. Uses
# GEOARROW_C_DIR, or a clone cached under target/, the same as
# tests/parity.rs. Run after `cargo build`, from the repository root.
set -e

: "${GEOARROW_C_DIR:=target/geoarrow-c}"
if [ ! -f "$GEOARROW_C_DIR/src/geoarrow/geoarrow.h" ]; then
  rm -rf "$GEOARROW_C_DIR"
  git clone --depth 1 https://github.com/geoarrow/geoarrow-c "$GEOARROW_C_DIR"
fi

BUILD=target/cross
mkdir -p "$BUILD/include/geoarrow"

# geoarrow_config.h is a CMake template in the checkout; the sources compiled
# here need only the version macros and the endianness default.
cat > "$BUILD/include/geoarrow/geoarrow_config.h" <<'EOF'
#ifndef GEOARROW_CONFIG_H_INCLUDED
#define GEOARROW_CONFIG_H_INCLUDED
#define GEOARROW_VERSION_MAJOR 0
#define GEOARROW_VERSION_MINOR 0
#define GEOARROW_VERSION_PATCH 0
#define GEOARROW_VERSION "cross-test"
#define GEOARROW_VERSION_INT 0
#ifndef GEOARROW_NATIVE_ENDIAN
#define GEOARROW_NATIVE_ENDIAN 0x01
#endif
#endif
EOF

GC_INCLUDES="-I $BUILD/include -I $GEOARROW_C_DIR/src -I $GEOARROW_C_DIR/src/vendor"

# Only the schema paths are needed, not the full library.
for src in schema schema_view metadata util; do
  cc -c "$GEOARROW_C_DIR/src/geoarrow/$src.c" $GC_INCLUDES -o "$BUILD/$src.o"
done
cc -c "$GEOARROW_C_DIR/src/vendor/nanoarrow/nanoarrow.c" \
  -I "$GEOARROW_C_DIR/src/vendor" -o "$BUILD/nanoarrow.o"

cc -c tests/cross_geoarrow_c.c $GC_INCLUDES -o "$BUILD/cross_geoarrow_c.o"
cc tests/cross.c "$BUILD"/*.o target/debug/libgeoarrow_rs.a \
  -lpthread -ldl -lm -o "$BUILD/cross"
"$BUILD/cross"
