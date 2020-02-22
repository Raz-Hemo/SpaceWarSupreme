pub struct Engine<'a> {
    pub input: crate::input::InputInfo<'a>,
    pub cfg: crate::config::Config,
}

impl<'a> Engine<'a> {
    pub fn new() -> Engine<'a> {
        Engine {
            input: crate::input::InputInfo::new(),
            cfg: crate::config::Config::load(),
        }
    }
}
