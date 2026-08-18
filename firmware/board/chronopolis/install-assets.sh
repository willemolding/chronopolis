#!/bin/sh
#
# Copy assets/ into the target rootfs, shrinking anything larger than MAX_DIM.
#
# Two reasons this is not a plain cp:
#
#  - VideoCore IV rejects textures above 2048x2048, and assets/clocktown/*.png
#    are 3000x3000, so those faces would come up blank on the Pi.
#  - textures.rs loads every image at startup and keeps it decoded, which at
#    source resolution is ~200MB of RGBA on a board with 512MB shared with the
#    GPU.
#
# The source tree is never modified.
#
# Usage: install-assets.sh <src-dir> <dst-dir> <max-dim>

set -eu

SRC="${1:?source assets directory required}"
DST="${2:?destination directory required}"
MAX="${3:?maximum edge length required}"

# ImageMagick 7 puts its tools behind `magick`; Debian still ships 6.
if command -v magick >/dev/null 2>&1; then
	MOGRIFY="magick mogrify"
else
	MOGRIFY="mogrify"
fi

# Rebuilt from scratch, so artwork deleted from the repo also leaves the image
# rather than lingering from an earlier build.
rm -rf "${DST}"
mkdir -p "${DST}"
cp -a "${SRC}/." "${DST}/"

# Only artwork ships. This is also what keeps stray .DS_Store files out.
find "${DST}" -type f \
	! \( -iname '*.png' -o -iname '*.jpg' -o -iname '*.jpeg' \) -delete

# '>' means shrink-only, so art already within the cap keeps its dimensions.
# -strip drops metadata the loader ignores anyway.
find "${DST}" -type f \( -iname '*.png' -o -iname '*.jpg' -o -iname '*.jpeg' \) \
	-exec ${MOGRIFY} -resize "${MAX}x${MAX}>" -strip {} +
