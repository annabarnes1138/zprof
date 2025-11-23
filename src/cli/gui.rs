use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct GuiArgs {
    // Future: Add GUI-specific options (e.g., --profile <name> to open specific profile)
}

#[cfg(feature = "gui")]
pub fn execute(_args: GuiArgs) -> Result<()> {
    use anyhow::bail;

    // NOTE: This is an informational placeholder command for Epic 0.5 (CLI Compatibility)
    // The GUI is a separate Tauri application (src-tauri) that can be launched independently
    // Full CLI→GUI launcher integration is planned for a future epic
    //
    // This command serves to:
    // 1. Inform users that a GUI exists
    // 2. Provide instructions on how to access it
    // 3. Maintain CLI compatibility while GUI is being developed

    println!("🖥️  zprof GUI Information");
    println!();
    println!("The zprof GUI is a separate Tauri application that provides visual");
    println!("profile management with theme previews and guided workflows.");
    println!();
    println!("📍 How to launch the GUI:");
    println!();
    println!("  Development:   cargo tauri dev");
    println!("  Production:    Run the installed zprof GUI app");
    println!();
    println!("🔨 How to build the GUI:");
    println!();
    println!("  macOS:        cargo tauri build  (produces .app + DMG)");
    println!("  Linux:        cargo tauri build  (produces .deb + AppImage)");
    println!();
    println!("ℹ️  Note: Direct GUI launch from this CLI is planned for a future release.");
    println!("    For now, please use the methods above to access the GUI.");
    println!();

    bail!(
        "GUI launcher not yet implemented (informational command only).\n\
        Use 'cargo tauri dev' or run the installed GUI application."
    )
}

#[cfg(not(feature = "gui"))]
pub fn execute(_args: GuiArgs) -> Result<()> {
    use anyhow::bail;

    eprintln!("❌ Error: GUI not available in this build");
    eprintln!();
    eprintln!("This zprof binary was compiled without GUI support.");
    eprintln!("To use the GUI, install the full version or build with:");
    eprintln!();
    eprintln!("  cargo build --release");
    eprintln!();

    bail!("GUI feature not enabled")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gui_args_parse() {
        // Ensure GuiArgs can be instantiated
        let _args = GuiArgs {};
    }

    #[test]
    #[cfg(feature = "gui")]
    fn test_gui_execute_with_feature() {
        let args = GuiArgs {};
        let result = execute(args);

        // Should fail with informative message (not yet fully implemented)
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("not yet implemented") || err_msg.contains("GUI"));
    }

    #[test]
    #[cfg(not(feature = "gui"))]
    fn test_gui_execute_without_feature() {
        let args = GuiArgs {};
        let result = execute(args);

        // Should fail with "not available" message
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("not enabled") || err_msg.contains("not available"));
    }
}
