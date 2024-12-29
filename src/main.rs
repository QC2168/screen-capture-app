extern crate winapi;
use clap::Parser;
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};


use std::{io::{self, Write}};
use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};
use windows_capture::window::Window;
use cpal;
use cpal::traits::{DeviceTrait, HostTrait};



// Handles capture events.
struct Capture {
    // The video encoder that will be used to encode the frames.
    encoder: Option<VideoEncoder>,
}

impl GraphicsCaptureApiHandler for Capture {
    // The type of flags used to get the values from the settings.
    type Flags = String;

    // The type of error that can be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create a new instance. The flags can be passed from settings.
    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        println!("Created with Flags: {}", ctx.flags);

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
        })
    }

    // Called every time a new frame is available.
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        _: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        self.encoder.as_mut().unwrap().send_frame(frame)?;
        self.encoder.as_mut().unwrap().send_audio_buffer(frame(), 0)?;
        Ok(())
    }

    // Optional handler called when the capture item (usually a window) closes.
    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture session ended");

        Ok(())
    }
}
fn main() {
    /// Simple program to greet a person
    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    struct Args {
        /// 录屏宽度
        #[arg(short = 'W', long)]
        width: u32,

        /// 录屏高度
        #[arg(short = 'H', long)]
        height: u32,

        /// 录屏窗口标题（传入窗口标题）/全屏（传入屏幕标识）
        #[arg(short = 'T', long)]
        window_title: String,

        /// 视频格式
        #[arg(short, long, default_value  = "mp4")]
        video_format: String,

        /// 声音
        #[arg(short, long)]
        audio: bool,  // true 表示录音，false 表示不录音

        /// 录制边框
        #[arg(short, long)]
        border: bool,  // true 表示录制边框，false 不录制

        /// 是否显示鼠标
        #[arg(short, long)]
        show_mouse: bool,  // true 表示显示鼠标，false 表示不显示

        /// 帧率
        #[arg(short, long, default_value_t = 30)]
        framerate: u8,  // 默认帧率为 30

        /// 画质
        #[arg(short, long, default_value = "high")]
        quality: String,  // 默认画质为 "high"

        /// 导出文件路径
        #[arg(short, long)]
        output_path: String,
    }
    let args = Args::parse();
    println!("录制参数");
    println!("{:?}", args);

    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");

    fn get_window(title:&str) -> Window {
        let window = Window::from_name(title).unwrap();
        println!("{:?}",window);
        window
    }
    let capture_target = get_window("QQ");
    // Gets the foreground window, refer to the docs for other capture items
    let primary_monitor = Monitor::primary().expect("There is no primary monitor");
    let cursor = if args.show_mouse {
        CursorCaptureSettings::WithCursor
    } else {
        CursorCaptureSettings::WithoutCursor
    };
    let border = if args.border {
        DrawBorderSettings::WithBorder
    } else {
        DrawBorderSettings::WithoutBorder
    };

    let settings = Settings::new(
        // Item to capture
        primary_monitor,
        // Capture cursor settings
        cursor,
        // Draw border settings
        border,
        // The desired color format for the captured frame.
        ColorFormat::default(),
        // Additional flags for the capture settings that will be passed to user defined `new` function.
        "Yea this works".to_string(),
    );

    // Starts the capture and takes control of the current thread.
    // The errors from handler trait will end up here

    let i=Capture::start_free_threaded(settings).expect("Screen capture failed");

    loop {
        // 提示用户输入
        print!("请输入命令: ");
        io::stdout().flush().unwrap(); // 刷新stdout以确保提示立即显示
        // 获取用户输入
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // 去掉输入末尾的换行符
        let input = input.trim();
        // 如果输入为空，继续监听
        if input.is_empty() {
            continue;
        }
        // 根据输入退出程序
        if input == "exit" || input == "stop" {
            i.stop().expect("Failed to stop capture");
            println!("退出程序");
            break;
        }
    }

}

