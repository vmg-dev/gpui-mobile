//! Camera package for camera discovery, live preview, photo/video capture,
//! and camera controls (flash, focus, exposure, zoom).
//!
//! Provides a cross-platform camera API backed by:
//! - iOS: AVFoundation (`AVCaptureSession`, `AVCaptureDevice`) via Objective-C
//! - Android: Camera2 API via JNI
//!
//! Inspired by [camera](https://pub.dev/packages/camera).
//!
//! Feature-gated behind `camera`.

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "ios")]
mod ios;

/// Direction the camera lens faces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraLensDirection {
    Front,
    Back,
    External,
}

/// Resolution preset for camera capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionPreset {
    /// 352x288 on iOS, 320x240 on Android.
    Low,
    /// 480p.
    Medium,
    /// 720p.
    High,
    /// 1080p.
    VeryHigh,
    /// 2160p (4K).
    UltraHigh,
    /// Highest available resolution.
    Max,
}

/// Flash mode for photo/video capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashMode {
    Off,
    Auto,
    Always,
    Torch,
}

/// Exposure mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExposureMode {
    Auto,
    Locked,
}

/// Focus mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusMode {
    Auto,
    Locked,
}

/// Describes a camera device available on the system.
#[derive(Debug, Clone)]
pub struct CameraDescription {
    /// Platform-specific camera identifier.
    pub name: String,
    /// Which direction the lens faces.
    pub lens_direction: CameraLensDirection,
    /// Sensor orientation in degrees (0, 90, 180, 270).
    pub sensor_orientation: i32,
}

/// Opaque handle to an active camera session.
#[derive(Debug)]
pub struct CameraHandle {
    /// Platform-specific session identifier.
    pub id: usize,
}

/// Result of a photo capture.
#[derive(Debug, Clone)]
pub struct CapturedImage {
    /// Absolute file path to the captured JPEG.
    pub path: String,
    /// Image width in pixels.
    pub width: u32,
    /// Image height in pixels.
    pub height: u32,
}

/// Result of a video recording.
#[derive(Debug, Clone)]
pub struct RecordedVideo {
    /// Absolute file path to the recorded video.
    pub path: String,
}

// ── Top-level functions ─────────────────────────────────────────────────────

/// List all available cameras on the device.
pub fn available_cameras() -> Result<Vec<CameraDescription>, String> {
    #[cfg(target_os = "ios")]
    {
        ios::available_cameras()
    }
    #[cfg(target_os = "android")]
    {
        android::available_cameras()
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        Err("camera is only available on iOS and Android".into())
    }
}

/// Create and initialize a camera session.
pub fn create_camera(
    camera: &CameraDescription,
    resolution: ResolutionPreset,
    enable_audio: bool,
) -> Result<CameraHandle, String> {
    #[cfg(target_os = "ios")]
    {
        ios::create_camera(camera, resolution, enable_audio)
    }
    #[cfg(target_os = "android")]
    {
        android::create_camera(camera, resolution, enable_audio)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = (camera, resolution, enable_audio);
        Err("camera is only available on iOS and Android".into())
    }
}

/// Start the camera preview (adds a native preview layer to the window).
pub fn start_preview(handle: &CameraHandle) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::start_preview(handle)
    }
    #[cfg(target_os = "android")]
    {
        android::start_preview(handle)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = handle;
        Err("camera is only available on iOS and Android".into())
    }
}

/// Stop the camera preview.
pub fn stop_preview(handle: &CameraHandle) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::stop_preview(handle)
    }
    #[cfg(target_os = "android")]
    {
        android::stop_preview(handle)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = handle;
        Err("camera is only available on iOS and Android".into())
    }
}

/// Capture a still photo, save to temp dir, return path + dimensions.
pub fn take_picture(handle: &CameraHandle) -> Result<CapturedImage, String> {
    #[cfg(target_os = "ios")]
    {
        ios::take_picture(handle)
    }
    #[cfg(target_os = "android")]
    {
        android::take_picture(handle)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = handle;
        Err("camera is only available on iOS and Android".into())
    }
}

/// Start recording video.
pub fn start_video_recording(handle: &CameraHandle) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::start_video_recording(handle)
    }
    #[cfg(target_os = "android")]
    {
        android::start_video_recording(handle)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = handle;
        Err("camera is only available on iOS and Android".into())
    }
}

/// Stop recording video and return the recorded file.
pub fn stop_video_recording(handle: &CameraHandle) -> Result<RecordedVideo, String> {
    #[cfg(target_os = "ios")]
    {
        ios::stop_video_recording(handle)
    }
    #[cfg(target_os = "android")]
    {
        android::stop_video_recording(handle)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = handle;
        Err("camera is only available on iOS and Android".into())
    }
}

/// Set flash mode.
pub fn set_flash_mode(handle: &CameraHandle, mode: FlashMode) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::set_flash_mode(handle, mode)
    }
    #[cfg(target_os = "android")]
    {
        android::set_flash_mode(handle, mode)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = (handle, mode);
        Err("camera is only available on iOS and Android".into())
    }
}

/// Set focus mode.
pub fn set_focus_mode(handle: &CameraHandle, mode: FocusMode) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::set_focus_mode(handle, mode)
    }
    #[cfg(target_os = "android")]
    {
        android::set_focus_mode(handle, mode)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = (handle, mode);
        Err("camera is only available on iOS and Android".into())
    }
}

/// Set exposure mode.
pub fn set_exposure_mode(handle: &CameraHandle, mode: ExposureMode) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::set_exposure_mode(handle, mode)
    }
    #[cfg(target_os = "android")]
    {
        android::set_exposure_mode(handle, mode)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = (handle, mode);
        Err("camera is only available on iOS and Android".into())
    }
}

/// Get the minimum zoom level.
pub fn get_min_zoom(handle: &CameraHandle) -> Result<f64, String> {
    #[cfg(target_os = "ios")]
    {
        ios::get_min_zoom(handle)
    }
    #[cfg(target_os = "android")]
    {
        android::get_min_zoom(handle)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = handle;
        Err("camera is only available on iOS and Android".into())
    }
}

/// Get the maximum zoom level.
pub fn get_max_zoom(handle: &CameraHandle) -> Result<f64, String> {
    #[cfg(target_os = "ios")]
    {
        ios::get_max_zoom(handle)
    }
    #[cfg(target_os = "android")]
    {
        android::get_max_zoom(handle)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = handle;
        Err("camera is only available on iOS and Android".into())
    }
}

/// Set zoom level (clamped to min/max range).
pub fn set_zoom(handle: &CameraHandle, zoom: f64) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::set_zoom(handle, zoom)
    }
    #[cfg(target_os = "android")]
    {
        android::set_zoom(handle, zoom)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = (handle, zoom);
        Err("camera is only available on iOS and Android".into())
    }
}

/// Switch to a different camera (e.g. front <-> back).
pub fn set_camera(handle: &CameraHandle, camera: &CameraDescription) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::set_camera(handle, camera)
    }
    #[cfg(target_os = "android")]
    {
        android::set_camera(handle, camera)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = (handle, camera);
        Err("camera is only available on iOS and Android".into())
    }
}

/// Release camera resources.
pub fn dispose(handle: CameraHandle) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::dispose(handle)
    }
    #[cfg(target_os = "android")]
    {
        android::dispose(handle)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        let _ = handle;
        Err("camera is only available on iOS and Android".into())
    }
}
