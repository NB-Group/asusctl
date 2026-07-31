use std::path::PathBuf;

use slint_build::CompilerConfiguration;

/// Compile .po source files to .mo binaries at build time so dev builds
/// have translations without committing binary artifacts to Git.
/// Silently skips if `msgfmt` is not installed (installed packages use
/// /usr/share/locale/ instead).
fn compile_locales() {
    let root = env!("CARGO_MANIFEST_DIR");
    let translations_dir = PathBuf::from(root).join("translations");
    if let Ok(entries) = std::fs::read_dir(&translations_dir) {
        for entry in entries.flatten() {
            let po = entry.path().join("LC_MESSAGES/rog-control-center.po");
            if po.exists() {
                let mo = entry.path().join("LC_MESSAGES/rog-control-center.mo");
                let _ = std::process::Command::new("msgfmt")
                    .arg("-o")
                    .arg(&mo)
                    .arg(&po)
                    .status();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_locales();
    let root = env!("CARGO_MANIFEST_DIR");
    let mut main = PathBuf::from(root);
    main.push("ui/main_window.slint");

    let mut include = PathBuf::from(root);
    include.push("ui");

    slint_build::print_rustc_flags()?;
    slint_build::compile_with_config(
        main,
        CompilerConfiguration::new()
            // .embed_resources(EmbedResourcesKind::EmbedFiles)
            .with_include_paths(vec![include])
            .with_style("fluent".into()),
    )?;
    Ok(())
}
