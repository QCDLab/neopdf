# App icons

Icons are generated from `icon.svg`. To regenerate after editing:

```bash
cd neopdf_gui && cargo tauri icon icons/icon.svg -o icons
```

**For the app icon to show:**

1. **Build the bundle** (not just `cargo build`):

   ```bash
   cd neopdf_gui && cargo tauri build
   ```

2. **Run the built app** from `target/release/bundle/macos/NeoPDF.app`
   (or the `.app` / installer for your OS). The icon is embedded in
   the bundle.

3. **macOS:** If the icon still looks old after rebuilding, the system
   may have cached it. Clear the icon cache and restart Dock:

   ```bash
   sudo rm -rf /Library/Caches/com.apple.iconservices.store
   killall Dock
   ```

   Or log out and back in.
