#!/usr/bin/env bash
# Build README demo GIF from maestro-activate-demo.mp4 (palette-optimized).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MP4="${ROOT}/docs/screenshots/maestro-activate-demo.mp4"
GIF_OUT="${ROOT}/docs/screenshots/maestro-activate-demo.gif"
PALETTE="$(mktemp /tmp/maestro-demo-palette.XXXXXX.png)"

if [[ ! -f "$MP4" ]]; then
  echo "Missing $MP4 — run optimize-demo-video.sh or capture-demo-video.mjs first." >&2
  exit 1
fi

FFMPEG="${FFMPEG:-}"
if [[ -z "$FFMPEG" ]]; then
  if command -v ffmpeg >/dev/null 2>&1; then
    FFMPEG="$(command -v ffmpeg)"
  elif [[ -x "${ROOT}/.tools/ffmpeg" ]]; then
    FFMPEG="${ROOT}/.tools/ffmpeg"
  fi
fi
if [[ -z "${FFMPEG}" || ! -x "${FFMPEG}" ]]; then
  echo "ffmpeg is required (sudo apt install ffmpeg, or place a binary at .tools/ffmpeg)" >&2
  exit 1
fi

trap 'rm -f "$PALETTE"' EXIT

echo "Generating palette..."
"$FFMPEG" -y -i "$MP4" \
  -vf "fps=12,scale=1280:-1:flags=lanczos,palettegen=stats_mode=diff" \
  -frames:v 1 -update 1 "$PALETTE"

echo "Writing $GIF_OUT ..."
"$FFMPEG" -y -i "$MP4" -i "$PALETTE" \
  -lavfi "fps=12,scale=1280:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer:bayer_scale=5" \
  "$GIF_OUT"

ls -lh "$GIF_OUT"
