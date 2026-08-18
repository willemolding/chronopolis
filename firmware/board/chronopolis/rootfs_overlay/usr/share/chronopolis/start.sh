#!/bin/sh
#
# Run by init as a `respawn` entry, so this script exiting means the clock
# restarts. Deliberately not `exec`: returning here gives us the sleep below.

export HOME=/root
# The rootfs is read-only, so anything that wants a scratch directory has to be
# pointed at the tmpfs.
export XDG_RUNTIME_DIR=/tmp

# -logfile /tmp: Xorg's default is /var/log, which is not writable here.
# -nocursor:     no pointer on a gallery screen.
# -novtswitch:   nothing else is competing for the VT.
/usr/bin/xinit /usr/share/chronopolis/xinitrc -- :0 vt1 \
	-nocursor -novtswitch -logfile /tmp/Xorg.0.log

# If X or the clock died immediately — a bad mode, a GL failure — this stops
# init from spinning on it fast enough to be told off for respawning too often.
sleep 2
