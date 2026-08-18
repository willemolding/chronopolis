# Chronopolis firmware — Raspberry Pi Zero W

Builds a bootable SD card image that powers on straight into the clock — no
desktop, no login, no operator.

This directory is a [Buildroot](https://buildroot.org) *br2-external* tree in
the standard layout (`external.desc`, `external.mk`, `Config.in`, plus
`configs/`, `package/` and `board/`), with a pinned container alongside it so
the build is reproducible and lands nothing on your machine. Nothing here
patches Buildroot itself.

## Build it

```shell
make -C firmware config     # build the container, generate the Buildroot config
make -C firmware image      # the long one
```

The result is `firmware/output/images/sdcard.img`. Flash it with
[Raspberry Pi Imager](https://www.raspberrypi.com/software/) ("Use custom") or:

```shell
sudo dd if=firmware/output/images/sdcard.img of=/dev/diskN bs=4m status=progress
```

The first build takes roughly one to three hours — it compiles a kernel, Mesa
and X.org from source. Downloads and ccache live in `firmware/.cache/`, which
`make clean` leaves alone, so later builds are much shorter.

Day to day you only change the Rust or the artwork, and that is:

```shell
make -C firmware app        # rebuild just chronopolis, then refresh the image
```

Other targets: `menuconfig` to tune the config, `savedefconfig` to dump a
normalised copy for diffing against the committed one, `reconfig` to start over
from that committed one, `shell` for a prompt inside the builder, `clean` /
`distclean`.

`configs/chronopolis_defconfig` is maintained by hand, and `savedefconfig`
deliberately does not overwrite it — Buildroot's normalised output strips every
comment, and those comments are the only record of why any of it is set the way
it is.

### If the build is unhappy

`make config` ends with a `config check`. It exists because `make <foo>_defconfig`
drops any symbol whose dependencies are not met and says nothing about it, so a
defconfig can quietly become something quite different from what it reads as. If
that check fails, the named symbols are the place to look — not the build log an
hour later.

On Docker Desktop, give the VM a decent amount of memory and CPU (Settings →
Resources); a parallel Buildroot run is heavy, and on Apple Silicon it has been
seen to trip a kernel oops in the Docker VM itself. If that happens, lower the
parallelism:

```shell
make -C firmware image JOBS=4
```

The container runs natively on both amd64 and arm64 hosts — the Buildroot base
image is multi-arch, and the toolchain is built from source rather than
downloaded as an x86_64 binary, so there is no emulation involved either way.

### Without the container

The container is a convenience, not a dependency. With Buildroot already
checked out, and ImageMagick on the host:

```shell
make -C /path/to/buildroot O=$PWD/output BR2_EXTERNAL=$PWD/firmware \
     chronopolis_defconfig
make -C /path/to/buildroot O=$PWD/output all
```

## What's in the image

| | |
|---|---|
| Base | Buildroot 2026.05.1, `raspberrypi0w_defconfig` |
| Toolchain | Buildroot's own, glibc, ARMv6 EABIhf → `arm-unknown-linux-gnueabihf` |
| Rust | 1.96.1, as shipped by Buildroot — the same version `rust-toolchain.toml` pins |
| Graphics | Mesa with the gallium **vc4** driver, on the open `vc4-kms-v3d` KMS stack |
| Display server | X.org, plus matchbox as the window manager |
| Init | BusyBox, with the clock as a `respawn` entry |
| Root filesystem | ext4, **read-only** |
| Networking | none |

## Why each of the odd-looking pieces exists

**An X server, on an appliance.** macroquad renders through miniquad, whose only
stable Linux backend is X11 (`LinuxBackend::X11Only` is its default; the Wayland
backend is documented as unstable). It `dlopen`s `libX11`, `libXi`,
`libxkbcommon` and `libGL`/`libEGL` at runtime rather than linking them, so a
missing library is a silent startup failure on the bench, not a build error —
which is why [`board/chronopolis/post-build.sh`](board/chronopolis/post-build.sh)
checks for all four.

**A window manager, for a single fullscreen app.** miniquad requests fullscreen
by sending an EWMH `_NET_WM_STATE_FULLSCREEN` client message. With no window
manager running, nothing acts on that message and the clock draws in a
1024×1024 corner of the screen. matchbox is about 200KB and exists solely to
answer it.

**Downscaled artwork.** `assets/clocktown/*.png` are 3000×3000, and VideoCore IV
rejects textures above 2048×2048 — those faces would come up blank. Separately,
`src/textures.rs` loads all 26 images at startup and holds them decoded, which at
source resolution is around 200MB of RGBA on a board with 512MB shared with the
GPU. [`board/chronopolis/install-assets.sh`](board/chronopolis/install-assets.sh)
shrinks anything over the cap on the way into the rootfs. The source `assets/`
directory is never touched.

**A read-only root.** An installation gets unplugged, not shut down. A clock
that cannot corrupt its own filesystem is a clock that still works next week.

## Tuning

The two settings that matter, and they trade against each other:

- `CHRONOPOLIS_ASSET_MAX_DIM` in
  [`package/chronopolis/chronopolis.mk`](package/chronopolis/chronopolis.mk) —
  largest edge length any texture may have. Default 1024.
- `cma-192` in [`board/chronopolis/config.txt`](board/chronopolis/config.txt) —
  the memory pool those textures are allocated from. Keep
  `CONFIG_CMA_SIZE_MBYTES` in `board/chronopolis/linux.fragment` at least this
  large.

If textures fail to allocate, raise CMA or lower the cap. If userspace starts
hitting OOM, lower CMA. If it still will not fit, the real fix is loading each
face's artwork on demand instead of all of it at boot, which is a change to
`src/textures.rs` rather than to the image.

HDMI mode is left on auto-detect. To pin it for a specific panel, uncomment the
`hdmi_group` / `hdmi_mode` / `hdmi_cvt` block in `board/chronopolis/config.txt`.

## Getting into a running system

There is no network, by design. The serial console on the UART pins
(`ttyAMA0`, 115200 8N1) is the way in — GPIO 14/15, plus ground, into any
USB-serial adapter. The kernel logs there and there is a getty waiting.

Useful once you are in:

```shell
dmesg | grep vc4          # did the KMS driver bind?
cat /tmp/Xorg.0.log       # X server log (the rootfs is read-only, so it lives here)
```

If the clock is slow, check that Mesa picked the vc4 renderer and not a software
one — a software fallback means the vc4 path failed quietly, and the framerate
will show it.

## Known limitation: the time is wrong after a power cut

The Pi Zero W has no real-time clock, and this image has no networking, so there
is no NTP either. Every boot starts from the epoch. The clock runs correctly; it
is just offset. Given that the project's own description is "It might tell the
time, it might not", this may be exactly right.

If you would rather it were accurate, the cheap fix is a DS3231 module on the
I²C header — about four lines, no networking:

- `dtoverlay=i2c-rtc,ds3231` in `board/chronopolis/config.txt`
- `BR2_PACKAGE_BUSYBOX_SHOW_OTHERS` already gives you `hwclock`; add
  `hwclock -s` as a `sysinit` line in `board/chronopolis/rootfs_overlay/etc/inittab`
- enable I²C in `board/chronopolis/linux.fragment`
  (`CONFIG_I2C_BCM2835=y`, `CONFIG_RTC_DRV_DS1307=y`)
