#!/bin/bash
# Minimal sysroot extraction script for Raspberry Pi

# Set the Raspberry Pi SSH connection
PI_USER="kevin"
PI_HOST="192.168.1.126"

# Local path for the sysroot directory
SYSROOT_DIR="./sysroot"

# Create target directories
mkdir -p "${SYSROOT_DIR}/usr/include"
mkdir -p "${SYSROOT_DIR}/lib"
mkdir -p "${SYSROOT_DIR}/usr/lib"

# Rsync options for minimal extraction
RSYNC_OPTS="-avz --delete"

# Sync the Pi's /usr/include, /lib, and /usr/lib directories
rsync ${RSYNC_OPTS} "${PI_USER}@${PI_HOST}:/usr/include/" "${SYSROOT_DIR}/usr/include/"
rsync ${RSYNC_OPTS} "${PI_USER}@${PI_HOST}:/lib/" "${SYSROOT_DIR}/lib/"
rsync ${RSYNC_OPTS} "${PI_USER}@${PI_HOST}:/usr/lib/" "${SYSROOT_DIR}/usr/lib/"