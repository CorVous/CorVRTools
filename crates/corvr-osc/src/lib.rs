//! Sends [`FaceState`] to VRChat over OSC.
//!
//! Parameter names follow VRCFaceTracking's v2 scheme (`v2/JawOpen`,
//! `v2/EyeLeftX`, ...), so avatars set up for VRCFT work without changes.
//! Only float parameters are sent for now; binary-encoded parameters come
//! later with avatar config (OSCQuery) support.

use std::net::{SocketAddr, UdpSocket};

use corvr_core::{Eye, FaceState};
use rosc::{encoder, OscBundle, OscMessage, OscPacket, OscTime, OscType};

/// VRChat's default OSC input address.
pub const VRCHAT_DEFAULT: &str = "127.0.0.1:9000";

#[derive(Debug, thiserror::Error)]
pub enum OscError {
    #[error("network error: {0}")]
    Io(#[from] std::io::Error),
    #[error("encode error: {0}")]
    Encode(#[from] rosc::OscError),
}

/// Settings for the OSC output.
#[derive(Debug, Clone)]
pub struct OscConfig {
    pub target: SocketAddr,
    /// Parameter prefix inside `/avatar/parameters/`. VRCFT avatars use
    /// either `""` or `"FT/"`.
    pub prefix: String,
}

impl Default for OscConfig {
    fn default() -> Self {
        Self {
            target: VRCHAT_DEFAULT.parse().expect("valid address"),
            prefix: String::new(),
        }
    }
}

/// Builds the OSC messages for one face state.
pub fn build_messages(state: &FaceState, prefix: &str) -> Vec<OscMessage> {
    let mut out = Vec::with_capacity(128);
    let mut float = |name: &str, v: f32| {
        out.push(OscMessage {
            addr: format!("/avatar/parameters/{prefix}v2/{name}"),
            args: vec![OscType::Float(v)],
        });
    };

    if state.eye_tracking_active {
        let both = state.combined_eye();
        for (label, eye) in [
            ("Eye", both),
            ("EyeLeft", state.left),
            ("EyeRight", state.right),
        ] {
            float(&format!("{label}X"), eye.gaze.0);
            float(&format!("{label}Y"), eye.gaze.1);
        }
        eye_openness(&mut float, "Left", state.left);
        eye_openness(&mut float, "Right", state.right);
        float("EyeOpen", both.openness);
        float("EyeClosed", 1.0 - both.openness);
    }

    for (e, w) in state.shapes() {
        float(e.name(), w);
    }

    out.push(OscMessage {
        addr: "/avatar/parameters/EyeTrackingActive".into(),
        args: vec![OscType::Bool(state.eye_tracking_active)],
    });
    out.push(OscMessage {
        addr: "/avatar/parameters/LipTrackingActive".into(),
        args: vec![OscType::Bool(state.lip_tracking_active)],
    });
    out
}

fn eye_openness(float: &mut impl FnMut(&str, f32), side: &str, eye: Eye) {
    float(&format!("EyeOpen{side}"), eye.openness);
    float(&format!("EyeClosed{side}"), 1.0 - eye.openness);
}

/// A UDP sender for VRChat.
pub struct OscSender {
    socket: UdpSocket,
    config: OscConfig,
}

impl OscSender {
    pub fn new(config: OscConfig) -> Result<Self, OscError> {
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        Ok(Self { socket, config })
    }

    /// Sends the whole face state. Messages are split into bundles that stay
    /// under a safe UDP size.
    pub fn send(&self, state: &FaceState) -> Result<(), OscError> {
        const MAX_BYTES: usize = 1400;
        let mut bundle = Vec::new();
        let mut size = 0;
        for msg in build_messages(state, &self.config.prefix) {
            let len = encoder::encode(&OscPacket::Message(msg.clone()))?.len() + 4;
            if size + len > MAX_BYTES && !bundle.is_empty() {
                self.send_bundle(std::mem::take(&mut bundle))?;
                size = 0;
            }
            size += len;
            bundle.push(OscPacket::Message(msg));
        }
        if !bundle.is_empty() {
            self.send_bundle(bundle)?;
        }
        Ok(())
    }

    fn send_bundle(&self, content: Vec<OscPacket>) -> Result<(), OscError> {
        let packet = OscPacket::Bundle(OscBundle {
            timetag: OscTime {
                seconds: 0,
                fractional: 1,
            },
            content,
        });
        self.socket
            .send_to(&encoder::encode(&packet)?, self.config.target)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use corvr_core::Expression;

    fn find<'a>(msgs: &'a [OscMessage], addr: &str) -> Option<&'a OscType> {
        msgs.iter().find(|m| m.addr == addr).map(|m| &m.args[0])
    }

    #[test]
    fn names_follow_vrcft_v2() {
        let mut s = FaceState::default();
        s.set(Expression::JawOpen, 0.25);
        s.eye_tracking_active = true;
        s.left.gaze = (0.5, -0.5);
        let msgs = build_messages(&s, "FT/");
        assert_eq!(
            find(&msgs, "/avatar/parameters/FT/v2/JawOpen"),
            Some(&OscType::Float(0.25))
        );
        assert_eq!(
            find(&msgs, "/avatar/parameters/FT/v2/EyeLeftX"),
            Some(&OscType::Float(0.5))
        );
        assert_eq!(
            find(&msgs, "/avatar/parameters/FT/v2/EyeClosedRight"),
            Some(&OscType::Float(0.0))
        );
    }

    #[test]
    fn skips_eye_params_when_eyes_inactive() {
        let msgs = build_messages(&FaceState::default(), "");
        assert!(find(&msgs, "/avatar/parameters/v2/EyeLeftX").is_none());
        assert_eq!(
            find(&msgs, "/avatar/parameters/EyeTrackingActive"),
            Some(&OscType::Bool(false))
        );
    }

    #[test]
    fn sends_bundles_that_decode() {
        let rx = UdpSocket::bind("127.0.0.1:0").unwrap();
        rx.set_read_timeout(Some(std::time::Duration::from_secs(2)))
            .unwrap();
        let sender = OscSender::new(OscConfig {
            target: rx.local_addr().unwrap(),
            prefix: String::new(),
        })
        .unwrap();
        let mut s = FaceState::default();
        s.set(Expression::JawOpen, 1.0);
        sender.send(&s).unwrap();

        let mut buf = [0u8; 2048];
        let mut seen = 0;
        let expected = build_messages(&s, "").len();
        while seen < expected {
            let n = rx.recv(&mut buf).unwrap();
            assert!(n <= 1400 + 16);
            match rosc::decoder::decode_udp(&buf[..n]).unwrap().1 {
                OscPacket::Bundle(b) => seen += b.content.len(),
                OscPacket::Message(_) => panic!("expected bundle"),
            }
        }
        assert_eq!(seen, expected);
    }
}
