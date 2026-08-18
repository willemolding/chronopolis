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
# use. Trades against the cma= size in board/chronopolis/config.txt.
CHRONOPOLIS_ASSET_MAX_DIM = 1024

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
