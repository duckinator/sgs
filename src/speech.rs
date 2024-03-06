use tts::Tts;

// TODO: Integrate macOS stuff, if I ever get a Mac to test it on. https://github.com/ndarilek/tts-rs/blob/master/examples/hello_world.rs

pub struct SpeechEngine {
    pub tts: Tts,
}

impl SpeechEngine {
    pub fn default() -> Result<Self, Box<dyn std::error::Error>> {
        let tts = Tts::default()?;

        Ok(SpeechEngine { tts })
    }


    pub fn speak<S: Into<String>>(&mut self, text: S) -> Result<(), Box<dyn std::error::Error>> {
        // true = interrupt current speech; false = don't interrupt current speech.
        let interrupt = false;

        self.tts.speak(text, interrupt)?;
        Ok(())
    }
}
