use nannou::prelude::*;
use chrono::offset::Local;

pub fn capture_folder_path(app: &App) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_project>/captures/<source_name>`.
        .join("captures")
        .join(app.exe_name().unwrap())
}

pub fn capture_path_timestamp(app: &App) -> std::path::PathBuf {
    capture_folder_path(app)
            .join(escaped_timestamp())
            .with_extension("png")
}

pub fn capture_path_frame_count(app: &App, frame: &Frame) -> std::path::PathBuf {
    capture_folder_path(app)
            .join(format!("{}", frame.nth()))
            .with_extension("png")
}


fn escaped_timestamp() -> String {
    format!("{:?}", Local::now()).replace(":", "-")
}