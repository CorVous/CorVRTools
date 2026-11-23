//! Unified expression shapes.
//!
//! The names and order match VRCFaceTracking's `UnifiedExpressions` enum
//! (Apache-2.0), so avatars built for VRCFT v2 parameters work unchanged.

macro_rules! expressions {
    ($($name:ident,)*) => {
        /// One face shape, weighted 0.0 to 1.0.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(usize)]
        pub enum Expression {
            $($name,)*
        }

        impl Expression {
            /// Every shape, in VRCFT order.
            pub const ALL: &'static [Expression] = &[$(Expression::$name,)*];

            /// Number of shapes.
            pub const COUNT: usize = Self::ALL.len();

            /// The shape's name, as used in the `v2/<name>` avatar parameter.
            pub const fn name(self) -> &'static str {
                match self {
                    $(Expression::$name => stringify!($name),)*
                }
            }
        }
    };
}

expressions! {
    EyeSquintRight,
    EyeSquintLeft,
    EyeWideRight,
    EyeWideLeft,
    BrowPinchRight,
    BrowPinchLeft,
    BrowLowererRight,
    BrowLowererLeft,
    BrowInnerUpRight,
    BrowInnerUpLeft,
    BrowOuterUpRight,
    BrowOuterUpLeft,
    NasalDilationRight,
    NasalDilationLeft,
    NasalConstrictRight,
    NasalConstrictLeft,
    CheekSquintRight,
    CheekSquintLeft,
    CheekPuffRight,
    CheekPuffLeft,
    CheekSuckRight,
    CheekSuckLeft,
    JawOpen,
    JawRight,
    JawLeft,
    JawForward,
    JawBackward,
    JawClench,
    JawMandibleRaise,
    MouthClosed,
    LipSuckUpperRight,
    LipSuckUpperLeft,
    LipSuckLowerRight,
    LipSuckLowerLeft,
    LipSuckCornerRight,
    LipSuckCornerLeft,
    LipFunnelUpperRight,
    LipFunnelUpperLeft,
    LipFunnelLowerRight,
    LipFunnelLowerLeft,
    LipPuckerUpperRight,
    LipPuckerUpperLeft,
    LipPuckerLowerRight,
    LipPuckerLowerLeft,
    MouthUpperUpRight,
    MouthUpperUpLeft,
    MouthUpperDeepenRight,
    MouthUpperDeepenLeft,
    NoseSneerRight,
    NoseSneerLeft,
    MouthLowerDownRight,
    MouthLowerDownLeft,
    MouthUpperRight,
    MouthUpperLeft,
    MouthLowerRight,
    MouthLowerLeft,
    MouthCornerPullRight,
    MouthCornerPullLeft,
    MouthCornerSlantRight,
    MouthCornerSlantLeft,
    MouthFrownRight,
    MouthFrownLeft,
    MouthStretchRight,
    MouthStretchLeft,
    MouthDimpleRight,
    MouthDimpleLeft,
    MouthRaiserUpper,
    MouthRaiserLower,
    MouthPressRight,
    MouthPressLeft,
    MouthTightenerRight,
    MouthTightenerLeft,
    TongueOut,
    TongueUp,
    TongueDown,
    TongueRight,
    TongueLeft,
    TongueRoll,
    TongueBendDown,
    TongueCurlUp,
    TongueSquish,
    TongueFlat,
    TongueTwistRight,
    TongueTwistLeft,
    SoftPalateClose,
    ThroatSwallow,
    NeckFlexRight,
    NeckFlexLeft,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_vrcft_count_and_order() {
        assert_eq!(Expression::COUNT, 88);
        assert_eq!(Expression::ALL[0], Expression::EyeSquintRight);
        assert_eq!(Expression::JawOpen as usize, 22);
        assert_eq!(Expression::NeckFlexLeft.name(), "NeckFlexLeft");
    }
}
