use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 录屏宽度
    #[arg(short = 'W', long)]
    pub width: u32,

    /// 录屏高度
    #[arg(short = 'H', long)]
    pub height: u32,

    /// 录屏窗口标题（传入窗口标题）/全屏（传入屏幕标识）
    #[arg(short = 'T', long)]
    pub window_title: String,

    /// 视频格式
    #[arg(short, long, default_value  = "mp4")]
    pub video_format: String,

    /// 声音
    #[arg(short, long)]
    pub audio: bool,  // true 表示录音，false 表示不录音

    /// 录制边框
    #[arg(short, long)]
    pub border: bool,  // true 表示录制边框，false 不录制

    /// 是否显示鼠标
    #[arg(short, long)]
    pub show_mouse: bool,  // true 表示显示鼠标，false 表示不显示

    /// 帧率
    #[arg(short, long, default_value_t = 30)]
    pub framerate: u8,  // 默认帧率为 30

    /// 画质
    #[arg(short, long, default_value = "high")]
    pub quality: String,  // 默认画质为 "high"

    /// 导出文件路径
    #[arg(short, long)]
    pub output_path: String,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}