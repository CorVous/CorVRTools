# Step 1 research notes (2026-11-23)

## Licenses

| Project | Current license | Older permissive versions |
| --- | --- | --- |
| Project Babble | Babble Software Distribution License 1.0 since 2025-01-14 | Apache-2.0 up to Dec 2024 |
| EyeTrackVR | Babble Software Distribution License 1.0 since 2025-02-18 | MIT until 2024-02-20, then dual license, then GPL-3.0 from 2024-04-14 |
| VRCFaceTracking | Apache-2.0 | n/a |

The Babble license is Apache-style with extra terms in section 10:
non-commercial use only, no use in paid hardware, and copyleft (derivative
works must use the same license and publish source for free).

Decision: use the current, best models under that license. Keep the code
unpublished for now.

## Models in the upstream repos

- Babble mouth: current `EFFB0E11BS128V7.5` (added 2025, BSDL). Last Apache
  model: `3MEFFB0E7MSE`.
- EyeTrackVR eyes: `NEXT_*` gaze models, including `NEXT_BSB` for the
  Bigscreen Beyond (added 2026-11-14), plus `NEXT_LARGE_BSB` and `NEXT_XL_BSB`.
- EyeTrackVR eyebrows: `Eyebrow_ETVR.onnx` and `eyetrackapp/eyebrow.py`
  (added 2026-10-31).
- MIT-era EyeTrackVR (before 2024-02) only had classical pupil algorithms
  (HSRAC, RANSAC, blob) and the early LEAP landmark model, with no eyebrows.

## VRChat output

VRCFaceTracking sends each unified shape as `v2/<Name>` (88 shapes, order in
`corvr-core`), gaze as `v2/EyeX`, `v2/EyeY`, `v2/EyeLeftX`, ..., openness as
`v2/EyeOpenLeft` and `v2/EyeClosedLeft`, plus `EyeTrackingActive` and
`LipTrackingActive`. Avatars use either no prefix or `FT/`.

## Still to find out on the PC

- How the Beyond 2e eye cameras appear in Windows (UVC device, or only
  through Bigscreen's software) and their resolution and frame rate.
- How the Babble camera appears (USB UVC or a network stream).

Run `corvrtools --list-cameras` (with `--features camera`) to check.
