use fnaf1_rust::ai::{
    jumpscare_frames, AiEvent, AnimatronicController, AnimatronicLocation, NightClock,
};
use fnaf1_rust::assets::{AssetManager, DigitSet};
use fnaf1_rust::models::{AnimatronicId, CameraId, GameSettings, GameState, NightConfig};
use fnaf1_rust::office::OfficeState;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

const VIRTUAL_W: f32 = 1280.0;
const VIRTUAL_H: f32 = 720.0;

const STATIC_FRAMES: [&str; 8] = [
    "12.png", "13.png", "14.png", "15.png", "16.png", "17.png", "18.png", "20.png",
];

fn window_conf() -> Conf {
    Conf {
        window_title: "Five Nights at Freddy's".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

fn mouse_virtual() -> (f32, f32) {
    let (mx, my) = mouse_position();
    let vx = mx * (VIRTUAL_W / screen_width().max(1.0));
    let vy = my * (VIRTUAL_H / screen_height().max(1.0));
    (vx, vy)
}

fn in_rect(px: f32, py: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
    px >= x && px <= x + w && py >= y && py <= y + h
}

async fn draw_sprite(
    assets: &mut AssetManager,
    name: &str,
    vx: f32,
    vy: f32,
    vw: Option<f32>,
    vh: Option<f32>,
    tint: Color,
) -> bool {
    if let Some(tex) = assets.get_texture(name).await {
        let sx = screen_width() / VIRTUAL_W;
        let sy = screen_height() / VIRTUAL_H;
        let w = vw.unwrap_or_else(|| tex.width()) * sx;
        let h = vh.unwrap_or_else(|| tex.height()) * sy;
        draw_texture_ex(
            &tex,
            vx * sx,
            vy * sy,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );
        true
    } else {
        false
    }
}

fn vrect(vx: f32, vy: f32, vw: f32, vh: f32, color: Color) {
    let sx = screen_width() / VIRTUAL_W;
    let sy = screen_height() / VIRTUAL_H;
    draw_rectangle(vx * sx, vy * sy, vw * sx, vh * sy, color);
}

fn vrect_lines(vx: f32, vy: f32, vw: f32, vh: f32, thick: f32, color: Color) {
    let sx = screen_width() / VIRTUAL_W;
    let sy = screen_height() / VIRTUAL_H;
    draw_rectangle_lines(vx * sx, vy * sy, vw * sx, vh * sy, thick * sy, color);
}

fn vtext(text: impl AsRef<str>, vx: f32, vy: f32, size: f32, color: Color) {
    let sx = screen_width() / VIRTUAL_W;
    let sy = screen_height() / VIRTUAL_H;
    draw_text(text.as_ref(), vx * sx, vy * sy, size * sy, color);
}

fn draw_procedural_static(global_time: f32, alpha: f32) {
    let seed = (global_time * 60.0) as u32;
    for i in 0..90 {
        let h = seed.wrapping_mul(2654435761).wrapping_add(i * 1597334677);
        let y = (h % 720) as f32;
        let x = ((h >> 10) % 1280) as f32;
        let w = 40.0 + ((h >> 18) % 180) as f32;
        let bright = 0.4 + (((h >> 24) % 60) as f32) / 100.0;
        vrect(x, y, w, 2.5, Color::new(bright, bright, bright, alpha));
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameState::Warning;
    let mut current_night: u8 = 1;
    let mut config = NightConfig::for_night(current_night);
    let mut clock = NightClock::new(config.hour_duration_secs);
    let mut office = OfficeState::new();
    let mut ai = AnimatronicController::new();
    let mut assets = AssetManager::new(AssetManager::resolve_assets_dir());

    let mut state_timer: f32 = 0.0;
    let mut global_time: f32 = 0.0;
    let mut menu_hover: usize = 0;
    let mut power_outage_active = false;
    let mut cam_bar_hovered_prev = false;
    let mut settings = GameSettings::load();
    let mut settings_hover: usize = 0;

    // Custom Night AI levels
    let mut custom_ai: [u8; 4] = [1, 3, 3, 1]; // Freddy, Bonnie, Chica, Foxy

    // Preload critical UI & menu textures
    assets
        .preload_textures(&[
            "605.png", "431.png", "440.png", "441.png", "442.png", "444.png", "448.png", "449.png",
            "450.png", "443.png", "524.png", "433.png", "588.png", "574.png", "39.png", "58.png",
            "127.png", "225.png", "227.png", "122.png", "124.png", "125.png", "130.png", "131.png",
            "134.png", "135.png", "47.png", "420.png", "145.png", "166.png", "167.png", "207.png",
            "208.png", "209.png", "212.png", "213.png", "214.png", "455.png", "251.png", "475.png",
            "481.png",
        ])
        .await;

    loop {
        let frame_start = Instant::now();
        let dt = get_frame_time().min(0.1);
        global_time += dt;
        clear_background(BLACK);

        let (mx, my) = mouse_virtual();
        let clicked = is_mouse_button_pressed(MouseButton::Left);

        match state {
            GameState::Warning => {
                state_timer += dt;
                let alpha = if state_timer < 0.8 {
                    state_timer / 0.8
                } else if state_timer > 2.7 {
                    ((3.5 - state_timer) / 0.8).max(0.0)
                } else {
                    1.0
                };
                if !draw_sprite(
                    &mut assets,
                    "605.png",
                    (VIRTUAL_W - 465.0) * 0.5,
                    (VIRTUAL_H - 124.0) * 0.5,
                    None,
                    None,
                    Color::new(1.0, 1.0, 1.0, alpha),
                )
                .await
                {
                    let c = Color::new(1.0, 1.0, 1.0, alpha);
                    vtext("WARNING!", 545.0, 310.0, 44.0, c);
                    vtext(
                        "This game contains flashing lights, loud noises, and lots of jumpscares!",
                        230.0,
                        370.0,
                        24.0,
                        Color::new(0.85, 0.85, 0.85, alpha),
                    );
                }

                if state_timer >= 3.5 || clicked || is_key_pressed(KeyCode::Space) {
                    state_timer = 0.0;
                    state = GameState::MainMenu;
                    assets.play_sfx("darkness.wav", true, 0.55).await;
                    assets.play_sfx("stat.wav", false, 0.45).await;
                }
            }
            GameState::MainMenu => {
                // Twitching Freddy background
                let glitch_roll = ((global_time * 17.0) as u32) % 97;
                let bg = match glitch_roll {
                    0..=1 => "440.png",
                    2 => "441.png",
                    3 => "442.png",
                    _ => "431.png",
                };
                if !draw_sprite(&mut assets, bg, 0.0, 0.0, Some(VIRTUAL_W), Some(VIRTUAL_H), WHITE)
                    .await
                {
                    // Clean-room animatronic silhouette on right
                    vrect(0.0, 0.0, VIRTUAL_W, VIRTUAL_H, Color::new(0.03, 0.03, 0.05, 1.0));
                    vrect(740.0, 140.0, 340.0, 440.0, Color::new(0.14, 0.09, 0.06, 1.0));
                    vrect(780.0, 80.0, 80.0, 70.0, Color::new(0.12, 0.08, 0.05, 1.0));
                    vrect(960.0, 80.0, 80.0, 70.0, Color::new(0.12, 0.08, 0.05, 1.0));
                    let eye_col = if glitch_roll <= 3 { RED } else { WHITE };
                    vrect(820.0, 250.0, 45.0, 45.0, BLACK);
                    vrect(945.0, 250.0, 45.0, 45.0, BLACK);
                    vrect(836.0, 266.0, 14.0, 14.0, eye_col);
                    vrect(961.0, 266.0, 14.0, 14.0, eye_col);
                }

                // Static overlay
                let st_idx = ((global_time * 30.0) as usize) % STATIC_FRAMES.len();
                if !draw_sprite(
                    &mut assets,
                    STATIC_FRAMES[st_idx],
                    0.0,
                    0.0,
                    Some(VIRTUAL_W),
                    Some(VIRTUAL_H),
                    Color::new(1.0, 1.0, 1.0, 0.30),
                )
                .await
                {
                    draw_procedural_static(global_time, 0.14);
                }

                // Scanline bar
                let scan_y = (global_time * 90.0) % VIRTUAL_H;
                if !draw_sprite(
                    &mut assets,
                    "4.png",
                    0.0,
                    scan_y,
                    Some(VIRTUAL_W),
                    Some(40.0),
                    Color::new(1.0, 1.0, 1.0, 0.08),
                )
                .await
                {
                    vrect(0.0, scan_y, VIRTUAL_W, 32.0, Color::new(1.0, 1.0, 1.0, 0.05));
                }

                // Title Logo & Menu Options
                if !draw_sprite(&mut assets, "444.png", 174.0, 70.0, None, None, WHITE).await {
                    vtext("FIVE", 174.0, 120.0, 56.0, WHITE);
                    vtext("NIGHTS", 174.0, 175.0, 56.0, WHITE);
                    vtext("AT FREDDY'S", 174.0, 230.0, 56.0, WHITE);
                    vtext("RUST OPEN-SOURCE EDITION", 174.0, 268.0, 22.0, GRAY);
                }
                if !draw_sprite(&mut assets, "448.png", 174.0, 404.0, None, None, WHITE).await {
                    vtext("New Game", 174.0, 434.0, 36.0, WHITE);
                }
                if !draw_sprite(&mut assets, "449.png", 174.0, 474.0, None, None, WHITE).await {
                    vtext("Continue", 174.0, 504.0, 36.0, WHITE);
                }
                if !draw_sprite(&mut assets, "447.png", 174.0, 514.0, None, None, WHITE).await {
                    vtext("Night", 174.0, 534.0, 22.0, LIGHTGRAY);
                }
                assets
                    .draw_digits(u32::from(current_night), 244.0, 512.0, 1.0, DigitSet::HudSmall, false)
                    .await;
                if !draw_sprite(&mut assets, "443.png", 174.0, 554.0, None, None, WHITE).await {
                    vtext("6th Night", 174.0, 586.0, 36.0, WHITE);
                }
                if !draw_sprite(&mut assets, "524.png", 174.0, 616.0, None, None, WHITE).await {
                    vtext("Custom Night", 174.0, 648.0, 36.0, WHITE);
                }
                if !draw_sprite(&mut assets, "433.png", 1030.0, 690.0, None, None, WHITE).await {
                    vtext("Rust Open-Source Port", 1030.0, 704.0, 18.0, GRAY);
                }
                if !draw_sprite(&mut assets, "588.png", 30.0, 690.0, None, None, WHITE).await {
                    vtext("v 1.132-rs", 30.0, 704.0, 18.0, GRAY);
                }

                // Bottom-right Options button (FNAF style)
                let opt_hovered = in_rect(mx, my, 1020.0, 622.0, 230.0, 48.0);
                if opt_hovered {
                    menu_hover = 4;
                }
                let sx = screen_width() / VIRTUAL_W;
                let sy = screen_height() / VIRTUAL_H;
                let opt_color = if menu_hover == 4 { YELLOW } else { WHITE };
                draw_text(
                    if menu_hover == 4 { ">> OPTIONS" } else { "OPTIONS" },
                    1032.0 * sx,
                    658.0 * sy,
                    32.0 * sy,
                    opt_color,
                );

                // Hover detection
                if in_rect(mx, my, 170.0, 400.0, 230.0, 40.0) {
                    menu_hover = 0;
                } else if in_rect(mx, my, 170.0, 470.0, 230.0, 45.0) {
                    menu_hover = 1;
                } else if in_rect(mx, my, 170.0, 550.0, 240.0, 45.0) {
                    menu_hover = 2;
                } else if in_rect(mx, my, 170.0, 612.0, 320.0, 45.0) {
                    menu_hover = 3;
                }

                if is_key_pressed(KeyCode::Down) {
                    menu_hover = (menu_hover + 1) % 5;
                    assets.play_sfx("blip.wav", false, 0.5).await;
                }
                if is_key_pressed(KeyCode::Up) {
                    menu_hover = (menu_hover + 4) % 5;
                    assets.play_sfx("blip.wav", false, 0.5).await;
                }

                if menu_hover < 4 {
                    let cursor_y = match menu_hover {
                        0 => 408.0,
                        1 => 478.0,
                        2 => 560.0,
                        _ => 622.0,
                    };
                    if !draw_sprite(&mut assets, "450.png", 118.0, cursor_y, None, None, WHITE).await
                    {
                        vtext(">>", 118.0, cursor_y + 28.0, 34.0, YELLOW);
                    }
                }

                let confirm = (clicked
                    && (in_rect(mx, my, 165.0, 395.0, 340.0, 270.0) || opt_hovered))
                    || is_key_pressed(KeyCode::Enter)
                    || is_key_pressed(KeyCode::Space);

                if confirm {
                    assets.play_sfx("blip.wav", false, 0.7).await;
                    if menu_hover == 4 {
                        state_timer = 0.0;
                        state = GameState::Settings;
                    } else {
                        assets.stop_all_sfx();
                        state_timer = 0.0;
                        match menu_hover {
                            0 => {
                                current_night = 1;
                                state = GameState::Newspaper;
                            }
                            1 => {
                                state = GameState::NightIntro {
                                    night: current_night,
                                };
                            }
                            2 => {
                                current_night = 6;
                                state = GameState::NightIntro { night: 6 };
                            }
                            _ => {
                                state = GameState::CustomNight;
                            }
                        }
                    }
                }
            }
            GameState::Settings => {
                // Darkened Freddy background + static + scanline + CRT frame
                let glitch_roll = ((global_time * 17.0) as u32) % 97;
                let bg = match glitch_roll {
                    0..=1 => "440.png",
                    2 => "441.png",
                    3 => "442.png",
                    _ => "431.png",
                };
                if !draw_sprite(
                    &mut assets,
                    bg,
                    0.0,
                    0.0,
                    Some(VIRTUAL_W),
                    Some(VIRTUAL_H),
                    Color::new(0.45, 0.45, 0.45, 1.0),
                )
                .await
                {
                    vrect(0.0, 0.0, VIRTUAL_W, VIRTUAL_H, Color::new(0.02, 0.02, 0.04, 1.0));
                }

                let st_idx = ((global_time * 30.0) as usize) % STATIC_FRAMES.len();
                if !draw_sprite(
                    &mut assets,
                    STATIC_FRAMES[st_idx],
                    0.0,
                    0.0,
                    Some(VIRTUAL_W),
                    Some(VIRTUAL_H),
                    Color::new(1.0, 1.0, 1.0, 0.25),
                )
                .await
                {
                    draw_procedural_static(global_time, 0.12);
                }

                let scan_y = (global_time * 90.0) % VIRTUAL_H;
                draw_sprite(
                    &mut assets,
                    "4.png",
                    0.0,
                    scan_y,
                    Some(VIRTUAL_W),
                    Some(40.0),
                    Color::new(1.0, 1.0, 1.0, 0.08),
                )
                .await;

                if !draw_sprite(
                    &mut assets,
                    "11.png",
                    0.0,
                    0.0,
                    Some(VIRTUAL_W),
                    Some(VIRTUAL_H),
                    WHITE,
                )
                .await
                {
                    vrect_lines(18.0, 18.0, VIRTUAL_W - 36.0, VIRTUAL_H - 36.0, 2.0, DARKGRAY);
                }

                let sx = screen_width() / VIRTUAL_W;
                let sy = screen_height() / VIRTUAL_H;

                draw_text("SETTINGS", 140.0 * sx, 110.0 * sy, 52.0 * sy, WHITE);
                draw_line(140.0 * sx, 130.0 * sy, 680.0 * sx, 130.0 * sy, 2.0 * sy, DARKGRAY);

                // Hover zones
                if in_rect(mx, my, 130.0, 210.0, 620.0, 52.0) {
                    settings_hover = 0;
                } else if in_rect(mx, my, 130.0, 280.0, 620.0, 52.0) {
                    settings_hover = 1;
                } else if in_rect(mx, my, 130.0, 420.0, 620.0, 52.0) {
                    settings_hover = 2;
                } else if in_rect(mx, my, 130.0, 580.0, 260.0, 55.0) {
                    settings_hover = 3;
                }

                if is_key_pressed(KeyCode::Down) {
                    settings_hover = (settings_hover + 1) % 4;
                    assets.play_sfx("blip.wav", false, 0.5).await;
                }
                if is_key_pressed(KeyCode::Up) {
                    settings_hover = (settings_hover + 3) % 4;
                    assets.play_sfx("blip.wav", false, 0.5).await;
                }

                // Section: DISPLAY
                draw_text("DISPLAY & PERFORMANCE", 140.0 * sx, 185.0 * sy, 24.0 * sy, GRAY);

                let row0_col = if settings_hover == 0 { YELLOW } else { WHITE };
                draw_text("FPS Limit:", 180.0 * sx, 250.0 * sy, 32.0 * sy, row0_col);
                draw_text(
                    format!("[ {} ]", settings.fps_limit.label()),
                    500.0 * sx,
                    250.0 * sy,
                    32.0 * sy,
                    row0_col,
                );

                let row1_col = if settings_hover == 1 { YELLOW } else { WHITE };
                draw_text("FPS Displayer (Top-Right):", 180.0 * sx, 320.0 * sy, 32.0 * sy, row1_col);
                draw_text(
                    if settings.show_fps { "[ ON ]" } else { "[ OFF ]" },
                    590.0 * sx,
                    320.0 * sy,
                    32.0 * sy,
                    if settings.show_fps { GREEN } else { RED },
                );

                // Section: CHEATS
                draw_text("CHEATS", 140.0 * sx, 395.0 * sy, 24.0 * sy, GRAY);

                let row2_col = if settings_hover == 2 { YELLOW } else { WHITE };
                draw_text("Unlimited Charge:", 180.0 * sx, 460.0 * sy, 32.0 * sy, row2_col);
                draw_text(
                    if settings.unlimited_charge { "[ ON ]" } else { "[ OFF ]" },
                    500.0 * sx,
                    460.0 * sy,
                    32.0 * sy,
                    if settings.unlimited_charge { GREEN } else { RED },
                );

                // Back button
                let back_col = if settings_hover == 3 { YELLOW } else { LIGHTGRAY };
                draw_text("<< BACK", 140.0 * sx, 620.0 * sy, 36.0 * sy, back_col);

                // Cursor >>
                if settings_hover < 3 {
                    let cy = match settings_hover {
                        0 => 226.0,
                        1 => 296.0,
                        _ => 436.0,
                    };
                    if !draw_sprite(&mut assets, "450.png", 122.0, cy, None, None, WHITE).await {
                        vtext(">>", 122.0, cy + 26.0, 32.0, YELLOW);
                    }
                }

                let activate = (clicked && in_rect(mx, my, 120.0, 200.0, 680.0, 450.0))
                    || is_key_pressed(KeyCode::Enter)
                    || is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Right)
                    || is_key_pressed(KeyCode::Left);

                if activate {
                    assets.play_sfx("blip.wav", false, 0.6).await;
                    match settings_hover {
                        0 => settings.fps_limit = settings.fps_limit.next(),
                        1 => settings.show_fps = !settings.show_fps,
                        2 => settings.unlimited_charge = !settings.unlimited_charge,
                        _ => {
                            settings.save();
                            state = GameState::MainMenu;
                        }
                    }
                    settings.save();
                }

                if is_key_pressed(KeyCode::Escape) {
                    settings.save();
                    assets.play_sfx("blip.wav", false, 0.6).await;
                    state = GameState::MainMenu;
                }
            }
            GameState::Newspaper => {
                state_timer += dt;
                let alpha = if state_timer < 1.5 {
                    state_timer / 1.5
                } else if state_timer > 5.0 {
                    ((6.0 - state_timer) / 1.0).max(0.0)
                } else {
                    1.0
                };
                if !draw_sprite(
                    &mut assets,
                    "574.png",
                    0.0,
                    0.0,
                    Some(VIRTUAL_W),
                    Some(VIRTUAL_H),
                    Color::new(1.0, 1.0, 1.0, alpha),
                )
                .await
                {
                    let c = Color::new(0.85, 0.82, 0.75, alpha);
                    vrect(220.0, 70.0, 840.0, 580.0, Color::new(0.18, 0.17, 0.15, alpha));
                    vrect_lines(220.0, 70.0, 840.0, 580.0, 3.0, c);
                    vtext("HELP WANTED!", 440.0, 145.0, 52.0, c);
                    vtext("Freddy Fazbear's Pizza", 435.0, 200.0, 34.0, c);
                    vtext(
                        "Family pizzeria looking for security guard to work the nightshift.",
                        270.0,
                        310.0,
                        24.0,
                        c,
                    );
                    vtext("12 am to 6 am. $120 a week.", 450.0, 360.0, 24.0, c);
                }
                if state_timer >= 6.0 || clicked || is_key_pressed(KeyCode::Space) {
                    state_timer = 0.0;
                    state = GameState::NightIntro { night: 1 };
                    assets.play_sfx("blip.wav", false, 0.7).await;
                }
            }
            GameState::CustomNight => {
                if !draw_sprite(&mut assets, "526.png", 480.0, 36.0, None, None, WHITE).await {
                    vtext("CUSTOM NIGHT", 495.0, 78.0, 44.0, WHITE);
                }
                let portraits = [
                    ("527.png", "531.png", "Freddy", Color::new(0.55, 0.35, 0.15, 1.0)),
                    ("528.png", "535.png", "Bonnie", Color::new(0.45, 0.35, 0.75, 1.0)),
                    ("529.png", "533.png", "Chica", Color::new(0.85, 0.80, 0.20, 1.0)),
                    ("536.png", "534.png", "Foxy", Color::new(0.75, 0.25, 0.20, 1.0)),
                ];
                for (i, (img, lbl, name, col)) in portraits.iter().enumerate() {
                    let bx = 130.0 + (i as f32) * 265.0;
                    if !draw_sprite(&mut assets, img, bx, 130.0, None, None, WHITE).await {
                        vrect(bx, 130.0, 200.0, 200.0, *col);
                        vrect_lines(bx, 130.0, 200.0, 200.0, 3.0, WHITE);
                    }
                    if !draw_sprite(&mut assets, lbl, bx + 25.0, 345.0, None, None, WHITE).await {
                        vtext(name, bx + 45.0, 375.0, 32.0, WHITE);
                    }
                    if !draw_sprite(&mut assets, "537.png", bx, 405.0, None, None, WHITE).await {
                        vtext("A.I. Level", bx + 40.0, 428.0, 22.0, LIGHTGRAY);
                    }
                    if !draw_sprite(&mut assets, "542.png", bx + 5.0, 460.0, None, None, WHITE).await
                    {
                        vtext("<", bx + 10.0, 495.0, 40.0, WHITE);
                    }
                    if !draw_sprite(&mut assets, "541.png", bx + 160.0, 460.0, None, None, WHITE)
                        .await
                    {
                        vtext(">", bx + 165.0, 495.0, 40.0, WHITE);
                    }
                    assets
                        .draw_digits(
                            u32::from(custom_ai[i]),
                            bx + 80.0,
                            445.0,
                            0.85,
                            DigitSet::CustomAi,
                            false,
                        )
                        .await;
                    if clicked {
                        if in_rect(mx, my, bx, 450.0, 50.0, 65.0) && custom_ai[i] > 0 {
                            custom_ai[i] -= 1;
                            assets.play_sfx("blip.wav", false, 0.5).await;
                        } else if in_rect(mx, my, bx + 150.0, 450.0, 50.0, 65.0)
                            && custom_ai[i] < 20
                        {
                            custom_ai[i] += 1;
                            assets.play_sfx("blip.wav", false, 0.5).await;
                        }
                    }
                }
                if !draw_sprite(&mut assets, "530.png", 1040.0, 620.0, None, None, WHITE).await {
                    vrect(1035.0, 615.0, 195.0, 65.0, DARKGRAY);
                    vrect_lines(1035.0, 615.0, 195.0, 65.0, 2.0, WHITE);
                    vtext("READY", 1068.0, 660.0, 38.0, WHITE);
                }
                if (clicked && in_rect(mx, my, 1030.0, 610.0, 210.0, 75.0))
                    || is_key_pressed(KeyCode::Enter)
                {
                    current_night = 7;
                    config = NightConfig::custom(
                        custom_ai[0],
                        custom_ai[1],
                        custom_ai[2],
                        custom_ai[3],
                    );
                    state_timer = 0.0;
                    state = GameState::NightIntro { night: 7 };
                    assets.play_sfx("blip.wav", false, 0.7).await;
                }
                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::MainMenu;
                }
            }
            GameState::NightIntro { night } => {
                state_timer += dt;
                let intro_sprite = match night {
                    1 => "453.png",
                    2 => "454.png",
                    3 => "472.png",
                    4 => "473.png",
                    5 => "474.png",
                    6 => "446.png",
                    _ => "538.png",
                };
                if !draw_sprite(
                    &mut assets,
                    intro_sprite,
                    (VIRTUAL_W - 231.0) * 0.5,
                    (VIRTUAL_H - 97.0) * 0.5,
                    None,
                    None,
                    WHITE,
                )
                .await
                {
                    vtext("12:00 AM", 540.0, 330.0, 48.0, WHITE);
                    vtext(format!("Night {night}"), 565.0, 390.0, 38.0, LIGHTGRAY);
                }
                if state_timer >= 2.5 {
                    state_timer = 0.0;
                    if night < 7 {
                        config = NightConfig::for_night(night);
                    }
                    clock = NightClock::new(config.hour_duration_secs);
                    office = OfficeState::new();
                    ai = AnimatronicController::new();
                    power_outage_active = false;
                    state = GameState::Playing;
                    assets.play_sfx("Buzz_Fan_Flo.wav", true, 0.30).await;
                    assets.play_sfx("ambien.wav", true, 0.25).await;
                    if night == 1 {
                        assets.play_sfx("voiceo.wav", false, 0.70).await;
                    }
                }
            }
            GameState::Playing => {
                office.tick_animations(dt);

                // Keyboard shortcuts
                if is_key_pressed(KeyCode::Q) {
                    office.toggle_left_door();
                    assets.play_sfx("SFXBible.wav", false, 0.65).await;
                }
                if is_key_pressed(KeyCode::E) {
                    office.toggle_right_door();
                    assets.play_sfx("SFXBible.wav", false, 0.65).await;
                }
                if is_key_pressed(KeyCode::A) {
                    office.toggle_left_light();
                    assets.play_sfx("BallastHum.wav", false, 0.4).await;
                }
                if is_key_pressed(KeyCode::D) {
                    office.toggle_right_light();
                    assets.play_sfx("BallastHum.wav", false, 0.4).await;
                }
                if is_key_pressed(KeyCode::Space) {
                    office.toggle_cameras();
                    assets.play_sfx("CAMERA_VIDEO_L.wav", false, 0.6).await;
                }

                // Bottom camera flip bar hover/click
                let bar_hovered = in_rect(mx, my, 340.0, 650.0, 600.0, 70.0);
                if (bar_hovered && !cam_bar_hovered_prev)
                    || (clicked && in_rect(mx, my, 340.0, 650.0, 600.0, 70.0))
                {
                    office.toggle_cameras();
                    assets.play_sfx("CAMERA_VIDEO_L.wav", false, 0.6).await;
                }
                cam_bar_hovered_prev = bar_hovered;

                // Mute Call button
                if !office.call_muted
                    && current_night == 1
                    && clock.elapsed_secs < 90.0
                    && clicked
                    && in_rect(mx, my, 26.0, 22.0, 121.0, 31.0)
                {
                    office.call_muted = true;
                    assets.stop_sfx("voiceo.wav");
                }

                // Mouse panning & office button clicks
                if !office.cameras_up {
                    let norm_x = (mx / VIRTUAL_W) * 2.0 - 1.0;
                    if norm_x.abs() > 0.25 {
                        office.pan_offset =
                            (office.pan_offset + norm_x * dt * 1.6).clamp(0.0, 1.0);
                    }
                    let pan_px = office.pan_offset * 320.0;
                    if clicked {
                        // Left panel (world x=6..98, y=263..510)
                        let lx = 6.0 - pan_px;
                        if in_rect(mx, my, lx, 263.0, 92.0, 120.0) {
                            office.toggle_left_door();
                            assets.play_sfx("SFXBible.wav", false, 0.65).await;
                        } else if in_rect(mx, my, lx, 383.0, 92.0, 127.0) {
                            office.toggle_left_light();
                            assets.play_sfx("BallastHum.wav", false, 0.4).await;
                        }

                        // Right panel (world x=1498..1590, y=263..510)
                        let rx = 1498.0 - pan_px;
                        if in_rect(mx, my, rx, 263.0, 92.0, 120.0) {
                            office.toggle_right_door();
                            assets.play_sfx("SFXBible.wav", false, 0.65).await;
                        } else if in_rect(mx, my, rx, 383.0, 92.0, 127.0) {
                            office.toggle_right_light();
                            assets.play_sfx("BallastHum.wav", false, 0.4).await;
                        }

                        // Freddy nose honk easter egg (world x=674, y=236)
                        let nx = 674.0 - pan_px;
                        if in_rect(mx, my, nx - 16.0, 220.0, 32.0, 32.0) {
                            assets.play_sfx("PartyFavorrasp.wav", false, 0.85).await;
                        }
                    }
                } else if office.is_camera_feed_visible() && clicked {
                    // Clickable camera buttons on map (848, 313)
                    for cam in CameraId::ALL {
                        let (ox, oy) = cam.map_offset();
                        if in_rect(mx, my, 848.0 + ox, 313.0 + oy, 60.0, 40.0) {
                            office.select_camera(cam);
                            assets.play_sfx("blip.wav", false, 0.5).await;
                            break;
                        }
                    }
                }

                // Simulation
                clock.tick(dt);
                if clock.is_6am() {
                    assets.stop_all_sfx();
                    state_timer = 0.0;
                    state = GameState::NightComplete {
                        night: current_night,
                    };
                    assets.play_sfx("chimes.wav", false, 0.85).await;
                    continue;
                }

                if office.drain_power(dt, settings.unlimited_charge) {
                    assets.stop_all_sfx();
                    state_timer = 0.0;
                    power_outage_active = true;
                    state = GameState::PowerOutage;
                    assets.play_sfx("powerd.wav", false, 0.85).await;
                    continue;
                }

                match ai.update(
                    dt,
                    &config,
                    office.left_door_closed,
                    office.right_door_closed,
                    office.cameras_up,
                    office.active_camera,
                ) {
                    AiEvent::Footsteps => {
                        assets.play_sfx("deeps.wav", false, 0.45).await;
                    }
                    AiEvent::FreddyLaugh => {
                        assets.play_sfx("Laugh_Giggle.wav", false, 0.55).await;
                    }
                    AiEvent::KitchenSound => {
                        assets.play_sfx("OVEN-DRAWE_G.wav", false, 0.50).await;
                    }
                    AiEvent::FoxyBangOnDoor => {
                        office.apply_foxy_drain(settings.unlimited_charge);
                        assets.play_sfx("DOOR_POUNDING_.wav", false, 0.90).await;
                    }
                    AiEvent::Jumpscare(by) => {
                        assets.stop_all_sfx();
                        state_timer = 0.0;
                        state = GameState::Jumpscare { by };
                        assets.play_sfx("XSCREA.wav", false, 0.95).await;
                        continue;
                    }
                    AiEvent::None => {}
                }

                // Render
                if office.is_camera_feed_visible() {
                    draw_camera_view(&office, &ai, &mut assets, global_time).await;
                } else {
                    draw_office_view(&office, &ai, &mut assets, global_time).await;
                    if office.is_camera_flipping() {
                        let flip_sprite = office.current_cam_flip_sprite();
                        draw_sprite(
                            &mut assets,
                            flip_sprite,
                            0.0,
                            0.0,
                            Some(VIRTUAL_W),
                            Some(VIRTUAL_H),
                            WHITE,
                        )
                        .await;
                    }
                }
                draw_hud(&office, &clock, current_night, &mut assets).await;
            }
            GameState::PowerOutage => {
                state_timer += dt;
                let pan_px = office.pan_offset * 320.0;
                if state_timer > 2.5 && state_timer < 9.0 {
                    if (state_timer - dt) <= 2.5 {
                        assets.play_sfx("music.wav", false, 0.8).await;
                    }
                    let blink = ((state_timer * 14.0) as i32) % 3 != 0;
                    let img = if blink { "305.png" } else { "304.png" };
                    if !draw_sprite(&mut assets, img, -pan_px, 0.0, None, None, WHITE).await
                        && blink
                    {
                        vrect(145.0 - pan_px, 230.0, 14.0, 14.0, WHITE);
                        vrect(185.0 - pan_px, 230.0, 14.0, 14.0, WHITE);
                    }
                } else {
                    draw_sprite(&mut assets, "304.png", -pan_px, 0.0, None, None, WHITE).await;
                }

                if state_timer >= 11.0 {
                    assets.stop_all_sfx();
                    state_timer = 0.0;
                    state = GameState::Jumpscare {
                        by: AnimatronicId::Freddy,
                    };
                    assets.play_sfx("XSCREA.wav", false, 0.95).await;
                }
            }
            GameState::Jumpscare { by } => {
                state_timer += dt;
                let frames = jumpscare_frames(by, power_outage_active);
                let idx = ((state_timer * 24.0) as usize).min(frames.len() - 1);
                if !draw_sprite(
                    &mut assets,
                    frames[idx],
                    0.0,
                    0.0,
                    Some(VIRTUAL_W),
                    Some(VIRTUAL_H),
                    WHITE,
                )
                .await
                {
                    let shake_x = ((state_timer * 60.0).sin()) * 35.0;
                    let shake_y = ((state_timer * 53.0).cos()) * 25.0;
                    let col = match by {
                        AnimatronicId::Freddy => Color::new(0.55, 0.35, 0.15, 1.0),
                        AnimatronicId::Bonnie => Color::new(0.45, 0.35, 0.75, 1.0),
                        AnimatronicId::Chica => Color::new(0.85, 0.80, 0.20, 1.0),
                        AnimatronicId::Foxy => Color::new(0.75, 0.25, 0.20, 1.0),
                    };
                    vrect(0.0, 0.0, VIRTUAL_W, VIRTUAL_H, Color::new(0.15, 0.0, 0.0, 1.0));
                    vrect(360.0 + shake_x, 90.0 + shake_y, 560.0, 560.0, col);
                    vrect(460.0 + shake_x, 220.0 + shake_y, 110.0, 110.0, BLACK);
                    vrect(710.0 + shake_x, 220.0 + shake_y, 110.0, 110.0, BLACK);
                    vrect(500.0 + shake_x, 260.0 + shake_y, 30.0, 30.0, WHITE);
                    vrect(750.0 + shake_x, 260.0 + shake_y, 30.0, 30.0, WHITE);
                    vrect(480.0 + shake_x, 420.0 + shake_y, 320.0, 160.0, BLACK);
                }
                if state_timer >= 1.2 {
                    assets.stop_all_sfx();
                    state_timer = 0.0;
                    state = GameState::GameOver;
                    assets.play_sfx("stat.wav", true, 0.6).await;
                }
            }
            GameState::GameOver => {
                state_timer += dt;
                if state_timer < 4.5 {
                    let st_idx = ((global_time * 30.0) as usize) % STATIC_FRAMES.len();
                    if !draw_sprite(
                        &mut assets,
                        STATIC_FRAMES[st_idx],
                        0.0,
                        0.0,
                        Some(VIRTUAL_W),
                        Some(VIRTUAL_H),
                        WHITE,
                    )
                    .await
                    {
                        draw_procedural_static(global_time, 0.55);
                    }
                } else {
                    if state_timer - dt < 4.5 {
                        assets.stop_sfx("stat.wav");
                    }
                    if !draw_sprite(
                        &mut assets,
                        "358.png",
                        0.0,
                        0.0,
                        Some(VIRTUAL_W),
                        Some(VIRTUAL_H),
                        WHITE,
                    )
                    .await
                    {
                        vrect(0.0, 0.0, VIRTUAL_W, VIRTUAL_H, Color::new(0.05, 0.03, 0.03, 1.0));
                        vtext("YOU WERE STUFFED IN A SUIT", 380.0, 340.0, 38.0, DARKGRAY);
                    }
                    if !draw_sprite(&mut assets, "471.png", 980.0, 640.0, None, None, WHITE).await {
                        vtext("Game Over", 980.0, 670.0, 38.0, RED);
                    }
                    if state_timer >= 9.0 || clicked || is_key_pressed(KeyCode::Space) {
                        assets.stop_all_sfx();
                        state_timer = 0.0;
                        state = GameState::MainMenu;
                        assets.play_sfx("darkness.wav", true, 0.55).await;
                    }
                }
            }
            GameState::NightComplete { night } => {
                state_timer += dt;
                let scroll = (state_timer / 3.0).min(1.0);
                let y_5 = 310.0 - scroll * 110.0;
                let y_6 = 420.0 - scroll * 110.0;
                if !draw_sprite(&mut assets, "350.png", 520.0, y_5, None, None, WHITE).await {
                    vtext("5", 525.0, y_5 + 58.0, 76.0, WHITE);
                }
                if !draw_sprite(&mut assets, "351.png", 520.0, y_6, None, None, WHITE).await {
                    vtext("6", 525.0, y_6 + 58.0, 76.0, WHITE);
                }
                let sy = screen_height() / VIRTUAL_H;
                draw_rectangle(0.0, 0.0, screen_width(), 300.0 * sy, BLACK);
                draw_rectangle(0.0, 395.0 * sy, screen_width(), 325.0 * sy, BLACK);
                if !draw_sprite(&mut assets, "352.png", 600.0, 310.0, None, None, WHITE).await {
                    vtext("AM", 600.0, 368.0, 76.0, WHITE);
                }

                if (state_timer - dt) < 3.0 && state_timer >= 3.0 {
                    assets.play_sfx("CROWD_SMALL_CH.wav", false, 0.75).await;
                }

                if state_timer >= 7.5 || is_key_pressed(KeyCode::Space) {
                    assets.stop_all_sfx();
                    state_timer = 0.0;
                    if night >= 5 {
                        state = GameState::Paycheck { night };
                    } else {
                        current_night = (night + 1).min(5);
                        state = GameState::MainMenu;
                        assets.play_sfx("darkness.wav", true, 0.55).await;
                    }
                }
            }
            GameState::Paycheck { night } => {
                state_timer += dt;
                let img = match night {
                    5 => "210.png",
                    6 => "522.png",
                    _ => "523.png",
                };
                if !draw_sprite(
                    &mut assets,
                    img,
                    0.0,
                    0.0,
                    Some(VIRTUAL_W),
                    Some(VIRTUAL_H),
                    WHITE,
                )
                .await
                {
                    vrect(220.0, 120.0, 840.0, 480.0, Color::new(0.85, 0.80, 0.70, 1.0));
                    vtext("PAYCHECK - GOOD JOB!", 420.0, 300.0, 44.0, BLACK);
                    vtext(format!("Completed Night {night}"), 480.0, 370.0, 32.0, DARKGRAY);
                }
                if state_timer >= 8.0 || clicked || is_key_pressed(KeyCode::Space) {
                    assets.stop_all_sfx();
                    state_timer = 0.0;
                    state = GameState::MainMenu;
                    assets.play_sfx("darkness.wav", true, 0.55).await;
                }
            }
        }

        if settings.show_fps {
            let sx = screen_width() / VIRTUAL_W;
            let sy = screen_height() / VIRTUAL_H;
            let fps_text = format!("FPS: {}", get_fps());
            draw_text(
                fps_text,
                1140.0 * sx,
                22.0 * sy,
                20.0 * sy,
                Color::new(0.4, 1.0, 0.4, 0.9),
            );
        }

        next_frame().await;

        if let Some(target_fps) = settings.fps_limit.as_u32() {
            let target_dur = Duration::from_secs_f64(1.0 / f64::from(target_fps));
            let elapsed = frame_start.elapsed();
            if elapsed < target_dur {
                std::thread::sleep(target_dur - elapsed);
            }
        }
    }
}

async fn draw_office_view(
    office: &OfficeState,
    ai: &AnimatronicController,
    assets: &mut AssetManager,
    global_time: f32,
) {
    let pan_px = office.pan_offset * 320.0;
    let flicker_on = !((global_time * 37.0) as u32).is_multiple_of(7);

    let office_sprite = if office.left_light_on && flicker_on {
        if ai.bonnie_loc == AnimatronicLocation::LeftDoor {
            "225.png"
        } else {
            "58.png"
        }
    } else if office.right_light_on && flicker_on {
        if ai.chica_loc == AnimatronicLocation::RightDoor {
            "227.png"
        } else {
            "127.png"
        }
    } else {
        "39.png"
    };

    if !draw_sprite(assets, office_sprite, -pan_px, 0.0, None, None, WHITE).await {
        // Clean-room stylized Security Office (1600x720)
        vrect(-pan_px, 0.0, 1600.0, 720.0, Color::new(0.08, 0.08, 0.10, 1.0));
        // Back wall & checkered stripe
        vrect(320.0 - pan_px, 40.0, 960.0, 440.0, Color::new(0.14, 0.13, 0.16, 1.0));
        for i in 0..24 {
            let col = if i % 2 == 0 { WHITE } else { BLACK };
            vrect(320.0 - pan_px + (i as f32) * 40.0, 260.0, 40.0, 22.0, col);
        }
        // Celebrate Poster (with Freddy nose at 674, 236)
        vrect(560.0 - pan_px, 110.0, 220.0, 260.0, Color::new(0.20, 0.14, 0.12, 1.0));
        vrect_lines(560.0 - pan_px, 110.0, 220.0, 260.0, 2.0, DARKGRAY);
        vtext("CELEBRATE!", 588.0 - pan_px, 145.0, 28.0, YELLOW);
        vrect(650.0 - pan_px, 210.0, 48.0, 48.0, Color::new(0.45, 0.28, 0.14, 1.0));
        vrect(668.0 - pan_px, 230.0, 12.0, 10.0, BLACK);
        // Security Desk
        vrect(440.0 - pan_px, 440.0, 720.0, 280.0, Color::new(0.22, 0.20, 0.19, 1.0));
        vrect(440.0 - pan_px, 440.0, 720.0, 18.0, Color::new(0.32, 0.30, 0.28, 1.0));
        // Left & Right Doorways
        let left_bg = if office.left_light_on && flicker_on {
            Color::new(0.45, 0.45, 0.35, 1.0)
        } else {
            Color::new(0.03, 0.03, 0.04, 1.0)
        };
        vrect(72.0 - pan_px, 0.0, 223.0, 720.0, left_bg);
        if office.left_light_on && flicker_on && ai.bonnie_loc == AnimatronicLocation::LeftDoor {
            vrect(130.0 - pan_px, 180.0, 110.0, 320.0, Color::new(0.40, 0.30, 0.70, 1.0));
            vrect(155.0 - pan_px, 220.0, 20.0, 20.0, WHITE);
            vrect(195.0 - pan_px, 220.0, 20.0, 20.0, WHITE);
        }
        let right_bg = if office.right_light_on && flicker_on {
            Color::new(0.45, 0.45, 0.35, 1.0)
        } else {
            Color::new(0.03, 0.03, 0.04, 1.0)
        };
        vrect(1270.0 - pan_px, 0.0, 248.0, 720.0, right_bg);
        if office.right_light_on && flicker_on && ai.chica_loc == AnimatronicLocation::RightDoor {
            vrect(1335.0 - pan_px, 180.0, 110.0, 320.0, Color::new(0.85, 0.80, 0.20, 1.0));
            vrect(1360.0 - pan_px, 220.0, 20.0, 20.0, WHITE);
            vrect(1400.0 - pan_px, 220.0, 20.0, 20.0, WHITE);
        }
    }

    // Animated Desk Fan
    if !draw_sprite(
        assets,
        office.current_fan_sprite(),
        780.0 - pan_px,
        303.0,
        None,
        None,
        WHITE,
    )
    .await
    {
        vrect(825.0 - pan_px, 390.0, 36.0, 50.0, DARKGRAY);
        vrect_lines(798.0 - pan_px, 310.0, 90.0, 90.0, 3.0, GRAY);
        let blade = if ((global_time * 24.0) as i32) % 2 == 0 {
            (840.0 - pan_px, 320.0, 8.0, 70.0)
        } else {
            (808.0 - pan_px, 352.0, 70.0, 8.0)
        };
        vrect(blade.0, blade.1, blade.2, blade.3, LIGHTGRAY);
    }

    // Left Door
    if office.left_door_anim > 0.01
        && !draw_sprite(
            assets,
            office.current_left_door_sprite(),
            72.0 - pan_px,
            0.0,
            None,
            None,
            WHITE,
        )
        .await
    {
        vrect(
            72.0 - pan_px,
            0.0,
            223.0,
            720.0 * office.left_door_anim,
            Color::new(0.28, 0.30, 0.33, 1.0),
        );
    }

    // Right Door
    if office.right_door_anim > 0.01
        && !draw_sprite(
            assets,
            office.current_right_door_sprite(),
            1270.0 - pan_px,
            0.0,
            None,
            None,
            WHITE,
        )
        .await
    {
        vrect(
            1270.0 - pan_px,
            0.0,
            248.0,
            720.0 * office.right_door_anim,
            Color::new(0.28, 0.30, 0.33, 1.0),
        );
    }

    // Left & Right Control Panels
    if !draw_sprite(
        assets,
        office.left_panel_sprite(),
        6.0 - pan_px,
        263.0,
        None,
        None,
        WHITE,
    )
    .await
    {
        vrect(6.0 - pan_px, 263.0, 76.0, 220.0, DARKGRAY);
        vrect(
            16.0 - pan_px,
            278.0,
            56.0,
            84.0,
            if office.left_door_closed { GREEN } else { RED },
        );
        vtext("DOOR", 22.0 - pan_px, 326.0, 18.0, WHITE);
        vrect(
            16.0 - pan_px,
            382.0,
            56.0,
            84.0,
            if office.left_light_on { WHITE } else { GRAY },
        );
        vtext("LIGHT", 19.0 - pan_px, 430.0, 18.0, BLACK);
    }
    if !draw_sprite(
        assets,
        office.right_panel_sprite(),
        1498.0 - pan_px,
        263.0,
        None,
        None,
        WHITE,
    )
    .await
    {
        vrect(1514.0 - pan_px, 263.0, 76.0, 220.0, DARKGRAY);
        vrect(
            1524.0 - pan_px,
            278.0,
            56.0,
            84.0,
            if office.right_door_closed { GREEN } else { RED },
        );
        vtext("DOOR", 1530.0 - pan_px, 326.0, 18.0, WHITE);
        vrect(
            1524.0 - pan_px,
            382.0,
            56.0,
            84.0,
            if office.right_light_on { WHITE } else { GRAY },
        );
        vtext("LIGHT", 1527.0 - pan_px, 430.0, 18.0, BLACK);
    }
}

async fn draw_camera_view(
    office: &OfficeState,
    ai: &AnimatronicController,
    assets: &mut AssetManager,
    global_time: f32,
) {
    let cam = office.active_camera;
    if cam == CameraId::Cam6 {
        if !draw_sprite(assets, "42.png", 454.0, 72.0, None, None, WHITE).await {
            vtext("-CAMERA DISABLED-", 490.0, 120.0, 32.0, WHITE);
            vtext("AUDIO ONLY", 545.0, 160.0, 28.0, GRAY);
        }
    } else {
        let feed_sprite = ai.camera_feed_sprite(cam, global_time);
        let pan_x = office.camera_pan_x();
        if !draw_sprite(assets, feed_sprite, -pan_x, 0.0, None, None, WHITE).await {
            vrect(0.0, 0.0, VIRTUAL_W, VIRTUAL_H, Color::new(0.05, 0.06, 0.07, 1.0));
            vrect(120.0 - pan_x * 0.5, 120.0, 1040.0, 480.0, Color::new(0.10, 0.11, 0.13, 1.0));
            if ai.freddy_loc == AnimatronicLocation::Camera(cam) {
                vrect(560.0, 220.0, 90.0, 170.0, Color::new(0.55, 0.35, 0.15, 1.0));
                vtext("FREDDY", 565.0, 210.0, 20.0, WHITE);
            }
            if ai.bonnie_loc == AnimatronicLocation::Camera(cam) {
                vrect(360.0, 230.0, 90.0, 170.0, Color::new(0.45, 0.35, 0.75, 1.0));
                vtext("BONNIE", 365.0, 220.0, 20.0, WHITE);
            }
            if ai.chica_loc == AnimatronicLocation::Camera(cam) {
                vrect(740.0, 230.0, 90.0, 170.0, Color::new(0.85, 0.80, 0.20, 1.0));
                vtext("CHICA", 750.0, 220.0, 20.0, WHITE);
            }
            if cam == CameraId::Cam1C {
                vtext(
                    format!("PIRATE COVE STAGE: {}", ai.foxy_stage),
                    420.0,
                    340.0,
                    30.0,
                    RED,
                );
            }
        }
    }

    // Static Overlay
    let st_idx = ((global_time * 30.0) as usize) % STATIC_FRAMES.len();
    if !draw_sprite(
        assets,
        STATIC_FRAMES[st_idx],
        0.0,
        0.0,
        Some(VIRTUAL_W),
        Some(VIRTUAL_H),
        Color::new(1.0, 1.0, 1.0, 0.25),
    )
    .await
    {
        draw_procedural_static(global_time, 0.15);
    }

    // White monitor frame
    if !draw_sprite(
        assets,
        "11.png",
        0.0,
        0.0,
        Some(VIRTUAL_W),
        Some(VIRTUAL_H),
        WHITE,
    )
    .await
    {
        vrect_lines(20.0, 20.0, VIRTUAL_W - 40.0, VIRTUAL_H - 40.0, 3.0, WHITE);
    }

    // Blinking REC red dot
    if ((global_time * 2.0) as i32) % 2 == 0
        && !draw_sprite(assets, "7.png", 68.0, 56.0, None, None, WHITE).await
    {
        vrect(68.0, 56.0, 32.0, 32.0, RED);
    }

    // Room Name Banner
    if !draw_sprite(assets, cam.banner_sprite(), 834.0, 275.0, None, None, WHITE).await {
        vtext(cam.label(), 848.0, 298.0, 28.0, WHITE);
    }

    // Camera Map
    if !draw_sprite(assets, "145.png", 848.0, 313.0, None, None, WHITE).await {
        vrect(
            848.0,
            313.0,
            400.0,
            400.0,
            Color::new(0.05, 0.08, 0.06, 0.75),
        );
        vrect_lines(848.0, 313.0, 400.0, 400.0, 2.0, DARKGRAY);
    }

    // All 11 Camera Buttons
    for c in CameraId::ALL {
        let (ox, oy) = c.map_offset();
        let bx = 848.0 + ox;
        let by = 313.0 + oy;
        let box_sprite = if c == cam { "166.png" } else { "167.png" };
        if !draw_sprite(assets, box_sprite, bx, by, None, None, WHITE).await {
            let bg = if c == cam {
                Color::new(0.45, 0.75, 0.15, 1.0)
            } else {
                Color::new(0.22, 0.22, 0.25, 1.0)
            };
            vrect(bx, by, 60.0, 40.0, bg);
            vrect_lines(bx, by, 60.0, 40.0, 2.0, WHITE);
        }
        if !draw_sprite(
            assets,
            c.button_label_sprite(),
            bx + 14.0,
            by + 7.0,
            None,
            None,
            WHITE,
        )
        .await
        {
            vtext(c.code(), bx + 6.0, by + 26.0, 16.0, WHITE);
        }
    }
}

async fn draw_hud(
    office: &OfficeState,
    clock: &NightClock,
    night: u8,
    assets: &mut AssetManager,
) {
    // Top-right Clock & Night
    assets
        .draw_digits(clock.display_hour_num(), 1190.0, 32.0, 1.0, DigitSet::HudLarge, true)
        .await;
    if !draw_sprite(assets, "251.png", 1196.0, 36.0, None, None, WHITE).await {
        vtext("AM", 1196.0, 58.0, 28.0, WHITE);
    }
    if !draw_sprite(assets, "475.png", 1142.0, 74.0, None, None, WHITE).await {
        vtext("Night", 1142.0, 94.0, 22.0, LIGHTGRAY);
    }
    assets
        .draw_digits(u32::from(night), 1218.0, 72.0, 1.0, DigitSet::NightNum, false)
        .await;

    // Bottom-left Power & Usage
    if !draw_sprite(assets, "207.png", 38.0, 634.0, None, None, WHITE).await {
        vtext("Power left:", 38.0, 648.0, 24.0, WHITE);
    }
    let pwr = office.power_percent.ceil() as u32;
    assets
        .draw_digits(pwr, 185.0, 626.0, 0.85, DigitSet::HudLarge, false)
        .await;
    if !draw_sprite(assets, "208.png", 248.0, 634.0, None, None, WHITE).await {
        vtext("%", 248.0, 648.0, 24.0, WHITE);
    }

    if !draw_sprite(assets, "209.png", 38.0, 668.0, None, None, WHITE).await {
        vtext("Usage:", 38.0, 684.0, 22.0, WHITE);
    }
    if !draw_sprite(assets, office.usage_sprite(), 118.0, 660.0, None, None, WHITE).await {
        for i in 0..office.usage_bars() {
            let col = match i {
                0 | 1 => GREEN,
                2 => YELLOW,
                _ => RED,
            };
            vrect(118.0 + f32::from(i) * 24.0, 664.0, 18.0, 22.0, col);
        }
    }

    // Bottom-center Camera Flip Bar
    if !draw_sprite(assets, "420.png", 340.0, 650.0, None, None, WHITE).await {
        vrect(340.0, 658.0, 600.0, 44.0, Color::new(0.25, 0.25, 0.28, 0.75));
        vrect_lines(340.0, 658.0, 600.0, 44.0, 2.0, WHITE);
        vtext("^^^  CAMERAS  ^^^", 545.0, 686.0, 22.0, WHITE);
    }

    // Mute Call button on Night 1
    if !office.call_muted
        && night == 1
        && clock.elapsed_secs < 90.0
        && !draw_sprite(assets, "481.png", 26.0, 22.0, None, None, WHITE).await
    {
        vrect(26.0, 22.0, 121.0, 31.0, DARKGRAY);
        vrect_lines(26.0, 22.0, 121.0, 31.0, 2.0, WHITE);
        vtext("MUTE CALL", 36.0, 43.0, 18.0, WHITE);
    }
}
