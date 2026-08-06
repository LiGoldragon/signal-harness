use std::process::Command;

#[test]
fn runtime_tree_is_one_current_signal_family_without_nota() {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "normal", "--all-features"])
        .output()
        .expect("run cargo tree");
    assert!(output.status.success());
    let tree = String::from_utf8(output.stdout).expect("dependency tree");
    assert!(!tree.contains("nota"));
    assert_eq!(tree.matches("signal-frame v0.3.0").count(), 2);
    assert!(
        tree.contains("signal-frame.git?rev=01676293a623d97b65e320d4138c4b480c6d5dad#01676293")
    );
    assert!(!tree.contains("0786fbe8"));
    assert!(
        tree.contains("signal-persona.git?rev=7d2568d420869aa0cded49c3c04cc0ac180e66a2#7d2568d4")
    );
}

#[test]
fn current_persona_coordinates_are_direct_and_unaliased() {
    let source = include_str!("../src/lib.rs");
    for encoded in ["z2Veez", "z2VNyf", "z2VckR", "z2VSSX", "z2VRBs", "z2VUtF"] {
        assert!(
            source.contains(encoded),
            "missing strict coordinate {encoded}"
        );
    }
    for retired in [
        "DomainSocketPath",
        "DomainSocketMode",
        "EngineManagementSocketPath",
        "EngineManagementSocketMode",
        "OwnerIdentity",
        "TimestampNanos",
    ] {
        assert!(
            !source.contains(retired),
            "readable alias {retired} survived"
        );
    }
}

#[test]
fn historical_sources_features_and_moving_pins_are_absent() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("schema").exists());
    assert!(root.join("examples/canonical.dotos").is_file());
    assert!(!root.join("examples/canonical.nota").exists());

    for source in [include_str!("../Cargo.toml"), include_str!("../src/lib.rs")] {
        for forbidden in ["nota", ".schema", "branch ="] {
            assert!(!source.to_ascii_lowercase().contains(forbidden));
        }
    }
}
