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

The result is `firmware/artifacts/sdcard.img`. Flash it with
[Raspberry Pi Imager](https://www.raspberrypi.com/software/) ("Use custom") or:

```shell
sudo dd if=firmware/artifacts/sdcard.img of=/dev/diskN bs=4m status=progress
```

The first build takes roughly one to three hours — it compiles a toolchain, a kernel, Mesa and X.org from source. Downloads and ccache live in Docker volumes that `make clean` leaves alone, so later builds are much shorter; `make distclean` drops those too.

Day to day you only change the Rust or the artwork, and that is:

```shell
make -C firmware app        # rebuild just chronopolis, then refresh the image
```

Other targets: `menuconfig` to tune the config, `savedefconfig` to dump a
normalised copy for diffing against the committed one, `reconfig` to start over
from that committed one, `shell` for a prompt inside the builder, `clean` /
`distclean`.

`configs/chronopolis_defconfig` is maintained by hand, and `savedefconfig`(which writes `artifacts/defconfig`) deliberately does not overwrite it — Buildroot's normalised output strips every comment, and those comments are the only record of why any of it is set the way it is.

### If the build is unhappy

`make config` ends with a `config check`. It exists because `make <foo>_defconfig`
drops any symbol whose dependencies are not met and says nothing about it, so a
defconfig can quietly become something quite different from what it reads as. If
that check fails, the named symbols are the place to look — not the build log an
hour later.

**Changing a config option does not reliably rebuild what depends on it.**
Buildroot leaves already-stamped packages alone when a new dependency appears,
and it configures autotools packages with `--disable-dependency-tracking`, so
even a `<pkg>-reconfigure` will regenerate headers and then relink stale object
files against them. Enabling `libepoxy` this way produced an X server with a
fresh glamor module and a `modesetting_drv.so` still compiled without glamor —
which fails only at runtime, on the board. After changing anything a built
package depends on, use `<pkg>-dirclean <pkg>-rebuild`, not `-reconfigure`.

`board/chronopolis/post-build.sh` asserts on built artefacts rather than config
symbols for exactly this reason, and it is worth extending whenever a new
"silently built wrong" case turns up.

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

### Why the build tree is in a Docker volume, not a bind mount

It would be more convenient to have `output/` sitting on the host where you can
poke at it. It does not work. Docker Desktop's macOS file sharing mishandles
`fchmod()` on an open descriptor: the file is left permanently un-chmod-able,
returning `EPERM` on a later `chmod()` even to its owner, and even though `stat`
reports the right uid and mode. e2fsprogs' `subst` tool does exactly that —
`fchmod(fd, 0444)` then `rename()` — so `host-e2fsprogs` cannot build, and it is
a hard dependency of the ext4 image. A path-based `chmod()` before the rename is
unaffected, which is what makes this so easy to miss when probing by hand.

The build tree, downloads and ccache therefore live in named volumes, which are
ext4 inside the VM and behave correctly. Only the source (read-only, in) and
`artifacts/` (out) cross the boundary. To look inside the build tree, use
`make shell`.

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
| Graphics | Mesa with the gallium **vc4** driver, on the **fake-KMS** (`vc4-fkms-v3d`) stack |
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

### Frame rate

The clock is **fill-rate bound, not CPU bound**. A face is four full-canvas
bilinear-filtered alpha-blended quads, and the VideoCore IV shares one ~1.2GB/s
LPDDR2 bus with the ARM core. Filling a 1080px canvas four times over is ~55MB
of memory traffic per frame, which is essentially the whole budget — hence low
double-digit frames per second on a 1080p panel, and hence the ranking below.
Profiling the Rust is a waste of an afternoon; the GPU is waiting on memory.

Measure before and after. `CHRONOPOLIS_FPS=1` in the environment makes the clock
print an averaged frame rate every five seconds; add it to the `exec` line in
[`xinitrc`](board/chronopolis/rootfs_overlay/usr/share/chronopolis/xinitrc),
rebuild, and read it back from `/tmp/chronopolis.log`. Take it out afterwards —
the log lives on a tmpfs and nothing rotates it.

**1. Render fewer pixels.** `framebuffer_width` / `framebuffer_height` in
[`config.txt`](board/chronopolis/config.txt). In fake-KMS mode the firmware's
hardware scaler upscales the framebuffer to the negotiated HDMI mode for free,
so halving both edges quarters the per-frame traffic and costs only sharpness.
Set to 960x540 (half of 1080p). Match the panel's aspect ratio or the clock face
comes out elliptical. This is worth more than everything below combined.

**2. Match texture size to canvas size.** `CHRONOPOLIS_ASSET_MAX_DIM` in
[`chronopolis.mk`](package/chronopolis/chronopolis.mk), now 512. Textures larger
than the canvas are minified, and with a texture cache this small that turns
every bilinear tap into a fresh memory read. Raise it and the framebuffer
together, or not at all.

**3. Uncapped presentation.** `swap_interval: Some(0)` in `window_conf`
(`src/main.rs`). miniquad otherwise defaults to `eglSwapInterval(1)`, which
quantises the frame rate to 60/n — a frame that misses 60Hz by a hair displays
at 30. Costs a faint tearing seam; set it back to `Some(1)` if that reads worse
on the panel than the lower frame rate does.

**4. Crop the artwork.** Not done, and the largest remaining win. The three hand
layers are full-canvas PNGs that are mostly transparent, and a transparent pixel
costs the same to shade as an opaque one. Cropping each to the smallest *centred*
square containing opaque pixels (centred, so rotation about the origin still
works) and drawing it at the corresponding fraction of the canvas would cut the
shaded area of a typical face by roughly half. It needs the crop factor carried
from `install-assets.sh` through to `draw_texture_centred`, which is why it is a
note rather than a patch.

`sample_count` in `src/main.rs` must stay at 1: 4x multisampling at 1920x1080
allocated ~86MB and drew nothing on this GPU.

### Memory

`CHRONOPOLIS_ASSET_MAX_DIM` and `gpu_mem=128` in
[`config.txt`](board/chronopolis/config.txt) trade against each other: the board
has 512MB shared between the ARM core and the firmware, which owns the display
in fake-KMS mode. Every image under `assets/` is decoded to RGBA and held for
the whole run, out of the 192MB CMA pool sized in `linux.fragment` — at 1024px
that was ~104MB of artwork, at 512px it is ~26MB.

If textures fail to allocate, raise `gpu_mem`; if userspace starts hitting OOM,
lower it. If it still will not fit, the real fix is loading each face's artwork
on demand rather than all of it at boot, which is a change to `src/textures.rs`
rather than to the image. Note that `textures.rs` currently loads *everything*
under `assets/`, including art for faces not registered in `faces::all()`.

### Display

HDMI mode is left on auto-detect. To pin it for a specific panel, uncomment the
`hdmi_group` / `hdmi_mode` / `hdmi_cvt` block in `board/chronopolis/config.txt`.

## Debugging a board that will not start

There is no network, by design. Three ways in, in order of how little hardware
they need.

**1. The screen itself.** If X or the clock exits, the VT drops back to text mode
and [`show-failure.sh`](board/chronopolis/rootfs_overlay/usr/share/chronopolis/show-failure.sh)
prints the exit status, whether `/dev/dri` exists, and the tail of both logs —
then holds it for 15 seconds before the respawn retries, so it stays readable.

**2. A USB keyboard.** The Pi Zero W's data port is OTG, so a keyboard plus an
adapter gets you a login shell on **Alt+F2** or **Alt+F3** (root, no password).
X takes vt7; tty1 stays the kernel console.

```shell
ls /dev/dri              # missing => the vc4 driver never bound
cat /tmp/Xorg.0.log      # X server log (rootfs is read-only, so it lives here)
cat /tmp/chronopolis.log # the clock's own output, and anything start.sh printed
```

**3. A serial console**, if you have a USB-TTL adapter: `ttyAMA0`, 115200 8N1,
on GPIO 14/15 plus ground.

### If the screen is black and every log says everything is fine

That combination cost a long afternoon, so: the display pipeline is the first
thing to doubt, not the last. `dtoverlay=vc4-kms-v3d` (full KMS) on this board
produces a working `/dev/dri/card0`, a glamor-accelerated X server on "VC4 V3D
2.1", a correct mode on the correct output, no errors anywhere — and nothing on
the panel. `vc4-fkms-v3d` fixes it outright. See the comments in
[`board/chronopolis/config.txt`](board/chronopolis/config.txt).

The test that made that obvious is worth remembering: boot, and write noise
straight into the framebuffer with X out of the picture.

```shell
dd if=/dev/urandom of=/dev/fb0 bs=1M count=16
```

Static on screen means the kernel's scanout path works and the fault is above
it. Nothing means it is below. That one command halves the search space.

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
