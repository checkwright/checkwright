// spec: doctrine-kit/SPEC.md §Knob defaults — doctrine-kit's static knob table
use super::{Default, Kit, Row, Shape};

pub const KIT: Kit = Kit {
    root: "doctrine-kit",
    rows: &[
        Row {
            name: "DOCTRINE_KIT_AGENT_FILE",
            shape: Shape::Scalar,
            default: Default::Scalar("CLAUDE.md"),
            inputs: &[],
            empty_takes_default: false,
        },
        Row {
            name: "DOCTRINE_KIT_DOCTRINE_FILE",
            shape: Shape::Scalar,
            default: Default::Scalar("doctrine-kit/DOCTRINE.md"),
            inputs: &[],
            empty_takes_default: false,
        },
        Row {
            name: "DOCTRINE_KIT_DIGEST_SECTION",
            shape: Shape::Scalar,
            default: Default::Scalar("## Delivery doctrine"),
            inputs: &[],
            empty_takes_default: false,
        },
    ],
    validate: None,
    open_family: false,
    families: &[],
    retired: &[],
    env_only: &[],
};
