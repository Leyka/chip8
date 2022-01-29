// Reference: https://nukep.github.io/glium-sdl2/sdl2/audio/index.html

use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    Sdl,
};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = match self.phase {
                v if v >= 0.0 && v <= 0.5 => self.volume,
                _ => -self.volume,
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Speaker {
    audio: AudioDevice<SquareWave>,
}

impl Speaker {
    pub fn new(sdl: &Sdl) -> Self {
        let audio_subsystem = sdl.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let audio = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .unwrap();

        Speaker { audio }
    }

    /// Emit a buzz sound
    pub fn emit_sound(&mut self) {
        self.audio.resume()
    }

    /// Stop emitting buzz sound
    pub fn stop_emitting(&mut self) {
        self.audio.pause()
    }
}
