# CorVRTools

Eye, eyebrow and mouth tracking for VRChat in one Rust app, built for a
Bigscreen Beyond 2e (eye cameras) and a Project Babble mouth tracker. It sends
VRCFaceTracking v2 parameters straight to VRChat over OSC, so VRCFT avatars
work without VRCFT running.

Status: step 1 scaffold. A demo tracker opens and closes the jaw so the OSC
path can be checked in VRChat. Real tracking comes in later steps.

## Layout

| Crate | What it does |
| --- | --- |
| `corvr-core` | Shared types: `Frame` (grayscale image), `FaceState`, and the 88 VRCFT `Expression` shapes |
| `corvr-capture` | `FrameSource` trait, a `TestPattern` source, and real cameras through nokhwa (`camera` feature) |
| `corvr-inference` | `Tracker` trait, a demo mouth tracker, and ONNX model loading through `ort` (`onnx` feature) |
| `corvr-osc` | Builds VRCFT v2 OSC messages (`/avatar/parameters/[FT/]v2/...`) and sends them over UDP |
| `corvr-app` | The `corvrtools` binary |

## Build and run (Windows)

```powershell
cargo test
cargo run --release --features camera -- --list-cameras
cargo run --release --features camera -- --camera "Babble" --prefix FT/
```

- With no `--camera`, the app uses a test pattern.
- `--prefix` should match your avatar: `FT/` for most VRCFT v2 avatars, or empty.
- `--target` defaults to VRChat's OSC port, `127.0.0.1:9000`. Turn on OSC in VRChat's action menu.
- The `onnx` feature downloads ONNX Runtime during the first build.

## Models

Model files (`*.onnx`) are not stored in this repo. The plan is to use the
current Project Babble mouth model and the EyeTrackVR gaze (`NEXT_BSB`) and
eyebrow models. These are under the Babble Software Distribution License 1.0
(non-commercial, copyleft), so any release that includes them or code ported
from those projects must use that license too. See
[docs/research.md](docs/research.md).

## Plan

1. Groundwork: this scaffold, plus camera findings on real hardware.
2. Mouth tracking from the Babble camera.
3. Eyes and eyebrows from the Beyond 2e cameras, following EyeTrackVR.
4. One polished app: UI, calibration, smoothing, profiles, release build.
