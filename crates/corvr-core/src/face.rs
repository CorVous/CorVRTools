use crate::Expression;

/// Tracking data for one eye.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Eye {
    /// Gaze direction, each axis -1.0 to 1.0 (x right, y up).
    pub gaze: (f32, f32),
    /// 0.0 closed to 1.0 open.
    pub openness: f32,
    /// Pupil diameter in millimetres.
    pub pupil_mm: f32,
}

impl Default for Eye {
    fn default() -> Self {
        Self {
            gaze: (0.0, 0.0),
            openness: 1.0,
            pupil_mm: 5.0,
        }
    }
}

/// Everything the trackers know about the face at one moment.
///
/// Each tracker fills in only the parts it owns: the mouth tracker writes
/// mouth, jaw, cheek and tongue shapes, the eye tracker writes the eyes and
/// eye shapes, and the brow tracker writes brow shapes.
#[derive(Debug, Clone, PartialEq)]
pub struct FaceState {
    pub left: Eye,
    pub right: Eye,
    shapes: [f32; Expression::COUNT],
    pub eye_tracking_active: bool,
    pub lip_tracking_active: bool,
}

impl Default for FaceState {
    fn default() -> Self {
        Self {
            left: Eye::default(),
            right: Eye::default(),
            shapes: [0.0; Expression::COUNT],
            eye_tracking_active: false,
            lip_tracking_active: false,
        }
    }
}

impl FaceState {
    /// Weight of a shape, 0.0 to 1.0.
    pub fn get(&self, e: Expression) -> f32 {
        self.shapes[e as usize]
    }

    /// Sets a shape's weight, clamped to 0.0 to 1.0. NaN becomes 0.0.
    pub fn set(&mut self, e: Expression, weight: f32) {
        self.shapes[e as usize] = if weight.is_nan() {
            0.0
        } else {
            weight.clamp(0.0, 1.0)
        };
    }

    /// All shapes with their weights, in VRCFT order.
    pub fn shapes(&self) -> impl Iterator<Item = (Expression, f32)> + '_ {
        Expression::ALL.iter().map(|&e| (e, self.get(e)))
    }

    /// Average of both eyes.
    pub fn combined_eye(&self) -> Eye {
        let avg = |a: f32, b: f32| (a + b) / 2.0;
        Eye {
            gaze: (
                avg(self.left.gaze.0, self.right.gaze.0),
                avg(self.left.gaze.1, self.right.gaze.1),
            ),
            openness: avg(self.left.openness, self.right.openness),
            pupil_mm: avg(self.left.pupil_mm, self.right.pupil_mm),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_clamps_and_rejects_nan() {
        let mut s = FaceState::default();
        s.set(Expression::JawOpen, 1.5);
        assert_eq!(s.get(Expression::JawOpen), 1.0);
        s.set(Expression::JawOpen, f32::NAN);
        assert_eq!(s.get(Expression::JawOpen), 0.0);
        s.set(Expression::JawOpen, -1.0);
        assert_eq!(s.get(Expression::JawOpen), 0.0);
    }
}
