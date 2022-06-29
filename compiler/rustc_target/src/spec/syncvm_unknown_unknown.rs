use crate::spec::{Target, TargetOptions, Endian};

pub fn target() -> Target {
    let base = TargetOptions {
        cpu: "unknown".into(),
        os: "unknown".into(),
        env: "".into(),
        max_atomic_width: Some(256),
        endian: Endian::Big,
        ..Default::default()
    };

    Target {
        llvm_target: "syncvm-unknown-unknown".into(),
        pointer_width: 256,
        data_layout: "E-p:256:256-i8:256:256:256-i256:256:256-S32-a:256:256"
            .into(),
        arch: "syncvm".into(),
        options: base,
    }
}
