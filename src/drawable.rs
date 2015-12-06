use cocoa::base::id;
use sys::MTLDrawable;

#[cfg(feature = "time2")]
use std::time::Instant;

pub struct Drawable(id);

impl Drawable {
	pub fn present(&mut self) {
		unsafe { self.0.present() }
	}

	#[cfg(feature = "time2")]
	pub fn present_at_time(&mut self, time: Instant) {
		unsafe { self.0.presentAtTime(time.elapsed().as_seconds()) }
	}

	pub fn present_at_time_secs(&mut self, time: f64) {
		unsafe { self.0.presentAtTime(time) }
	}
}
