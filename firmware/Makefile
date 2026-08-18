# Builds the Raspberry Pi Zero W SD card image.
#
#   make image        build firmware/output/images/sdcard.img   (slow, first time)
#   make app          rebuild just the Rust app and refresh the image
#   make menuconfig   tune the Buildroot config interactively
#   make shell        a shell in the build container
#
# Everything runs inside the container defined by ./Dockerfile; nothing but
# Docker is needed on this machine.

BUILDROOT_VERSION ?= 2026.05.1

export BUILDROOT_VERSION
export DOCKER_UID := $(shell id -u)
export DOCKER_GID := $(shell id -g)

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

# Bind-mount sources, created here so they end up owned by you rather than by
# the Docker daemon.
DIRS = output .cache/dl .cache/home

.PHONY: help dirs config reconfig check-config image app menuconfig savedefconfig shell clean distclean

# `\?` is a GNU sed extension, and this Makefile runs on the host, which may be
# a Mac. Two plain expressions instead.
help:
	@sed -n '1,9p' $(firstword $(MAKEFILE_LIST)) | sed -e 's/^# //' -e 's/^#$$//'

dirs:
	@mkdir -p $(DIRS)

# Buildroot stores the config in output/, so only run defconfig if it is absent.
# Re-running it would silently discard anything done via `make menuconfig`.
config: dirs
	@test -f output/.config || $(RUN) $(BR) chronopolis_defconfig
	@$(MAKE) --no-print-directory check-config

# Start over from configs/chronopolis_defconfig, discarding menuconfig changes.
reconfig: dirs
	rm -f output/.config
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
	BR2_PACKAGE_MATCHBOX=y

check-config:
	@missing=$$(for s in $(REQUIRED_SYMBOLS); do \
		grep -qx "$$s" output/.config || echo "    $$s"; done); \
	if [ -n "$$missing" ]; then \
		printf 'config check: the defconfig degraded, these were dropped:\n%s\n' \
			"$$missing" >&2; \
		exit 1; \
	fi; \
	echo "config check: ok"

image: config
	$(RUN) $(BR) all
	@echo
	@echo "Image: firmware/output/images/sdcard.img"

# The app is the only thing that changes day to day. This skips straight to it
# rather than reconsidering the kernel, Mesa and X every time.
app: config
	$(RUN) $(BR) chronopolis-rebuild all

menuconfig: config
	$(RUN) $(BR) menuconfig

# Deliberately does NOT overwrite configs/chronopolis_defconfig. Buildroot's
# savedefconfig emits a minimal, normalised config with every comment stripped,
# and those comments are the only record of why any of this is set the way it
# is. Diff against it and port changes over by hand.
savedefconfig: config
	$(RUN) $(BR) savedefconfig BR2_DEFCONFIG=/output/defconfig
	@echo
	@echo "Wrote output/defconfig. To fold changes into the committed config:"
	@echo "    diff -u configs/chronopolis_defconfig output/defconfig"

shell: dirs
	$(RUN) bash

# Drops the build tree but keeps the download and ccache volumes, so the next
# build is slow rather than glacial.
clean:
	rm -rf output

# ...and this drops those too.
distclean: clean
	rm -rf .cache
