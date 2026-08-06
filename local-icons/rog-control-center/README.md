# rog-control-center custom icon (personal, not for upstream)

Personal WhiteSur-style squircle icon for rog-control-center. The package
ships a flat stock icon; these files restore the custom one after pacman
overwrites `/usr/share/icons/hicolor/*/apps/rog-control-center.png`.

- `original-chatgpt-1024.png` — source image (ChatGPT, 2026-08-01).
- `squircle-master-1024.png` — 1024 master, dark body fills the canvas,
  rx=238 (23.2%) rounded corners, transparent outside.

## Reinstall after an upgrade
```
cd local-icons/rog-control-center
for s in 16 22 24 32 36 48 64 72 96 128 192 256 384 512; do
  magick squircle-master-1024.png -resize ${s}x${s} -strip /tmp/r-$s.png
  sudo cp /tmp/r-$s.png /usr/share/icons/hicolor/${s}x${s}/apps/rog-control-center.png
done
sudo gtk-update-icon-cache -f /usr/share/icons/hicolor/
```
Log out of GNOME (Wayland) for the dock to pick up the new icon.
