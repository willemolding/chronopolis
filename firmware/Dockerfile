# syntax=docker/dockerfile:1

# Buildroot's own CI container. It already carries every host dependency
# Buildroot needs, and it is the image the Buildroot project actually keeps
# current — unlike buildroot/base on Docker Hub, whose newest tag is from 2021.
FROM registry.gitlab.com/buildroot.org/buildroot/base:20250218.2110

# 2026.05.1 ships Rust 1.96.1, which this crate needs for edition 2024.
ARG BUILDROOT_VERSION=2026.05.1

USER root

# The one thing the base image lacks: ImageMagick, used by
# board/chronopolis/install-assets.sh to fit the artwork inside the VideoCore IV
# texture limit on the way into the rootfs.
RUN apt-get update \
 && apt-get install -y --no-install-recommends imagemagick \
 && rm -rf /var/lib/apt/lists/*

# Buildroot itself is baked in and pinned, so the image is the version lock.
# The project source is bind-mounted at run time instead, so editing src/ or
# assets/ never invalidates this layer.
RUN git clone --depth 1 --branch "${BUILDROOT_VERSION}" \
        https://github.com/buildroot/buildroot.git /opt/buildroot

# Everything is built out of tree into /output, so /opt/buildroot is only ever
# read. That is what lets the container run as an arbitrary host UID.
WORKDIR /output
