use tts::{Error, Tts, UtteranceId};

// TODO: Integrate macOS stuff, if I ever get a Mac to test it on. https://github.com/ndarilek/tts-rs/blob/master/examples/hello_world.rs

struct SpeechEngine {
    pub tts: Tts,
}

impl SpeechEngine {
    fn new() -> Self {
        // FIXME: Doesn't work on FreeBSD. This is an upstream problem: https://github.com/ndarilek/tts-rs/
        let tts = Tts::default().expect("Could not set up text-to-speech system.");

        SpeechEngine { tts }
    }

    fn speak<S: Into<String>>(&self, text: S) {
        // true = interrupt current speech; false = don't interrupt current speech.
        let interrupt = true;

        self.tts.speak(text, interrupt);
    }
}