# FNAF 1 — Rust Edition (`fnaf1_rust`)

A clean-room, modular **Rust** reimplementation of the classic Five Nights at Freddy's 1 engine built with **Macroquad**.

> **100% Playable Out-of-the-Box (Zero External Assets Required):**  
> This repository contains **only original Rust source code** under the MIT License. Built-in **procedural clean-room visuals & synthesized WAV audio** work immediately with just `cargo run --release` — no external images or audio files needed!  
> *(Optional: If you own original FNAF 1 files and place them in `assets/images/` and `assets/sounds/`, the engine automatically upgrades to 1:1 original sprites & audio.)*

---

## Features
- **Modular Rust Architecture:** Clean separation between `models`, `ai`, `office`, `assets`, and `main`.
- **Full Game Loop:** Warning screen, Main Menu, Help Wanted Newspaper, Night 1–7 (including Custom Night), Office panning, Door/Light controls, Desk Fan animation, 11-camera Monitor & Map, Power Outage sequence, Jumpscares, and 6 AM / Paycheck screens.
- **Built-in Options Menu (`OPTIONS`):**
  - **FPS Limit:** `30 / 60 / 120 / 144 / Unlimited`
  - **FPS Displayer:** Live top-right FPS counter toggle
  - **Cheats:** `Unlimited Charge` toggle
  - Persistent settings saved to `fnaf1_settings.cfg`.

## Controls
- **Mouse:** Look left/right in the office, click door/light panels, flip camera monitor bar, click camera map buttons, honk Freddy's nose.
- **`Q` / `E`:** Left / Right Door
- **`A` / `D`:** Left / Right Light
- **`SPACE`:** Flip Camera Monitor
- **`1`–`9`, `0`, `-`:** Switch Cameras (`CAM 1A`–`CAM 7`)

## Build & Run
```bash
# Place your assets in ./assets/images/ and ./assets/sounds/
cargo test --lib
cargo run --release
```
