#!/usr/bin/env bash
# Trim and compress a raw screen recording into README demo assets.
# Usage:
#   ./scripts/optimize-demo-video.sh /path/to/raw.mp4
#   START=2 END=28 ./scripts/optimize-demo-video.sh /path/to/raw.mp4
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="${ROOT}/docs/screenshots"
RAW="${1:?Usage: $0 /path/to/raw.mp4}"
START="${START:-0}"
END="${END:-}"

if [[ ! -f "$RAW" ]]; then
  echo "Input not found: $RAW" >&2
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

mkdir -p "$OUT_DIR"

TRIM_ARGS=()
if [[ -n "$START" && "$START" != "0" ]]; then
  TRIM_ARGS+=(-ss "$START")
fi
if [[ -n "$END" ]]; then
  TRIM_ARGS+=(-to "$END")
fi

MP4_OUT="${OUT_DIR}/maestro-activate-demo.mp4"
WEBM_OUT="${OUT_DIR}/maestro-activate-demo.webm"

echo "Writing $MP4_OUT ..."
"$FFMPEG" -y "${TRIM_ARGS[@]}" -i "$RAW" \
  -vf "scale=1280:-2" \
  -c:v libx264 -crf 28 -preset slow -pix_fmt yuv420p -movflags +faststart -an \
  "$MP4_OUT"

echo "Writing $WEBM_OUT ..."
"$FFMPEG" -y "${TRIM_ARGS[@]}" -i "$RAW" \
  -vf "scale=1280:-2" \
  -c:v libvpx-vp9 -crf 35 -b:v 0 -an \
  "$WEBM_OUT"

ls -lh "$MP4_OUT" "$WEBM_OUT"
TOTAL_BYTES=$(stat -c%s "$MP4_OUT")
TOTAL_BYTES=$((TOTAL_BYTES + $(stat -c%s "$WEBM_OUT")))
TOTAL_MB=$(awk -v b="$TOTAL_BYTES" 'BEGIN { printf "%.2f", b/1024/1024 }')
echo "Combined size: ${TOTAL_MB} MB (target < 5 MB)"
