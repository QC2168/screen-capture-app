use cpal;
use cpal::traits::{HostTrait};

pub fn init_audio() -> cpal::Device {
    let host = cpal::default_host();
    host.default_output_device().expect("no output device available")
}