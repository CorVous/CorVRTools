//! CorVRTools: eye, eyebrow and mouth tracking for VRChat in one app.
//!
//! Step 1 scaffold: runs a demo mouth tracker on a test pattern (or a real
//! camera with `--features camera`) and sends the result to VRChat over OSC.
//!
//! Usage:
//!   corvrtools                 run the demo, sending to 127.0.0.1:9000
//!   corvrtools --list-cameras  list cameras (needs the `camera` feature)
//!   corvrtools --camera NAME   use the first camera whose name contains NAME
//!   corvrtools --prefix FT/    parameter prefix used by your avatar

use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use corvr_capture::{FrameSource, TestPattern};
use corvr_core::FaceState;
use corvr_inference::{DemoMouthTracker, Tracker};
use corvr_osc::{OscConfig, OscSender};

struct Args {
    list_cameras: bool,
    camera: Option<String>,
    prefix: String,
    target: String,
}

fn parse_args() -> Result<Args> {
    let mut args = Args {
        list_cameras: false,
        camera: None,
        prefix: String::new(),
        target: corvr_osc::VRCHAT_DEFAULT.to_string(),
    };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--list-cameras" => args.list_cameras = true,
            "--camera" => args.camera = Some(it.next().context("--camera needs a name")?),
            "--prefix" => args.prefix = it.next().context("--prefix needs a value")?,
            "--target" => args.target = it.next().context("--target needs host:port")?,
            "-h" | "--help" => {
                println!("corvrtools [--list-cameras] [--camera NAME] [--prefix FT/] [--target 127.0.0.1:9000]");
                std::process::exit(0);
            }
            other => bail!("unknown argument: {other}"),
        }
    }
    Ok(args)
}

#[cfg(feature = "camera")]
fn list_cameras() -> Result<()> {
    for c in corvr_capture::list_cameras()? {
        println!("[{}] {} ({})", c.index, c.name, c.description);
    }
    Ok(())
}

#[cfg(not(feature = "camera"))]
fn list_cameras() -> Result<()> {
    bail!("built without camera support; rebuild with --features camera")
}

#[cfg(feature = "camera")]
fn open_source(camera: Option<&str>) -> Result<Box<dyn FrameSource>> {
    Ok(match camera {
        Some(name) => Box::new(corvr_capture::CameraSource::open_by_name(name)?),
        None => Box::new(TestPattern::new(240, 240, 60)),
    })
}

#[cfg(not(feature = "camera"))]
fn open_source(camera: Option<&str>) -> Result<Box<dyn FrameSource>> {
    if camera.is_some() {
        bail!("built without camera support; rebuild with --features camera");
    }
    Ok(Box::new(TestPattern::new(240, 240, 60)))
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = parse_args()?;
    if args.list_cameras {
        return list_cameras();
    }

    let mut source = open_source(args.camera.as_deref())?;
    let mut tracker = DemoMouthTracker;
    let sender = OscSender::new(OscConfig {
        target: args.target.parse().context("bad --target")?,
        prefix: args.prefix,
    })?;

    log::info!("sending to {} with {}", args.target, tracker.name());
    let mut state = FaceState::default();
    let frame_time = Duration::from_secs_f64(1.0 / 60.0);
    loop {
        let started = Instant::now();
        let frame = source.next_frame()?;
        tracker.process(&frame, &mut state)?;
        sender.send(&state)?;
        if let Some(rest) = frame_time.checked_sub(started.elapsed()) {
            std::thread::sleep(rest);
        }
    }
}
