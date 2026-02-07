pub fn build() -> anyhow::Result<()> {
    println!("[Xtask] Building all...");
    std::process::Command::new("cargo")
        .args(["build", "--release"])
        .status()?;
    Ok(())
}

