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

# Buildroot's xserver_xorg-server package installs an init script that starts an
# X server at boot, using the stock /etc/X11/xinit/xinitrc — which launches twm,
# an xclock and three xterms, none of which are in this image. The result is a
# bare X server holding display :0 and showing a blank root window, while
# start.sh's own xinit fails forever with "Server is already active for display
# 0". The clock manages its own X server, so this has to go.
rm -f "${TARGET_DIR}/etc/init.d/S40xorg"

# Fail here, with a clear message, rather than on the bench with a black screen.
# The libraries matter most: miniquad dlopen()s them by soname at startup, so a
# missing one is a silent runtime failure rather than a link error.
for f in \
	/usr/bin/chronopolis \
	/usr/bin/xinit \
	/usr/bin/matchbox-window-manager \
	/usr/share/chronopolis/start.sh \
	/usr/share/chronopolis/xinitrc \
	/usr/share/chronopolis/show-failure.sh \
	/usr/lib/libX11.so.6 \
	/usr/lib/libXi.so.6 \
	/usr/lib/libxkbcommon.so.0 \
	/usr/lib/libGL.so.1 \
	/usr/lib/dri/vc4_dri.so
do
	if [ ! -e "${TARGET_DIR}${f}" ]; then
		echo "post-build.sh: ${f} is missing from the target rootfs" >&2
		exit 1
	fi
done

# Checked separately because the failure it catches is so indirect. Without
# glamor the modesetting driver has no acceleration path and never creates a
# DRI3 device, so Mesa's EGL fails with "Could not get DRI3 device" and the clock
# panics — on a board whose GPU is working perfectly. Buildroot decides this from
# whether libepoxy happens to be enabled, and will not rebuild an already-stamped
# X server when it later appears, so the built result is what has to be checked.
if [ ! -e "${TARGET_DIR}/usr/lib/xorg/modules/libglamoregl.so" ]; then
	echo "post-build.sh: libglamoregl.so is missing; enable BR2_PACKAGE_LIBEPOXY." >&2
	exit 1
fi

# And separately, that the driver which has to *load* glamor was actually
# compiled against it. These come apart: xorg-server configures with
# --disable-dependency-tracking, so regenerating dix-config.h does not
# invalidate any object files. A -reconfigure therefore builds a fresh glamor
# module and relinks a stale modesetting_drv.so that still reports "No glamor
# support in the X Server" at runtime. Only a dirclean fixes it.
# Say how much artwork shipped. Every image is decoded and held in memory at
# startup, so this number is worth seeing on each build.
echo "post-build.sh: artwork: $(find "${TARGET_DIR}/usr/share/chronopolis/assets" -type f 2>/dev/null | wc -l | tr -d ' ') files"

# matchbox asks Xft for "Sans" at startup and exits if it cannot be resolved.
# Without a window manager nothing honours the clock's fullscreen request, so a
# missing font shows up as a blank screen rather than as ugly text.
if ! find "${TARGET_DIR}/usr/share/fonts" -iname '*.ttf' 2>/dev/null | grep -q .; then
	echo "post-build.sh: no scalable font in the image; matchbox will die on its theme." >&2
	echo "  Enable BR2_PACKAGE_DEJAVU." >&2
	exit 1
fi

MODESETTING="${TARGET_DIR}/usr/lib/xorg/modules/drivers/modesetting_drv.so"

if ! grep -aq "glamor_egl" "${MODESETTING}"; then
	echo "post-build.sh: modesetting_drv.so was compiled without glamor support." >&2
	echo "  Stale objects. Run: xserver_xorg-server-dirclean xserver_xorg-server-rebuild" >&2
	exit 1
fi

# ...and that it links libgbm. xorg-server's autotools build computes GBM_LIBS
# and then leaves it out of modesetting_drv_la_LIBADD, so the driver ships with
# unresolved gbm_* symbols and the server refuses to load it: "no screens found".
# patches/xserver_xorg-server/ fixes this; the check is here because the symptom
# looks nothing like the cause.
if ! grep -aq "libgbm.so.1" "${MODESETTING}"; then
	echo "post-build.sh: modesetting_drv.so does not link libgbm; it will fail to load." >&2
	echo "  Check that patches/xserver_xorg-server/ was applied." >&2
	exit 1
fi
