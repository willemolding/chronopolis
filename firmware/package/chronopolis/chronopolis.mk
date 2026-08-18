################################################################################
#
# chronopolis
#
################################################################################

# Built straight out of this repository rather than a release tarball: the code
# and the artwork are versioned together and there is nothing published to
# download. SITE_METHOD = local makes Buildroot rsync the tree into the build
# directory, so the source checkout is never written to.
CHRONOPOLIS_VERSION = local
CHRONOPOLIS_SITE = $(BR2_EXTERNAL_CHRONOPOLIS_PATH)/..
CHRONOPOLIS_SITE_METHOD = local

# Without these every build rsyncs the host's Cargo build artifacts and any
# previously built images — gigabytes of churn for nothing.
CHRONOPOLIS_OVERRIDE_SRCDIR_RSYNC_EXCLUSIONS = \
	--exclude target \
	--exclude firmware/output \
	--exclude firmware/.cache

# Largest edge length any texture is allowed to have on the target. VideoCore IV
# rejects textures above 2048px, and every image under assets/ is decoded to RGBA
# and held in memory for the whole run, so this is also the main lever on memory
# use. Trades against gpu_mem in board/chronopolis/config.txt.
#
# Keep this at or just above the canvas edge implied by framebuffer_width /
# framebuffer_height in board/chronopolis/config.txt. Oversized textures cost
# twice: they are minified, which on a GPU with a texture cache this small means
# every bilinear tap is a fresh memory read, and they occupy four times the CMA
# for each doubling. 512 suits the 960x540 framebuffer that config.txt sets;
# raise both together if the artwork needs to be sharper.
CHRONOPOLIS_ASSET_MAX_DIM = 512

# pkg-cargo normally vendors dependencies during the download step, which does
# not run for a local package — but it still builds with `cargo --offline`. So
# vendor here instead, into the build directory. The crate downloads themselves
# land in $(BR_CARGO_HOME) under $(BR2_DL_DIR), which the container keeps as a
# cache across builds.
define CHRONOPOLIS_VENDOR_DEPENDENCIES
	cd $(@D) && \
	$(TARGET_MAKE_ENV) \
		$(PKG_COMMON_CARGO_ENV) \
		cargo vendor --locked --versioned-dirs VENDOR
	mkdir -p $(@D)/.cargo
	printf '%s\n' \
		'[source.crates-io]' \
		'replace-with = "vendored-sources"' \
		'' \
		'[source.vendored-sources]' \
		'directory = "VENDOR"' \
		> $(@D)/.cargo/config.toml
endef
CHRONOPOLIS_PRE_BUILD_HOOKS += CHRONOPOLIS_VENDOR_DEPENDENCIES

# cargo-package installs the binary to /usr/bin/chronopolis. The artwork is not
# something Cargo knows about, so install it here, downscaling on the way.
define CHRONOPOLIS_INSTALL_ASSETS
	$(BR2_EXTERNAL_CHRONOPOLIS_PATH)/board/chronopolis/install-assets.sh \
		$(@D)/assets \
		$(TARGET_DIR)/usr/share/chronopolis/assets \
		$(CHRONOPOLIS_ASSET_MAX_DIM)
endef
CHRONOPOLIS_POST_INSTALL_TARGET_HOOKS += CHRONOPOLIS_INSTALL_ASSETS

$(eval $(cargo-package))
