use macroquad::audio::{
    load_sound, load_sound_from_bytes, play_sound, stop_sound, PlaySoundParams, Sound,
};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitSet {
    HudLarge, // 249, 252..260 (24x30) - Power % and Hour
    HudSmall, // 187, 191..199 (14x17) - Night in office? Wait: 328, 353, 360..367 (13x25)
    NightNum, // 328, 353, 360..367 (13x25)
    CustomAi, // 556..565 (35x75)
}

pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
    sounds: HashMap<String, Sound>,
    base_path: PathBuf,
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new(Self::resolve_assets_dir())
    }
}

impl AssetManager {
    pub fn resolve_assets_dir() -> PathBuf {
        // 1. Next to the executable (e.g. target/release/assets)
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let candidate = exe_dir.join("assets");
                if candidate.join("images").exists() {
                    return candidate;
                }
            }
        }
        // 2. Current working directory (./assets)
        let cwd_candidate = PathBuf::from("assets");
        if cwd_candidate.join("images").exists() {
            return cwd_candidate;
        }
        // 3. Compile-time project root fallback
        let manifest_candidate = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
        if manifest_candidate.join("images").exists() {
            return manifest_candidate;
        }
        PathBuf::from("assets")
    }

    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            textures: HashMap::new(),
            sounds: HashMap::new(),
            base_path: base_path.into(),
        }
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    pub fn image_path(&self, name: &str) -> PathBuf {
        self.base_path.join("images").join(name)
    }

    pub fn sound_path(&self, name: &str) -> PathBuf {
        self.base_path.join("sounds").join(name)
    }

    pub async fn get_texture(&mut self, name: &str) -> Option<Texture2D> {
        if let Some(tex) = self.textures.get(name) {
            return Some(tex.clone());
        }
        let path = self.image_path(name);
        let path_str = path.to_string_lossy().to_string();
        if let Ok(tex) = load_texture(&path_str).await {
            tex.set_filter(FilterMode::Linear);
            self.textures.insert(name.to_string(), tex.clone());
            Some(tex)
        } else {
            None
        }
    }

    pub async fn preload_textures(&mut self, names: &[&str]) {
        for &name in names {
            let _ = self.get_texture(name).await;
        }
    }

    pub async fn ensure_sound(&mut self, name: &str) -> Option<Sound> {
        if let Some(snd) = self.sounds.get(name) {
            return Some(snd.clone());
        }
        let path = self.sound_path(name);
        let path_str = path.to_string_lossy().to_string();
        if let Ok(snd) = load_sound(&path_str).await {
            self.sounds.insert(name.to_string(), snd.clone());
            return Some(snd);
        }
        // Procedural clean-room fallback audio when external assets/ are not present
        if let Some(wav_bytes) = Self::procedural_wav_for(name) {
            if let Ok(snd) = load_sound_from_bytes(&wav_bytes).await {
                self.sounds.insert(name.to_string(), snd.clone());
                return Some(snd);
            }
        }
        None
    }

    fn procedural_wav_for(name: &str) -> Option<Vec<u8>> {
        match name {
            "blip.wav" => Some(Self::synth_wav(880.0, 660.0, 0.06, 0.05, 0.45)),
            "SFXBible.wav" | "knoc.wav" => Some(Self::synth_wav(140.0, 55.0, 0.16, 0.55, 0.7)),
            "BallastHum.wav" => Some(Self::synth_wav(120.0, 120.0, 0.12, 0.35, 0.3)),
            "CAMERA_VIDEO_L.wav" => Some(Self::synth_wav(320.0, 520.0, 0.14, 0.4, 0.5)),
            "DOOR_POUNDING_.wav" => Some(Self::synth_wav(110.0, 45.0, 0.35, 0.7, 0.8)),
            "PartyFavorrasp.wav" => Some(Self::synth_wav(740.0, 920.0, 0.18, 0.1, 0.7)),
            "XSCREA.wav" | "erro.wav" => Some(Self::synth_wav(580.0, 290.0, 0.85, 0.85, 0.85)),
            "chimes.wav" | "CROWD_SMALL_CH.wav" => {
                Some(Self::synth_wav(523.25, 783.99, 0.7, 0.02, 0.6))
            }
            "deeps.wav" => Some(Self::synth_wav(90.0, 60.0, 0.15, 0.6, 0.35)),
            "Laugh_Giggle.wav" => Some(Self::synth_wav(260.0, 180.0, 0.35, 0.25, 0.5)),
            "powerd.wav" => Some(Self::synth_wav(220.0, 40.0, 0.9, 0.3, 0.65)),
            "music.wav" => Some(Self::synth_wav(330.0, 392.0, 0.8, 0.05, 0.4)),
            "stat.wav" => Some(Self::synth_wav(100.0, 100.0, 0.5, 0.95, 0.3)),
            _ => None,
        }
    }

    pub fn synth_wav(
        freq_start: f32,
        freq_end: f32,
        duration_secs: f32,
        noise_mix: f32,
        volume: f32,
    ) -> Vec<u8> {
        let sample_rate: u32 = 22050;
        let num_samples = (sample_rate as f32 * duration_secs) as usize;
        let data_size = (num_samples * 2) as u32;
        let mut buf = Vec::with_capacity(44 + data_size as usize);

        // RIFF WAV header
        buf.extend_from_slice(b"RIFF");
        buf.extend_from_slice(&(36 + data_size).to_le_bytes());
        buf.extend_from_slice(b"WAVE");
        buf.extend_from_slice(b"fmt ");
        buf.extend_from_slice(&16u32.to_le_bytes());
        buf.extend_from_slice(&1u16.to_le_bytes()); // PCM
        buf.extend_from_slice(&1u16.to_le_bytes()); // Mono
        buf.extend_from_slice(&sample_rate.to_le_bytes());
        buf.extend_from_slice(&(sample_rate * 2).to_le_bytes());
        buf.extend_from_slice(&2u16.to_le_bytes());
        buf.extend_from_slice(&16u16.to_le_bytes());
        buf.extend_from_slice(b"data");
        buf.extend_from_slice(&data_size.to_le_bytes());

        let mut phase = 0.0_f32;
        let mut lfsr: u32 = 0xACE1;
        for i in 0..num_samples {
            let t = i as f32 / num_samples.max(1) as f32;
            let freq = freq_start + (freq_end - freq_start) * t;
            phase += 2.0 * std::f32::consts::PI * freq / sample_rate as f32;
            let tone = phase.sin();

            lfsr = (lfsr >> 1) ^ (if lfsr & 1 != 0 { 0xB400 } else { 0 });
            let noise = ((lfsr as f32 / 65535.0) * 2.0) - 1.0;

            let env = (1.0 - t).powf(0.5);
            let sample = (tone * (1.0 - noise_mix) + noise * noise_mix) * env * volume;
            let pcm = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
            buf.extend_from_slice(&pcm.to_le_bytes());
        }
        buf
    }

    pub async fn play_sfx(&mut self, name: &str, looped: bool, volume: f32) {
        if let Some(snd) = self.ensure_sound(name).await {
            play_sound(&snd, PlaySoundParams { looped, volume });
        }
    }

    pub fn stop_sfx(&self, name: &str) {
        if let Some(snd) = self.sounds.get(name) {
            stop_sound(snd);
        }
    }

    pub fn stop_all_sfx(&self) {
        for snd in self.sounds.values() {
            stop_sound(snd);
        }
    }

    pub async fn draw_digits(
        &mut self,
        value: u32,
        x: f32,
        y: f32,
        scale: f32,
        set: DigitSet,
        align_right: bool,
    ) {
        let s = value.to_string();
        let mut textures = Vec::with_capacity(s.len());
        let mut total_w = 0.0_f32;

        for ch in s.chars() {
            let fname = Self::digit_filename(ch, set);
            if let Some(tex) = self.get_texture(fname).await {
                total_w += tex.width() * scale + 2.0 * scale;
                textures.push(tex);
            }
        }

        let sx = screen_width() / 1280.0;
        let sy = screen_height() / 720.0;

        if textures.len() != s.len() {
            let font_size = match set {
                DigitSet::CustomAi => 56.0 * scale * sy,
                DigitSet::HudLarge => 34.0 * scale * sy,
                _ => 24.0 * scale * sy,
            };
            let draw_x = if align_right {
                (x - (s.len() as f32) * 18.0 * scale) * sx
            } else {
                x * sx
            };
            draw_text(s, draw_x, (y + 26.0 * scale) * sy, font_size, WHITE);
            return;
        }

        let mut cur_x = if align_right { x - total_w } else { x };
        for tex in textures {
            let w = tex.width() * scale;
            let h = tex.height() * scale;
            draw_texture_ex(
                &tex,
                cur_x * sx,
                y * sy,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(w * sx, h * sy)),
                    ..Default::default()
                },
            );
            cur_x += w + 2.0 * scale;
        }
    }

    fn digit_filename(ch: char, set: DigitSet) -> &'static str {
        match set {
            DigitSet::HudLarge => match ch {
                '0' => "249.png",
                '1' => "252.png",
                '2' => "253.png",
                '3' => "254.png",
                '4' => "255.png",
                '5' => "256.png",
                '6' => "257.png",
                '7' => "258.png",
                '8' => "259.png",
                _ => "260.png",
            },
            DigitSet::HudSmall => match ch {
                '0' => "187.png",
                '1' => "191.png",
                '2' => "192.png",
                '3' => "193.png",
                '4' => "194.png",
                '5' => "195.png",
                '6' => "196.png",
                '7' => "197.png",
                '8' => "198.png",
                _ => "199.png",
            },
            DigitSet::NightNum => match ch {
                '0' => "328.png",
                '1' => "353.png",
                '2' => "360.png",
                '3' => "361.png",
                '4' => "362.png",
                '5' => "363.png",
                '6' => "364.png",
                '7' => "365.png",
                '8' => "366.png",
                _ => "367.png",
            },
            DigitSet::CustomAi => match ch {
                '0' => "556.png",
                '1' => "557.png",
                '2' => "558.png",
                '3' => "559.png",
                '4' => "560.png",
                '5' => "561.png",
                '6' => "562.png",
                '7' => "563.png",
                '8' => "564.png",
                _ => "565.png",
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_paths_and_resolution() {
        let mgr = AssetManager::new("assets");
        assert_eq!(mgr.image_path("39.png"), PathBuf::from("assets/images/39.png"));
        assert_eq!(
            mgr.sound_path("ambien.wav"),
            PathBuf::from("assets/sounds/ambien.wav")
        );
        let _resolved = AssetManager::resolve_assets_dir();
    }

    #[test]
    fn test_procedural_wav_header() {
        let wav = AssetManager::synth_wav(440.0, 220.0, 0.1, 0.1, 0.5);
        assert!(wav.len() > 44);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
    }
}
