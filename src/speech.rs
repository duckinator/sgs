use tts::Tts;

// TODO: Integrate macOS stuff, if I ever get a Mac to test it on. https://github.com/ndarilek/tts-rs/blob/master/examples/hello_world.rs

pub struct SpeechEngine {
    pub tts: Tts,
}

impl Default for SpeechEngine {
    fn default() -> Self {
        // FIXME: Doesn't work on FreeBSD. This is an upstream problem: https://github.com/ndarilek/tts-rs/
        let tts = Tts::default().expect("Could not set up text-to-speech system.");

        SpeechEngine { tts }
    }
}

impl SpeechEngine {
    pub fn speak<S: Into<String>>(&mut self, text: S) -> Result<(), Box<dyn std::error::Error>> {
        // true = interrupt current speech; false = don't interrupt current speech.
        let interrupt = true;

        match self.tts.speak(text, interrupt) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
