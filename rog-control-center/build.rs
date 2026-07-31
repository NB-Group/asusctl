use std::path::PathBuf;

use slint_build::CompilerConfiguration;

/// Compile .po source files to .mo binaries at build time so dev builds
/// have translations without committing binary artifacts to Git.
/// Silently skips if `msgfmt` is not installed (installed packages use
/// /usr/share/locale/ instead).
fn compile_locales() {
    let root = env!("CARGO_MANIFEST_DIR");
    let translations_dir = PathBuf::from(root).join("translations");
    let Ok(entries) = std::fs::read_dir(&translations_dir) else {
        return;
    };
    for entry in entries.flatten() {
        // Sources live at translations/<locale>/rog-control-center.po; the
        // compiled catalog goes to the gettext layout (<locale>/LC_MESSAGES/)
        // that init_translations! resolves at runtime.
        let po = entry.path().join("rog-control-center.po");
        if !po.exists() {
            continue;
        }
        let mo = entry.path().join("LC_MESSAGES/rog-control-center.mo");
        // The LC_MESSAGES dir isn't tracked once the .mo are gitignored, so
        // recreate it on a fresh checkout before msgfmt writes into it.
        let _ = std::fs::create_dir_all(mo.parent().unwrap_or(entry.path().as_path()));
        match std::process::Command::new("msgfmt").arg("-o").arg(&mo).arg(&po).status() {
            Ok(status) if !status.success() => {
                println!("cargo:warning=msgfmt failed for {} (status {status})", po.display());
            }
            // msgfmt not installed (dev without gettext) — packaged builds use
            // /usr/share/locale instead, so silently skip.
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => {
                println!("cargo:warning=msgfmt could not run for {}: {e}", po.display());
            }
            Ok(_) => {}
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
