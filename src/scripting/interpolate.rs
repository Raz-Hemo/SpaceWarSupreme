/// Different interpolation methods make the motion appear different.
#[derive(Debug, Clone, Copy)]
pub enum InterpType {
    /// Snap to target immediately
    Constant,

    Smoothstep,
    Linear,
}

/// A type that is Interpolate can transition from two states smoothly based on
/// 0 < alpha < 1. An alpha of 0 yields the source state while 1 yields the dest.
pub trait Interpolate {
    fn get(&self, other: &Self, alpha: f32) -> Self;
}

/// Wraps an Interpolate type for abstracted storage
#[derive(Debug, Clone, Copy)]
pub struct Interpolated<T: Interpolate + Clone + Copy> {
    src: T,
    dst: T,
    start: std::time::Instant,
    duration: f32,
    interp: InterpType,
}

impl<T: Interpolate + Clone + Copy> Interpolated<T> {
    pub fn new(source: T) -> Interpolated<T> {
        Interpolated {
            src: source,
            dst: source,
            start: std::time::Instant::now(),
            duration: 1.0,
            interp: InterpType::Constant,
        }
    }
    pub fn get(&self) -> T {
        if self.duration == 0.0 {
            return self.dst
        }

        let normtime = crate::utils::clamp(
            self.start.elapsed().as_secs_f32() / self.duration,
            0.0,
            1.0
        );
        let alpha = match self.interp {
            InterpType::Constant => 1.0,
            InterpType::Linear => normtime,
            InterpType::Smoothstep => normtime * normtime * (3.0 - 2.0 * normtime),
        };
        self.src.get(&self.dst,alpha)
    }

    pub fn set(&mut self, dst: T, interp: InterpType, duration: f32) {
        self.src = self.get();
        self.dst = dst;
        self.interp = interp;
        self.duration = duration;
        self.start = std::time::Instant::now();
    }
}