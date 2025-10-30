#!/usr/bin/env bash
set -euo pipefail

CONFIG_DIR="/etc/dconf/db/gdm.d"
CONFIG_FILE="${CONFIG_DIR}/00-disable-suspend"
TMP_FILE="$(mktemp)"

cleanup() {
  rm -f "${TMP_FILE}"
}
trap cleanup EXIT

cat <<'EOF' >"${TMP_FILE}"
[org/gnome/settings-daemon/plugins/power]
sleep-inactive-ac-type='nothing'
sleep-inactive-ac-timeout=uint32 0
sleep-inactive-battery-type='nothing'
sleep-inactive-battery-timeout=uint32 0
EOF

sudo install -d -m 755 "${CONFIG_DIR}"

if ! sudo cmp -s "${TMP_FILE}" "${CONFIG_FILE}" 2>/dev/null; then
  sudo install -m 644 "${TMP_FILE}" "${CONFIG_FILE}"
  UPDATED=1
else
  UPDATED=0
fi

sudo dconf update

if [[ ${UPDATED} -eq 1 ]]; then
  printf '%s\n' "Updated ${CONFIG_FILE} to disable GDM idle suspend."
else
  printf '%s\n' "GDM idle suspend configuration already up to date."
fi

printf '%s\n' "Restart gdm (sudo systemctl restart gdm) or reboot to apply to the greeter."


