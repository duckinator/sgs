use tts::Tts;
use std::ops::Deref;

// TODO: Integrate macOS stuff, if I ever get a Mac to test it on. https://github.com/ndarilek/tts-rs/blob/master/examples/hello_world.rs

pub struct SpeechEngine {
    pub tts: Tts,
}

impl SpeechEngine {
    pub fn default() -> Result<Self, Box<dyn std::error::Error>> {
        let tts = Tts::default()?;

        Ok(SpeechEngine { tts })
    }


    pub fn speak<S: Into<String> + Deref<Target=str>>(&mut self, text: S) -> Result<(), Box<dyn std::error::Error>> {
        if text.len() == 0 {
            return Ok(());
        }

        // true = interrupt current speech; false = don't interrupt current speech.
        let interrupt = false;

        self.tts.speak(text, interrupt)?;
        Ok(())
    }

    pub fn stop(&mut self) {
        let _ = self.tts.stop();
    }

    pub fn is_speaking(&mut self) -> bool {
        // Questionable life choice: Assume not speaking if there's an error.
        self.tts.is_speaking().unwrap_or(false)
    }
}
