#!/bin/sh
#
# Runs after the rootfs overlay has been applied, just before the filesystem
# image is built.
#
# This replaces board/raspberrypi/post-build.sh rather than extending it:
# upstream's adds a getty on tty1, and tty1 is where X runs.

set -eu

# The rootfs is mounted read-only, so anything under /var that a program expects
# to write to has to live on the tmpfs instead. Xorg is pointed elsewhere
# explicitly in start.sh; this catches everything else.
if [ ! -L "${TARGET_DIR}/var/log" ]; then
	rm -rf "${TARGET_DIR}/var/log"
	ln -sf /tmp "${TARGET_DIR}/var/log"
fi

# Fail here, with a clear message, rather than on the bench with a black screen.
# The libraries matter most: miniquad dlopen()s them by soname at startup, so a
# missing one is a silent runtime failure rather than a link error.
for f in \
	/usr/bin/chronopolis \
	/usr/bin/xinit \
	/usr/bin/matchbox-window-manager \
	/usr/share/chronopolis/start.sh \
	/usr/share/chronopolis/xinitrc \
	/usr/share/chronopolis/assets \
	/usr/lib/libX11.so.6 \
	/usr/lib/libXi.so.6 \
	/usr/lib/libxkbcommon.so.0 \
	/usr/lib/libGL.so.1
do
	if [ ! -e "${TARGET_DIR}${f}" ]; then
		echo "post-build.sh: ${f} is missing from the target rootfs" >&2
		exit 1
	fi
done
