#!/bin/bash
# Build AppImage from release binary
set -euo pipefail

BINARY="${1:?Usage: build-appimage.sh <path-to-felix-binary>}"
VERSION="${2:-0.1.0}"
ARCH="$(uname -m)"

APPDIR="AppDir"
rm -rf "${APPDIR}"
mkdir -p "${APPDIR}/usr/bin"
mkdir -p "${APPDIR}/usr/share/applications"
mkdir -p "${APPDIR}/usr/share/icons/hicolor/256x256/apps"

# Copy binary
cp "${BINARY}" "${APPDIR}/usr/bin/felix"
chmod +x "${APPDIR}/usr/bin/felix"

# Copy desktop file
cp packaging/felix.desktop "${APPDIR}/usr/share/applications/"
cp packaging/felix.desktop "${APPDIR}/"

# Create a simple icon (placeholder - replace with real icon later)
# For now, create a minimal PNG placeholder
if command -v convert &> /dev/null; then
    convert -size 256x256 xc:#4A90D9 -fill white -gravity center \
        -pointsize 120 -annotate 0 "F" \
        "${APPDIR}/usr/share/icons/hicolor/256x256/apps/felix.png"
    cp "${APPDIR}/usr/share/icons/hicolor/256x256/apps/felix.png" "${APPDIR}/felix.png"
else
    # Create a minimal 1x1 PNG as placeholder (ImageMagick not available)
    echo "Warning: ImageMagick not found, using placeholder icon"
    # Minimal valid PNG (1x1 transparent pixel)
    printf '\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\nIDATx\x9cc\x00\x01\x00\x00\x05\x00\x01\r\n-\xb4\x00\x00\x00\x00IEND\xaeB`\x82' > "${APPDIR}/felix.png"
    cp "${APPDIR}/felix.png" "${APPDIR}/usr/share/icons/hicolor/256x256/apps/felix.png"
fi

# Copy AppRun script
cp packaging/appimage/AppRun "${APPDIR}/AppRun"
chmod +x "${APPDIR}/AppRun"

# Copy required shared libraries
echo "Copying shared libraries..."
for lib in $(ldd "${APPDIR}/usr/bin/felix" 2>/dev/null | grep "=> /" | awk '{print $3}'); do
    # Skip libc, libm, ld-linux, libpthread, libdl, librt
    case "$(basename "$lib")" in
        libc.*|libm.*|ld-linux*|libpthread.*|libdl.*|librt.*|libresolv.*) continue ;;
    esac
    mkdir -p "${APPDIR}/usr/lib/$(dirname "$lib" | sed 's|^/||')"
    cp "$lib" "${APPDIR}/usr/lib/$(echo "$lib" | sed 's|^/||')" 2>/dev/null || true
done

# Download appimagetool if not present
if [ ! -f "appimagetool" ]; then
    echo "Downloading appimagetool..."
    wget -q "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-${ARCH}.AppImage" -O appimagetool
    chmod +x appimagetool
fi

# Build AppImage
# APPIMAGE_EXTRACT_AND_RUN=1 allows running without FUSE (needed in CI)
echo "Building AppImage..."
OUTPUT="felix-${VERSION}-${ARCH}.AppImage"
export APPIMAGE_EXTRACT_AND_RUN=1
./appimagetool "${APPDIR}" "${OUTPUT}"

echo "✅ AppImage created: ${OUTPUT}"
ls -lh "${OUTPUT}"
