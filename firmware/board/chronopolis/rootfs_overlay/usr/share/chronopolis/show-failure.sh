#!/bin/sh
#
# Print a readable failure report on the console.
#
# When X exits, the VT drops back to text mode, so whatever is written here
# appears on the HDMI output. With no network and no serial adapter attached,
# that is the only place anyone is going to see why the clock stopped.
#
# start.sh calls this with xinit's exit status and redirects it to /dev/tty1.

rc="${1:-unknown}"

# `clear` is not guaranteed to be in this busybox; the escapes always are.
printf '\033[2J\033[H'

echo "=============================================================="
echo " chronopolis did not start — xinit exited $rc"
echo "=============================================================="
echo
echo "-- /dev/dri  (missing or empty => the vc4 driver never bound) --"
ls /dev/dri 2>&1 | head -5
echo
echo "-- Xorg.0.log, last 12 lines --"
tail -n 12 /tmp/Xorg.0.log 2>/dev/null || echo "   (no Xorg log — X never got far enough)"
echo
echo "-- clock output, last 8 lines --"
tail -n 8 /tmp/chronopolis.log 2>/dev/null || echo "   (no output)"
echo
echo "Log in as root on Alt+F2 for the full picture."
