#!/bin/sh
#
# Run by init as a `respawn` entry, so this script exiting means the clock
# restarts. Deliberately not `exec`: returning here gives us the failure report
# and the sleep below.

# Everything this script and its children print lands in one file, which
# show-failure.sh reads back if the clock stops.
exec > /tmp/chronopolis.log 2>&1

export HOME=/root
# The rootfs is read-only, so anything that wants a scratch directory has to be
# pointed at the tmpfs.
export XDG_RUNTIME_DIR=/tmp

# An X server that dies without cleaning up leaves these behind, and every
# respawn after it then fails with "Server is already active for display 0" —
# one unclean exit wedges an unattended installation permanently. Nothing else
# here uses display :0, so clearing them is always safe.
rm -f /tmp/.X0-lock /tmp/.X11-unix/X0

# -logfile /tmp: Xorg's default is /var/log, which is not writable here.
# -nocursor:     no pointer on a gallery screen.
# -s 0:          never blank the screen.
# -dpms:         disable display power management entirely.
#
# The last two matter more than they look. consoleblank=0 on the kernel command
# line only stops the *kernel* console blanker; the X server has its own
# screensaver and DPMS, both enabled by default and both firing after ten idle
# minutes. The clock produces no input events — there is no keyboard and no
# pointer — so X considers the session permanently idle and powers the panel
# down on schedule. An installation that goes dark after ten minutes is worse
# than one that never starts, because it looks like it worked.
#
# X gets vt7, its own VT, and is allowed to switch to it. Running it on vt1 —
# the VT the kernel console draws to via fbcon — while passing -novtswitch to
# decline taking that VT over is not how X is meant to be run.
/usr/bin/xinit /usr/share/chronopolis/xinitrc -- :0 vt7 \
	-nocursor -s 0 -dpms -logfile /tmp/Xorg.0.log
rc=$?

# Put the reason on the screen. X has exited by now, so the VT is back in text
# mode and this lands on the HDMI output — which on a board with no network and
# no serial adapter attached is the only place anyone will see it.
/usr/share/chronopolis/show-failure.sh "$rc" > /dev/tty1 2>&1 \
	|| /usr/share/chronopolis/show-failure.sh "$rc" > /dev/console 2>&1

# Long enough to read, and it stops init from being told off for respawning too
# often. The report is reprinted on every retry, so it stays on screen.
sleep 15
