use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Failed to load asset: {0}")]
    AssetLoad(String),
    #[error("Invalid camera ID: {0}")]
    InvalidCamera(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameState {
    Warning,
    MainMenu,
    Settings,
    Newspaper,
    CustomNight,
    NightIntro { night: u8 },
    Playing,
    PowerOutage,
    Jumpscare { by: AnimatronicId },
    GameOver,
    NightComplete { night: u8 },
    Paycheck { night: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FpsLimit {
    Fps30,
    Fps60,
    Fps120,
    Fps144,
    Unlimited,
}

impl FpsLimit {
    pub fn next(self) -> Self {
        match self {
            Self::Fps30 => Self::Fps60,
            Self::Fps60 => Self::Fps120,
            Self::Fps120 => Self::Fps144,
            Self::Fps144 => Self::Unlimited,
            Self::Unlimited => Self::Fps30,
        }
    }

    pub fn as_u32(self) -> Option<u32> {
        match self {
            Self::Fps30 => Some(30),
            Self::Fps60 => Some(60),
            Self::Fps120 => Some(120),
            Self::Fps144 => Some(144),
            Self::Unlimited => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Fps30 => "30 FPS",
            Self::Fps60 => "60 FPS",
            Self::Fps120 => "120 FPS",
            Self::Fps144 => "144 FPS",
            Self::Unlimited => "UNLIMITED",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSettings {
    pub fps_limit: FpsLimit,
    pub show_fps: bool,
    pub unlimited_charge: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fps_limit: FpsLimit::Fps60,
            show_fps: false,
            unlimited_charge: false,
        }
    }
}

impl GameSettings {
    fn config_path() -> std::path::PathBuf {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                return dir.join("fnaf1_settings.cfg");
            }
        }
        std::path::PathBuf::from("fnaf1_settings.cfg")
    }

    pub fn load() -> Self {
        let Ok(content) = std::fs::read_to_string(Self::config_path()) else {
            return Self::default();
        };
        let mut settings = Self::default();
        for line in content.lines() {
            if let Some((k, v)) = line.split_once('=') {
                match k.trim() {
                    "fps_limit" => {
                        settings.fps_limit = match v.trim() {
                            "30" => FpsLimit::Fps30,
                            "60" => FpsLimit::Fps60,
                            "120" => FpsLimit::Fps120,
                            "144" => FpsLimit::Fps144,
                            "unlimited" => FpsLimit::Unlimited,
                            _ => FpsLimit::Fps60,
                        };
                    }
                    "show_fps" => settings.show_fps = v.trim() == "true",
                    "unlimited_charge" => settings.unlimited_charge = v.trim() == "true",
                    _ => {}
                }
            }
        }
        settings
    }

    pub fn save(&self) {
        let fps_str = match self.fps_limit {
            FpsLimit::Fps30 => "30",
            FpsLimit::Fps60 => "60",
            FpsLimit::Fps120 => "120",
            FpsLimit::Fps144 => "144",
            FpsLimit::Unlimited => "unlimited",
        };
        let data = format!(
            "fps_limit={fps_str}\nshow_fps={}\nunlimited_charge={}\n",
            self.show_fps, self.unlimited_charge
        );
        let _ = std::fs::write(Self::config_path(), data);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimatronicId {
    Freddy,
    Bonnie,
    Chica,
    Foxy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CameraId {
    Cam1A, // Show Stage
    Cam1B, // Dining Area
    Cam1C, // Pirate Cove
    Cam2A, // West Hall
    Cam2B, // W. Hall Corner
    Cam3,  // Supply Closet
    Cam4A, // East Hall
    Cam4B, // E. Hall Corner
    Cam5,  // Backstage
    Cam6,  // Kitchen (Audio Only)
    Cam7,  // Restrooms
}

impl CameraId {
    pub const ALL: [CameraId; 11] = [
        CameraId::Cam1A,
        CameraId::Cam1B,
        CameraId::Cam1C,
        CameraId::Cam2A,
        CameraId::Cam2B,
        CameraId::Cam3,
        CameraId::Cam4A,
        CameraId::Cam4B,
        CameraId::Cam5,
        CameraId::Cam6,
        CameraId::Cam7,
    ];

    pub fn code(&self) -> &'static str {
        match self {
            CameraId::Cam1A => "CAM 1A",
            CameraId::Cam1B => "CAM 1B",
            CameraId::Cam1C => "CAM 1C",
            CameraId::Cam2A => "CAM 2A",
            CameraId::Cam2B => "CAM 2B",
            CameraId::Cam3 => "CAM 3",
            CameraId::Cam4A => "CAM 4A",
            CameraId::Cam4B => "CAM 4B",
            CameraId::Cam5 => "CAM 5",
            CameraId::Cam6 => "CAM 6",
            CameraId::Cam7 => "CAM 7",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            CameraId::Cam1A => "Show Stage",
            CameraId::Cam1B => "Dining Area",
            CameraId::Cam1C => "Pirate Cove",
            CameraId::Cam2A => "West Hall",
            CameraId::Cam2B => "W. Hall Corner",
            CameraId::Cam3 => "Supply Closet",
            CameraId::Cam4A => "East Hall",
            CameraId::Cam4B => "E. Hall Corner",
            CameraId::Cam5 => "Backstage",
            CameraId::Cam6 => "Kitchen (Audio Only)",
            CameraId::Cam7 => "Restrooms",
        }
    }

    pub fn banner_sprite(&self) -> &'static str {
        match self {
            CameraId::Cam1A => "54.png",
            CameraId::Cam1B => "72.png",
            CameraId::Cam1C => "73.png",
            CameraId::Cam2A => "74.png",
            CameraId::Cam2B => "76.png",
            CameraId::Cam3 => "50.png",
            CameraId::Cam4A => "79.png",
            CameraId::Cam4B => "75.png",
            CameraId::Cam5 => "71.png",
            CameraId::Cam6 => "78.png",
            CameraId::Cam7 => "77.png",
        }
    }

    pub fn button_label_sprite(&self) -> &'static str {
        match self {
            CameraId::Cam1A => "170.png",
            CameraId::Cam1B => "171.png",
            CameraId::Cam1C => "177.png",
            CameraId::Cam2A => "172.png",
            CameraId::Cam2B => "165.png",
            CameraId::Cam3 => "168.png",
            CameraId::Cam4A => "169.png",
            CameraId::Cam4B => "173.png",
            CameraId::Cam5 => "174.png",
            CameraId::Cam6 => "175.png",
            CameraId::Cam7 => "176.png",
        }
    }

    /// Position on the 400x400 map (relative to map top-left in 1280x720 coordinates)
    pub fn map_offset(&self) -> (f32, f32) {
        match self {
            CameraId::Cam1A => (127.0, 26.0),
            CameraId::Cam1B => (107.0, 82.0),
            CameraId::Cam1C => (74.0, 165.0),
            CameraId::Cam2A => (126.0, 282.0),
            CameraId::Cam2B => (126.0, 322.0),
            CameraId::Cam3 => (42.0, 263.0),
            CameraId::Cam4A => (232.0, 282.0),
            CameraId::Cam4B => (232.0, 322.0),
            CameraId::Cam5 => (16.0, 114.0),
            CameraId::Cam6 => (318.0, 248.0),
            CameraId::Cam7 => (334.0, 116.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NightConfig {
    pub night: u8,
    pub freddy_ai: u8,
    pub bonnie_ai: u8,
    pub chica_ai: u8,
    pub foxy_ai: u8,
    pub hour_duration_secs: f32,
}

impl NightConfig {
    pub fn for_night(night: u8) -> Self {
        match night {
            1 => Self {
                night: 1,
                freddy_ai: 0,
                bonnie_ai: 3,
                chica_ai: 1,
                foxy_ai: 1,
                hour_duration_secs: 89.0, // Original FNAF 1 ~89s per hour (approx 9 min total)
            },
            2 => Self {
                night: 2,
                freddy_ai: 0,
                bonnie_ai: 5,
                chica_ai: 3,
                foxy_ai: 2,
                hour_duration_secs: 89.0,
            },
            3 => Self {
                night: 3,
                freddy_ai: 1,
                bonnie_ai: 7,
                chica_ai: 5,
                foxy_ai: 4,
                hour_duration_secs: 89.0,
            },
            4 => Self {
                night: 4,
                freddy_ai: 4,
                bonnie_ai: 9,
                chica_ai: 7,
                foxy_ai: 6,
                hour_duration_secs: 89.0,
            },
            5 => Self {
                night: 5,
                freddy_ai: 6,
                bonnie_ai: 12,
                chica_ai: 10,
                foxy_ai: 8,
                hour_duration_secs: 89.0,
            },
            6 => Self {
                night: 6,
                freddy_ai: 10,
                bonnie_ai: 14,
                chica_ai: 12,
                foxy_ai: 12,
                hour_duration_secs: 89.0,
            },
            _ => Self {
                night: 7,
                freddy_ai: 20,
                bonnie_ai: 20,
                chica_ai: 20,
                foxy_ai: 20,
                hour_duration_secs: 89.0,
            },
        }
    }

    pub fn custom(freddy_ai: u8, bonnie_ai: u8, chica_ai: u8, foxy_ai: u8) -> Self {
        Self {
            night: 7,
            freddy_ai: freddy_ai.min(20),
            bonnie_ai: bonnie_ai.min(20),
            chica_ai: chica_ai.min(20),
            foxy_ai: foxy_ai.min(20),
            hour_duration_secs: 89.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_list_completeness() {
        assert_eq!(CameraId::ALL.len(), 11);
        assert_eq!(CameraId::Cam1A.code(), "CAM 1A");
        assert_eq!(CameraId::Cam6.label(), "Kitchen (Audio Only)");
        assert_eq!(CameraId::Cam1A.banner_sprite(), "54.png");
    }

    #[test]
    fn test_night_config_scaling() {
        let n1 = NightConfig::for_night(1);
        let n5 = NightConfig::for_night(5);
        assert_eq!(n1.freddy_ai, 0);
        assert!(n5.freddy_ai > n1.freddy_ai);
        assert!(n5.bonnie_ai > n1.bonnie_ai);
    }
}
