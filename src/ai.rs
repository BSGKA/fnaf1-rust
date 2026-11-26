use crate::models::{AnimatronicId, CameraId, NightConfig};
use rand::Rng;

pub const FOXY_RUN_FRAMES: [&str; 16] = [
    "282.png", "286.png", "290.png", "292.png", "302.png", "306.png", "327.png", "329.png",
    "330.png", "331.png", "332.png", "333.png", "334.png", "335.png", "336.png", "337.png",
];

pub const BONNIE_JUMPSCARE: [&str; 11] = [
    "291.png", "293.png", "294.png", "295.png", "296.png", "297.png", "298.png", "299.png",
    "300.png", "301.png", "303.png",
];

pub const CHICA_JUMPSCARE: [&str; 16] = [
    "65.png", "69.png", "216.png", "228.png", "229.png", "230.png", "231.png", "232.png",
    "233.png", "234.png", "235.png", "236.png", "237.png", "239.png", "279.png", "281.png",
];

pub const FOXY_JUMPSCARE: [&str; 21] = [
    "242.png", "243.png", "396.png", "397.png", "398.png", "399.png", "400.png", "401.png",
    "402.png", "403.png", "404.png", "405.png", "406.png", "407.png", "408.png", "409.png",
    "410.png", "411.png", "412.png", "413.png", "415.png",
];

pub const FREDDY_OFFICE_JUMPSCARE: [&str; 28] = [
    "490.png", "491.png", "493.png", "495.png", "496.png", "497.png", "498.png", "499.png",
    "500.png", "501.png", "502.png", "503.png", "504.png", "505.png", "506.png", "507.png",
    "508.png", "509.png", "510.png", "511.png", "512.png", "513.png", "514.png", "515.png",
    "516.png", "517.png", "518.png", "519.png",
];

pub const FREDDY_OUTAGE_JUMPSCARE: [&str; 20] = [
    "307.png", "308.png", "309.png", "310.png", "311.png", "312.png", "313.png", "314.png",
    "315.png", "316.png", "317.png", "318.png", "319.png", "320.png", "321.png", "322.png",
    "323.png", "324.png", "325.png", "348.png",
];

pub fn jumpscare_frames(by: AnimatronicId, power_outage: bool) -> &'static [&'static str] {
    match by {
        AnimatronicId::Bonnie => &BONNIE_JUMPSCARE,
        AnimatronicId::Chica => &CHICA_JUMPSCARE,
        AnimatronicId::Foxy => &FOXY_JUMPSCARE,
        AnimatronicId::Freddy => {
            if power_outage {
                &FREDDY_OUTAGE_JUMPSCARE
            } else {
                &FREDDY_OFFICE_JUMPSCARE
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatronicLocation {
    Camera(CameraId),
    LeftDoor,
    RightDoor,
    InsideOffice,
}

#[derive(Debug, Clone)]
pub struct NightClock {
    pub elapsed_secs: f32,
    pub hour_duration_secs: f32,
}

impl NightClock {
    pub fn new(hour_duration_secs: f32) -> Self {
        Self {
            elapsed_secs: 0.0,
            hour_duration_secs,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.elapsed_secs += dt;
    }

    pub fn current_hour(&self) -> u8 {
        ((self.elapsed_secs / self.hour_duration_secs).floor() as u8).min(6)
    }

    pub fn is_6am(&self) -> bool {
        self.current_hour() >= 6
    }

    pub fn display_hour_num(&self) -> u32 {
        match self.current_hour() {
            0 => 12,
            h => u32::from(h),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimatronicController {
    pub bonnie_loc: AnimatronicLocation,
    pub chica_loc: AnimatronicLocation,
    pub freddy_loc: AnimatronicLocation,
    pub foxy_stage: u8, // 0: Hidden, 1: Peeking, 2: Ready, 3: Running/Attacking
    pub foxy_running: bool,
    pub foxy_run_timer: f32,
    bonnie_timer: f32,
    chica_timer: f32,
    freddy_timer: f32,
    foxy_timer: f32,
}

impl Default for AnimatronicController {
    fn default() -> Self {
        Self {
            bonnie_loc: AnimatronicLocation::Camera(CameraId::Cam1A),
            chica_loc: AnimatronicLocation::Camera(CameraId::Cam1A),
            freddy_loc: AnimatronicLocation::Camera(CameraId::Cam1A),
            foxy_stage: 0,
            foxy_running: false,
            foxy_run_timer: 0.0,
            bonnie_timer: 0.0,
            chica_timer: 0.0,
            freddy_timer: 0.0,
            foxy_timer: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiEvent {
    None,
    Footsteps,
    FreddyLaugh,
    KitchenSound,
    FoxyBangOnDoor,
    Jumpscare(AnimatronicId),
}

impl AnimatronicController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(
        &mut self,
        dt: f32,
        config: &NightConfig,
        left_door_closed: bool,
        right_door_closed: bool,
        cameras_up: bool,
        active_camera: CameraId,
    ) -> AiEvent {
        let mut rng = rand::thread_rng();

        // Foxy running down West Hall sequence
        if self.foxy_running {
            self.foxy_run_timer += dt;
            if self.foxy_run_timer >= 1.5 {
                self.foxy_running = false;
                self.foxy_run_timer = 0.0;
                self.foxy_stage = 0;
                if left_door_closed {
                    return AiEvent::FoxyBangOnDoor;
                } else {
                    return AiEvent::Jumpscare(AnimatronicId::Foxy);
                }
            }
            return AiEvent::None;
        }

        // Trigger Foxy run immediately when player checks CAM 2A at stage 3
        if self.foxy_stage == 3 && cameras_up && active_camera == CameraId::Cam2A {
            self.foxy_running = true;
            self.foxy_run_timer = 0.0;
            return AiEvent::None;
        }

        // Bonnie AI (every ~4.97s)
        self.bonnie_timer += dt;
        if self.bonnie_timer >= 4.97 {
            self.bonnie_timer = 0.0;
            if rng.gen_range(1..=20) <= config.bonnie_ai {
                if let Some(ev) = self.step_bonnie(left_door_closed, &mut rng) {
                    return ev;
                }
                return AiEvent::Footsteps;
            }
        }

        // Chica AI (every ~4.98s)
        self.chica_timer += dt;
        if self.chica_timer >= 4.98 {
            self.chica_timer = 0.0;
            if rng.gen_range(1..=20) <= config.chica_ai {
                if let Some(ev) = self.step_chica(right_door_closed, &mut rng) {
                    return ev;
                }
                if self.chica_loc == AnimatronicLocation::Camera(CameraId::Cam6) {
                    return AiEvent::KitchenSound;
                }
                return AiEvent::Footsteps;
            }
        }

        // Freddy AI (every ~3.02s, stalled if player is watching his camera)
        self.freddy_timer += dt;
        if self.freddy_timer >= 3.02 {
            self.freddy_timer = 0.0;
            let watched =
                cameras_up && self.freddy_loc == AnimatronicLocation::Camera(active_camera);
            if !watched && rng.gen_range(1..=20) <= config.freddy_ai {
                if let Some(ev) = self.step_freddy(right_door_closed) {
                    return ev;
                }
                return AiEvent::FreddyLaugh;
            }
        }

        // Foxy AI (every ~5.01s, stalled if player is watching any camera)
        self.foxy_timer += dt;
        if self.foxy_timer >= 5.01 {
            self.foxy_timer = 0.0;
            if !cameras_up && rng.gen_range(1..=20) <= config.foxy_ai {
                return self.step_foxy(left_door_closed);
            }
        }

        AiEvent::None
    }

    pub fn camera_feed_sprite(&self, cam: CameraId, time: f32) -> &'static str {
        let on = |loc: AnimatronicLocation, target: CameraId| loc == AnimatronicLocation::Camera(target);
        let freddy = on(self.freddy_loc, cam);
        let bonnie = on(self.bonnie_loc, cam);
        let chica = on(self.chica_loc, cam);

        match cam {
            CameraId::Cam1A => match (freddy, bonnie, chica) {
                (true, true, true) => "19.png",
                (true, false, true) => "68.png",
                (true, true, false) => "223.png",
                (true, false, false) => "224.png",
                _ => "484.png",
            },
            CameraId::Cam1B => {
                if freddy {
                    "492.png"
                } else if bonnie {
                    "90.png"
                } else if chica {
                    "215.png"
                } else {
                    "48.png"
                }
            }
            CameraId::Cam1C => match self.foxy_stage {
                0 => "66.png",
                1 => "211.png",
                2 => "338.png",
                _ => "240.png",
            },
            CameraId::Cam2A => {
                if self.foxy_running {
                    let idx = ((self.foxy_run_timer * 15.0) as usize).min(FOXY_RUN_FRAMES.len() - 1);
                    FOXY_RUN_FRAMES[idx]
                } else if bonnie {
                    "206.png"
                } else if (time * 8.0) as i32 % 5 == 0 {
                    "43.png"
                } else {
                    "44.png"
                }
            }
            CameraId::Cam2B => {
                if bonnie {
                    "188.png"
                } else {
                    "0.png"
                }
            }
            CameraId::Cam3 => {
                if bonnie {
                    "190.png"
                } else {
                    "62.png"
                }
            }
            CameraId::Cam4A => {
                if freddy {
                    "487.png"
                } else if chica {
                    "221.png"
                } else {
                    "67.png"
                }
            }
            CameraId::Cam4B => {
                if freddy {
                    "486.png"
                } else if chica {
                    "220.png"
                } else {
                    "49.png"
                }
            }
            CameraId::Cam5 => {
                if bonnie {
                    "205.png"
                } else {
                    "83.png"
                }
            }
            CameraId::Cam6 => "42.png", // Audio Only
            CameraId::Cam7 => {
                if freddy {
                    "494.png"
                } else if chica {
                    "217.png"
                } else {
                    "41.png"
                }
            }
        }
    }

    fn step_bonnie<R: Rng>(&mut self, left_door_closed: bool, rng: &mut R) -> Option<AiEvent> {
        match self.bonnie_loc {
            AnimatronicLocation::Camera(CameraId::Cam1A) => {
                self.bonnie_loc = if rng.gen_bool(0.5) {
                    AnimatronicLocation::Camera(CameraId::Cam1B)
                } else {
                    AnimatronicLocation::Camera(CameraId::Cam5)
                };
            }
            AnimatronicLocation::Camera(CameraId::Cam1B | CameraId::Cam5) => {
                self.bonnie_loc = AnimatronicLocation::Camera(CameraId::Cam2A);
            }
            AnimatronicLocation::Camera(CameraId::Cam2A) => {
                self.bonnie_loc = if rng.gen_bool(0.3) {
                    AnimatronicLocation::Camera(CameraId::Cam3)
                } else {
                    AnimatronicLocation::Camera(CameraId::Cam2B)
                };
            }
            AnimatronicLocation::Camera(CameraId::Cam3) => {
                self.bonnie_loc = AnimatronicLocation::Camera(CameraId::Cam2B);
            }
            AnimatronicLocation::Camera(CameraId::Cam2B) => {
                self.bonnie_loc = AnimatronicLocation::LeftDoor;
            }
            AnimatronicLocation::LeftDoor => {
                if left_door_closed {
                    self.bonnie_loc = AnimatronicLocation::Camera(CameraId::Cam1B);
                } else {
                    self.bonnie_loc = AnimatronicLocation::InsideOffice;
                    return Some(AiEvent::Jumpscare(AnimatronicId::Bonnie));
                }
            }
            _ => {}
        }
        None
    }

    fn step_chica<R: Rng>(&mut self, right_door_closed: bool, rng: &mut R) -> Option<AiEvent> {
        match self.chica_loc {
            AnimatronicLocation::Camera(CameraId::Cam1A) => {
                self.chica_loc = AnimatronicLocation::Camera(CameraId::Cam1B);
            }
            AnimatronicLocation::Camera(CameraId::Cam1B) => {
                self.chica_loc = if rng.gen_bool(0.5) {
                    AnimatronicLocation::Camera(CameraId::Cam7)
                } else {
                    AnimatronicLocation::Camera(CameraId::Cam6)
                };
            }
            AnimatronicLocation::Camera(CameraId::Cam7 | CameraId::Cam6) => {
                self.chica_loc = AnimatronicLocation::Camera(CameraId::Cam4A);
            }
            AnimatronicLocation::Camera(CameraId::Cam4A) => {
                self.chica_loc = AnimatronicLocation::Camera(CameraId::Cam4B);
            }
            AnimatronicLocation::Camera(CameraId::Cam4B) => {
                self.chica_loc = AnimatronicLocation::RightDoor;
            }
            AnimatronicLocation::RightDoor => {
                if right_door_closed {
                    self.chica_loc = AnimatronicLocation::Camera(CameraId::Cam1B);
                } else {
                    self.chica_loc = AnimatronicLocation::InsideOffice;
                    return Some(AiEvent::Jumpscare(AnimatronicId::Chica));
                }
            }
            _ => {}
        }
        None
    }

    fn step_freddy(&mut self, right_door_closed: bool) -> Option<AiEvent> {
        match self.freddy_loc {
            AnimatronicLocation::Camera(CameraId::Cam1A) => {
                self.freddy_loc = AnimatronicLocation::Camera(CameraId::Cam1B);
            }
            AnimatronicLocation::Camera(CameraId::Cam1B) => {
                self.freddy_loc = AnimatronicLocation::Camera(CameraId::Cam7);
            }
            AnimatronicLocation::Camera(CameraId::Cam7) => {
                self.freddy_loc = AnimatronicLocation::Camera(CameraId::Cam6);
            }
            AnimatronicLocation::Camera(CameraId::Cam6) => {
                self.freddy_loc = AnimatronicLocation::Camera(CameraId::Cam4A);
            }
            AnimatronicLocation::Camera(CameraId::Cam4A) => {
                self.freddy_loc = AnimatronicLocation::Camera(CameraId::Cam4B);
            }
            AnimatronicLocation::Camera(CameraId::Cam4B) if !right_door_closed => {
                self.freddy_loc = AnimatronicLocation::InsideOffice;
                return Some(AiEvent::Jumpscare(AnimatronicId::Freddy));
            }
            _ => {}
        }
        None
    }

    fn step_foxy(&mut self, left_door_closed: bool) -> AiEvent {
        if self.foxy_stage < 3 {
            self.foxy_stage += 1;
            AiEvent::None
        } else {
            self.foxy_stage = 0;
            if left_door_closed {
                AiEvent::FoxyBangOnDoor
            } else {
                AiEvent::Jumpscare(AnimatronicId::Foxy)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_night_clock_progression() {
        let mut clock = NightClock::new(60.0);
        assert_eq!(clock.display_hour_num(), 12);
        assert!(!clock.is_6am());

        clock.tick(65.0);
        assert_eq!(clock.display_hour_num(), 1);

        clock.tick(300.0);
        assert!(clock.is_6am());
        assert_eq!(clock.display_hour_num(), 6);
    }

    #[test]
    fn test_bonnie_blocked_by_closed_door() {
        let mut ai = AnimatronicController::new();
        ai.bonnie_loc = AnimatronicLocation::LeftDoor;
        let mut rng = rand::thread_rng();

        let ev = ai.step_bonnie(true, &mut rng);
        assert_eq!(ev, None);
        assert_eq!(ai.bonnie_loc, AnimatronicLocation::Camera(CameraId::Cam1B));
    }

    #[test]
    fn test_foxy_bang_and_jumpscare() {
        let mut ai = AnimatronicController::new();
        ai.foxy_stage = 3;
        assert_eq!(ai.step_foxy(true), AiEvent::FoxyBangOnDoor);
        assert_eq!(ai.foxy_stage, 0);

        ai.foxy_stage = 3;
        assert_eq!(ai.step_foxy(false), AiEvent::Jumpscare(AnimatronicId::Foxy));
    }

    #[test]
    fn test_camera_feed_sprite_mapping() {
        let mut ai = AnimatronicController::new();
        assert_eq!(ai.camera_feed_sprite(CameraId::Cam1A, 0.0), "19.png");
        ai.bonnie_loc = AnimatronicLocation::Camera(CameraId::Cam1B);
        assert_eq!(ai.camera_feed_sprite(CameraId::Cam1A, 0.0), "68.png");
        assert_eq!(ai.camera_feed_sprite(CameraId::Cam1B, 0.0), "90.png");
    }
}
