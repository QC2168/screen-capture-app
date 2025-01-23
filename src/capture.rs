use std::sync::{mpsc, Arc, Mutex};
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use windows_capture::capture::{Context, GraphicsCaptureApiHandler};
use windows_capture::encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder};
use windows_capture::frame::Frame;
use windows_capture::graphics_capture_api::InternalCaptureControl;
use windows_capture::settings::Settings;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Capture {
    encoder: Option<VideoEncoder>,
    audio_buffer: Arc<Mutex<Vec<u8>>>,
    audio_receiver: mpsc::Receiver<Vec<u8>>, // 用于接收音频数据的通道
    running: Arc<AtomicBool>, // 新增字段

}

impl Capture {
    // 现有的方法改为不需要 mut self
    pub fn stop_audio(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

impl GraphicsCaptureApiHandler for Capture {
    type Flags = String;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        println!("Created with Flags: {}", ctx.flags);

        let (tx, rx) = mpsc::channel();
        let audio_buffer = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(AtomicBool::new(true)); // 初始化为 true
        let running_clone = running.clone();

        // 启动音频捕获线程
        std::thread::spawn(move || {
            let host = cpal::default_host();
            let output_device = host.default_output_device().expect("无法获取默认输入设备");
            let config = output_device.default_output_config().expect("无法获取默认输入格式");

            fn f32_to_u8_buffer(audio_data: &[f32]) -> Vec<u8> {
                let mut buffer = Vec::with_capacity(audio_data.len() * 4); // 每个 f32 占用 4 字节
                for &sample in audio_data {
                    let bytes = sample.to_ne_bytes(); // 将 f32 转换为字节数组
                    buffer.extend_from_slice(&bytes);
                }
                buffer
            }

            let stream = output_device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let buffer = f32_to_u8_buffer(data);
                    tx.send(buffer).unwrap();
                },
                move |err| {
                    eprintln!("音频流错误: {:?}", err);
                },
                None,
            ).unwrap();

            stream.play().unwrap();
            while running_clone.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(100));
            }
            // 当循环结束时，停止音频流
            drop(stream);
        });

        let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

        let encoder = VideoEncoder::new(
            VideoSettingsBuilder::new(screen_width as u32, screen_height as u32),
            AudioSettingsBuilder::default().disabled(true),
            ContainerSettingsBuilder::default(),
            "./target/video.mp4",
        )?;

        Ok(Self {
            encoder: Some(encoder),
            audio_buffer,
            audio_receiver: rx,
            running, // 添加新字段
        })
    }

    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        _: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        while let Ok(audio_data) = self.audio_receiver.try_recv() {
            let mut audio_buffer = self.audio_buffer.lock().unwrap();
            audio_buffer.extend_from_slice(&audio_data); // 将新数据追加到 audio_buffer
        }

        // 获取当前的音频数据
        let audio_buffer = self.audio_buffer.lock().unwrap();
        self.encoder.as_mut().unwrap().send_frame(frame)?;
        // self.encoder.as_mut().unwrap().send_frame_with_audio(frame, &**audio_buffer)?;
        Ok(())
    }

    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture session ended");
        // 设置 running 为 false，通知音频捕获线程结束
        self.running.store(false, Ordering::Relaxed);
        Ok(())
    }
}
