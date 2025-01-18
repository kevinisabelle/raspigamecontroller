#!/bin/bash

# Variables
SYSROOT_DIR="/home/kevin/sysroot"
SYSROOT_ARCHIVE="/home/kevin/sysroot_rpi.zip"

# Create a clean sysroot directory
echo "Creating sysroot directory..."
rm -rf "$SYSROOT_DIR"
mkdir -p "$SYSROOT_DIR"

# Copy necessary directories
echo "Copying system files to sysroot..."
rsync -avz --progress /lib "$SYSROOT_DIR"
rsync -avz --progress /usr "$SYSROOT_DIR"
rsync -avz --progress /opt "$SYSROOT_DIR"

# Remove unnecessary files (optional)
echo "Cleaning up unnecessary files..."
rm -rf "$SYSROOT_DIR/usr/share/doc"
rm -rf "$SYSROOT_DIR/usr/share/man"
rm -rf "$SYSROOT_DIR/usr/share/info"
rm -rf "$SYSROOT_DIR/var/cache/apt/archives"

# Compress the sysroot into a zip archive
echo "Compressing sysroot into $SYSROOT_ARCHIVE..."
zip -r "$SYSROOT_ARCHIVE" "$SYSROOT_DIR"

# Final message
echo "Sysroot archive created: $SYSROOT_ARCHIVE"
echo "You can now copy it to your Windows machine."