pub const SCRIPT_FILE_EXTENSION: &str = "rhai";
pub const SUPPORTED_SOUND_EXTENSIONS: &[&str] = &["wav", "ogg", "mp3", "flac"];
pub const CONFIG_FILE_PATH: &str = "./config.ini";
pub const ICON_PATH: &str = "./resources/icon.ico";
pub const SOUND_FOLDER_PATH: &str = "./resources/sounds";

pub const WINDOW_NAME: &str = "Space War Supreme!";

pub const DEFAULT_RESOLUTION: [u32; 2] = [1280, 720];
pub const DEFAULT_ASPECT_RATIO: f32 = DEFAULT_RESOLUTION[0] as f32 / DEFAULT_RESOLUTION[1] as f32;
pub const DEFAULT_VERTICAL_FOV_DEG: f32 = 65.0;
pub const DEFAULT_NEAR_CLIP: f32 = 0.01;
pub const DEFAULT_FAR_CLIP: f32 = 10000.0;
pub const DEFAULT_INSTANCE_BUFFER_SIZE: usize = 65536;
pub const DEFAULT_MAX_LIGHTS: usize = 2;

pub const MULTI_SKYBOX_WARNING_INTERVAL_SECONDS: f32 = 60.0;
pub const LOCALIZATION_PATH: &str = "./resources/localization";
pub const LOCALIZATION_EXTENSION: &str = "json";
pub const MAX_LOG_LINES: usize = 1000;
pub const CRASH_REPORTS_PATH: &str = "./crash_reports";