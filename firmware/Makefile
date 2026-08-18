# Builds the Raspberry Pi Zero W SD card image.
#
#   make image        build firmware/artifacts/sdcard.img   (slow, first time)
#   make app          rebuild just the Rust app and refresh the image
#   make menuconfig   tune the Buildroot config interactively
#   make shell        a shell in the build container
#
# Everything runs inside the container defined by ./Dockerfile; nothing but
# Docker is needed on this machine.

BUILDROOT_VERSION ?= 2026.05.1

DOCKER  ?= docker
COMPOSE ?= docker compose
RUN      = $(COMPOSE) run --rm builder

# Cap build parallelism, e.g. `make image JOBS=4`. Buildroot otherwise uses
# nproc+1, which can be more than the Docker VM enjoys. Passed as a make
# variable rather than an environment one: BR2_JLEVEL comes from .config, and
# only a command-line override beats that.
JOBS ?=

# Out of tree, so the pinned Buildroot checkout in the image stays pristine.
# This directory is the br2-external tree.
BR = make -C /opt/buildroot O=/output BR2_EXTERNAL=/src/firmware \
     $(if $(JOBS),BR2_JLEVEL=$(JOBS))

# The build tree lives in a named volume, not on the host — see compose.yaml for
# why it has to. That means `clean` is volume management rather than rm -rf.
VOLUME_BUILD = chronopolis-output
VOLUME_CACHE = chronopolis-dl chronopolis-home

.PHONY: help setup config reconfig check-config image app artifact menuconfig \
        savedefconfig shell clean distclean

# `\?` is a GNU sed extension, and this Makefile runs on the host, which may be
# a Mac. Two plain expressions instead.
help:
	@sed -n '1,9p' $(firstword $(MAKEFILE_LIST)) | sed -e 's/^# //' -e 's/^#$$//'

# docker compose loads .env from this directory automatically, so a bare
# `docker compose ...` here behaves exactly as going through this Makefile does.
# That matters: without it the two paths run as different UIDs, and the volumes
# are owned by whichever got there first.
setup:
	@mkdir -p artifacts
	@printf 'DOCKER_UID=%s\nDOCKER_GID=%s\nBUILDROOT_VERSION=%s\n' \
		"$$(id -u)" "$$(id -g)" "$(BUILDROOT_VERSION)" > .env

# Only run defconfig if there is no config yet. Re-running it would silently
# discard anything done via `make menuconfig`.
config: setup
	@$(RUN) test -f /output/.config >/dev/null 2>&1 \
		|| $(RUN) $(BR) chronopolis_defconfig
	@$(MAKE) --no-print-directory check-config

# Start over from configs/chronopolis_defconfig, discarding menuconfig changes.
reconfig: setup
	-@$(RUN) rm -f /output/.config >/dev/null 2>&1
	@$(MAKE) --no-print-directory config

# `make <foo>_defconfig` drops any symbol whose dependencies are not met, and
# says nothing about it. A defconfig can therefore degrade into something quite
# different from what it reads as — an unavailable toolchain quietly becomes an
# empty external one, and you find out an hour into the build. Assert the
# handful of settings everything else rests on.
REQUIRED_SYMBOLS = \
	BR2_arm1176jzf_s=y \
	BR2_ARM_EABIHF=y \
	BR2_TOOLCHAIN_BUILDROOT_GLIBC=y \
	BR2_TOOLCHAIN_BUILDROOT_CXX=y \
	BR2_PACKAGE_CHRONOPOLIS=y \
	BR2_PACKAGE_MESA3D_GALLIUM_DRIVER_VC4=y \
	BR2_PACKAGE_MESA3D_OPENGL_GLX=y \
	BR2_PACKAGE_XSERVER_XORG_SERVER=y \
	BR2_PACKAGE_XLIB_LIBXSHMFENCE=y \
	BR2_PACKAGE_LIBEPOXY=y \
	BR2_PACKAGE_MATCHBOX=y \
	BR2_PACKAGE_DEJAVU=y

check-config:
	@missing=$$($(RUN) sh -c 'for s in $(REQUIRED_SYMBOLS); do \
		grep -qx "$$s" /output/.config || echo "    $$s"; done' 2>/dev/null); \
	if [ -n "$$missing" ]; then \
		printf 'config check: the defconfig degraded, these were dropped:\n%s\n' \
			"$$missing" >&2; \
		exit 1; \
	fi; \
	echo "config check: ok"

# board/chronopolis/{config,cmdline}.txt are inputs to the rpi-firmware package,
# and Buildroot does not track them as dependencies. Editing one and running
# `make image` would otherwise silently ship the previous version — an hour lost
# on the bench wondering why a boot parameter had no effect. Forcing the
# reinstall is only a file copy.
image: config
	$(RUN) $(BR) rpi-firmware-reinstall all
	@$(MAKE) --no-print-directory artifact

# The app is the only thing that changes day to day. This skips straight to it
# rather than reconsidering the kernel, Mesa and X every time.
#
# dirclean, not just -rebuild: Buildroot rsyncs a local package's source once
# and guards the copy with a stamp that -rebuild does not remove, so editing
# src/ and running `make image` — or even `make app` — can silently build the
# previous source. Wiping the build directory is the only reliable way to pick
# up an edit. It costs a full recompile of the crate; shipping a binary that
# does not match the source costs an afternoon.
app: config
	$(RUN) $(BR) chronopolis-dirclean chronopolis-rebuild all
	@$(MAKE) --no-print-directory artifact

# Copy the finished image out of the build volume onto the host.
artifact:
	@$(RUN) cp -a /output/images/sdcard.img /artifacts/
	@echo
	@echo "Image: firmware/artifacts/sdcard.img"

menuconfig: config
	$(RUN) $(BR) menuconfig

# Deliberately does NOT overwrite configs/chronopolis_defconfig. Buildroot's
# savedefconfig emits a minimal, normalised config with every comment stripped,
# and those comments are the only record of why any of this is set the way it
# is. Diff against it and port changes over by hand.
savedefconfig: config
	$(RUN) $(BR) savedefconfig BR2_DEFCONFIG=/artifacts/defconfig
	@echo
	@echo "Wrote artifacts/defconfig. To fold changes into the committed config:"
	@echo "    diff -u configs/chronopolis_defconfig artifacts/defconfig"

shell: setup
	$(RUN) bash

# Drops the build tree but keeps the download and ccache volumes, so the next
# build is slow rather than glacial.
clean:
	-@$(DOCKER) volume rm -f $(VOLUME_BUILD)
	rm -rf artifacts

# ...and this drops those too.
distclean: clean
	-@$(DOCKER) volume rm -f $(VOLUME_CACHE)
