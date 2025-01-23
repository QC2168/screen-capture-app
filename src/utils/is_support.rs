use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder},
    frame::Frame,
    graphics_capture_api::{InternalCaptureControl,GraphicsCaptureApi},
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
    window::Window
};

fn main() {
    let is=GraphicsCaptureApi::is_supported().expect("GraphicsCaptureApi is not supported on this system");
    println!("{}", is);
    let monitors=Window::enumerate();
    for monitor in monitors {
        println!("{:?}", monitor);
    }
}