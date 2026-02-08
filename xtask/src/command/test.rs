pub fn test() -> anyhow::Result<()> {
    println!("[Xtask] testing all...");
    std::process::Command::new("cargo")
        .args(["test"])
        .current_dir("tests")
        .status()?;
    Ok(())
}
