use crate::models::CameraId;

pub const LEFT_DOOR_FRAMES: [&str; 16] = [
    "88.png", "89.png", "91.png", "92.png", "93.png", "94.png", "95.png", "96.png",
    "97.png", "98.png", "99.png", "100.png", "101.png", "102.png", "103.png", "105.png",
];

pub const RIGHT_DOOR_FRAMES: [&str; 17] = [
    "104.png", "106.png", "107.png", "108.png", "109.png", "110.png", "111.png", "112.png",
    "113.png", "114.png", "115.png", "116.png", "117.png", "118.png", "119.png", "121.png",
    "121.png",
];

pub const CAM_FLIP_FRAMES: [&str; 9] = [
    "132.png", "133.png", "136.png", "137.png", "138.png", "139.png", "140.png", "141.png",
    "142.png",
];

pub const FAN_FRAMES: [&str; 3] = ["57.png", "59.png", "60.png"];

#[derive(Debug, Clone)]
pub struct OfficeState {
    pub left_door_closed: bool,
    pub right_door_closed: bool,
    pub left_door_anim: f32,  // 0.0 (open) -> 1.0 (closed)
    pub right_door_anim: f32, // 0.0 (open) -> 1.0 (closed)
    pub left_light_on: bool,
    pub right_light_on: bool,
    pub cameras_up: bool,
    pub cam_flip_anim: f32, // 0.0 (down) -> 1.0 (fully up)
    pub active_camera: CameraId,
    pub power_percent: f32,
    pub pan_offset: f32, // 0.0 (left edge, 0px) -> 1.0 (right edge, 320px)
    pub cam_pan_time: f32,
    pub fan_time: f32,
    pub call_muted: bool,
}

impl Default for OfficeState {
    fn default() -> Self {
        Self {
            left_door_closed: false,
            right_door_closed: false,
            left_door_anim: 0.0,
            right_door_anim: 0.0,
            left_light_on: false,
            right_light_on: false,
            cameras_up: false,
            cam_flip_anim: 0.0,
            active_camera: CameraId::Cam1A,
            power_percent: 100.0,
            pan_offset: 0.5,
            cam_pan_time: 0.0,
            fan_time: 0.0,
            call_muted: false,
        }
    }
}

impl OfficeState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick_animations(&mut self, dt: f32) {
        self.fan_time += dt;
        self.cam_pan_time += dt;

        let door_speed = 6.5; // ~0.15s open/close
        if self.left_door_closed {
            self.left_door_anim = (self.left_door_anim + dt * door_speed).min(1.0);
        } else {
            self.left_door_anim = (self.left_door_anim - dt * door_speed).max(0.0);
        }

        if self.right_door_closed {
            self.right_door_anim = (self.right_door_anim + dt * door_speed).min(1.0);
        } else {
            self.right_door_anim = (self.right_door_anim - dt * door_speed).max(0.0);
        }

        let flip_speed = 5.5; // ~0.18s monitor flip
        if self.cameras_up {
            self.cam_flip_anim = (self.cam_flip_anim + dt * flip_speed).min(1.0);
        } else {
            self.cam_flip_anim = (self.cam_flip_anim - dt * flip_speed).max(0.0);
        }
    }

    pub fn is_camera_feed_visible(&self) -> bool {
        self.cameras_up && self.cam_flip_anim >= 0.99
    }

    pub fn is_camera_flipping(&self) -> bool {
        self.cam_flip_anim > 0.01 && self.cam_flip_anim < 0.99
    }

    pub fn current_cam_flip_sprite(&self) -> &'static str {
        let idx = ((self.cam_flip_anim * (CAM_FLIP_FRAMES.len() - 1) as f32).round() as usize)
            .min(CAM_FLIP_FRAMES.len() - 1);
        CAM_FLIP_FRAMES[idx]
    }

    pub fn current_left_door_sprite(&self) -> &'static str {
        let idx = ((self.left_door_anim * (LEFT_DOOR_FRAMES.len() - 1) as f32).round() as usize)
            .min(LEFT_DOOR_FRAMES.len() - 1);
        LEFT_DOOR_FRAMES[idx]
    }

    pub fn current_right_door_sprite(&self) -> &'static str {
        let idx = ((self.right_door_anim * (RIGHT_DOOR_FRAMES.len() - 1) as f32).round() as usize)
            .min(RIGHT_DOOR_FRAMES.len() - 1);
        RIGHT_DOOR_FRAMES[idx]
    }

    pub fn current_fan_sprite(&self) -> &'static str {
        let idx = ((self.fan_time * 24.0) as usize) % FAN_FRAMES.len();
        FAN_FRAMES[idx]
    }

    pub fn left_panel_sprite(&self) -> &'static str {
        match (self.left_door_closed, self.left_light_on) {
            (false, false) => "122.png",
            (true, false) => "124.png",
            (false, true) => "125.png",
            (true, true) => "130.png",
        }
    }

    pub fn right_panel_sprite(&self) -> &'static str {
        match (self.right_door_closed, self.right_light_on) {
            (false, false) => "134.png",
            (true, false) => "135.png",
            (false, true) => "131.png",
            (true, true) => "47.png",
        }
    }

    pub fn usage_sprite(&self) -> &'static str {
        match self.usage_bars() {
            1 => "212.png",
            2 => "213.png",
            3 => "214.png",
            4 => "455.png",
            _ => "456.png",
        }
    }

    /// Automatic smooth camera panning across 1600px -> 1280px (0.0..320.0)
    pub fn camera_pan_x(&self) -> f32 {
        // 12-second cycle: pause 2s left, sweep 4s right, pause 2s right, sweep 4s left
        let t = self.cam_pan_time % 12.0;
        let norm = if t < 2.0 {
            0.0
        } else if t < 6.0 {
            (t - 2.0) / 4.0
        } else if t < 8.0 {
            1.0
        } else {
            1.0 - (t - 8.0) / 4.0
        };
        norm * 320.0
    }

    pub fn usage_bars(&self) -> u8 {
        let mut bars = 1; // Base fan
        if self.left_door_closed {
            bars += 1;
        }
        if self.right_door_closed {
            bars += 1;
        }
        if self.left_light_on || self.right_light_on {
            bars += 1;
        }
        if self.cameras_up {
            bars += 1;
        }
        bars
    }

    /// Drains power based on active usage bars. Returns `true` if power just hit 0.
    pub fn drain_power(&mut self, dt: f32, unlimited_charge: bool) -> bool {
        if unlimited_charge {
            self.power_percent = 100.0;
            return false;
        }
        if self.power_percent <= 0.0 {
            return false;
        }
        let drain_rate = 0.104 * f32::from(self.usage_bars());
        self.power_percent = (self.power_percent - drain_rate * dt).max(0.0);
        if self.power_percent <= 0.0 {
            self.left_door_closed = false;
            self.right_door_closed = false;
            self.left_light_on = false;
            self.right_light_on = false;
            self.cameras_up = false;
            return true;
        }
        false
    }

    pub fn apply_foxy_drain(&mut self, unlimited_charge: bool) {
        if !unlimited_charge {
            self.power_percent = (self.power_percent - 5.0).max(0.0);
        }
    }

    pub fn toggle_left_door(&mut self) {
        if self.power_percent > 0.0 {
            self.left_door_closed = !self.left_door_closed;
        }
    }

    pub fn toggle_right_door(&mut self) {
        if self.power_percent > 0.0 {
            self.right_door_closed = !self.right_door_closed;
        }
    }

    pub fn toggle_left_light(&mut self) {
        if self.power_percent > 0.0 {
            self.left_light_on = !self.left_light_on;
            if self.left_light_on {
                self.right_light_on = false;
            }
        }
    }

    pub fn toggle_right_light(&mut self) {
        if self.power_percent > 0.0 {
            self.right_light_on = !self.right_light_on;
            if self.right_light_on {
                self.left_light_on = false;
            }
        }
    }

    pub fn toggle_cameras(&mut self) {
        if self.power_percent > 0.0 {
            self.cameras_up = !self.cameras_up;
            if self.cameras_up {
                self.left_light_on = false;
                self.right_light_on = false;
            }
        }
    }

    pub fn select_camera(&mut self, cam: CameraId) {
        self.active_camera = cam;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_bars_and_lights_mutual_exclusion() {
        let mut office = OfficeState::new();
        assert_eq!(office.usage_bars(), 1);
        assert_eq!(office.left_panel_sprite(), "122.png");
        assert_eq!(office.right_panel_sprite(), "134.png");

        office.toggle_left_door();
        office.toggle_left_light();
        assert_eq!(office.usage_bars(), 3);
        assert_eq!(office.left_panel_sprite(), "130.png");

        office.toggle_right_light();
        assert!(!office.left_light_on);
        assert!(office.right_light_on);
        assert_eq!(office.right_panel_sprite(), "131.png");
    }

    #[test]
    fn test_power_outage_resets_controls() {
        let mut office = OfficeState::new();
        office.power_percent = 0.05;
        office.toggle_left_door();
        office.toggle_cameras();

        let outage = office.drain_power(1.0, false);
        assert!(outage);
        assert_eq!(office.power_percent, 0.0);
        assert!(!office.left_door_closed);
        assert!(!office.cameras_up);
    }

    #[test]
    fn test_unlimited_charge_cheat() {
        let mut office = OfficeState::new();
        office.toggle_left_door();
        office.toggle_right_door();
        let outage = office.drain_power(10.0, true);
        assert!(!outage);
        assert_eq!(office.power_percent, 100.0);
        office.apply_foxy_drain(true);
        assert_eq!(office.power_percent, 100.0);
    }

    #[test]
    fn test_door_and_flip_animations() {
        let mut office = OfficeState::new();
        assert_eq!(office.current_left_door_sprite(), "88.png");
        office.toggle_left_door();
        office.tick_animations(0.5);
        assert_eq!(office.current_left_door_sprite(), "105.png");
    }
}
