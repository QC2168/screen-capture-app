mod capture;
mod audio;
mod config;

use std::{io::{self, Write}};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use windows_capture::{
    capture::{GraphicsCaptureApiHandler},
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};
use crate::capture::Capture;
use crate::config::Args;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let args = Args::new();
    println!("录制参数");
    println!("{:?}", args);
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

    let capture_control = Capture::start_free_threaded(settings)
        .expect("Screen capture failed");

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
        if input == "exit" || input == "stop"|| input == "q" {
            // 直接使用 running 来停止音频
            // running.store(false, Ordering::Relaxed);
            if let Err(e) = capture_control.stop() {
                eprintln!("停止录制时发生错误: {:?}", e);
            }
            println!("退出程序");
            break;
        }
    }

}

