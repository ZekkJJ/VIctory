use std::path::PathBuf;
use std::process::Command;

/// A file that is no PE image, outside the source tree: the asset policy
/// refuses inputs inside it.
fn not_an_image(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("victory-census-{test}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("Cargo.toml");
    std::fs::write(&file, "[package]\n").unwrap();
    file
}

#[test]
fn census_fails_when_a_requested_path_is_missing() {
    let valid = not_an_image("missing");
    let missing = valid.with_extension("missing-census-input");
    assert!(!missing.exists());

    let output = Command::new(env!("CARGO_BIN_EXE_victory"))
        .arg("census")
        .args([&valid, &missing])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "census succeeded: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(stderr.contains("Cargo.missing-census-input"), "{stderr}");
}

#[test]
fn census_accepts_a_valid_path_with_no_pe_images() {
    let valid = not_an_image("valid");
    let output = Command::new(env!("CARGO_BIN_EXE_victory"))
        .arg("census")
        .arg(valid)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("0 images scanned, 0 unreadable"));
}
