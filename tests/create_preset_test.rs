use anyhow::Result;
use std::env;
use tempfile::TempDir;
use zprof::cli::create_from_preset::create_from_preset;
use zprof::core::manifest::Manifest;
use zprof::presets::PRESET_REGISTRY;

// Helper to run test with isolated HOME
fn run_with_isolated_home<F>(test_fn: F) -> Result<()>
where
    F: FnOnce(&std::path::Path) -> Result<()>,
{
    // Create temp directory for home
    let temp_dir = TempDir::new()?;
    let home_path = temp_dir.path().to_path_buf();
    
    // Save original HOME
    let original_home = env::var("HOME").ok();
    
    // Set HOME to temp dir
    // SAFETY: This is not thread-safe. Tests using this must be run sequentially.
    // In a real CI environment, we'd use rusty-fork or similar.
    unsafe {
        env::set_var("HOME", &home_path);
        env::set_var("ZPROF_TEST_MODE", "1");
    }
    
    // Run the test
    let result = test_fn(&home_path);
    
    // Restore original HOME
    unsafe {
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }
        env::remove_var("ZPROF_TEST_MODE");
    }
    
    result
}

#[test]
fn test_manifest_from_preset_logic() {
    let minimal_preset = &PRESET_REGISTRY[0];
    let manifest = Manifest::from_preset("test-profile", minimal_preset);
    
    assert_eq!(manifest.profile.name, "test-profile");
    assert_eq!(manifest.profile.framework, "zap");
    assert_eq!(manifest.plugins.enabled.len(), 3);
    assert!(manifest.plugins.enabled.contains(&"git".to_string()));
}

#[test]
fn test_create_from_preset_integration() -> Result<()> {
    run_with_isolated_home(|home_path| {
        // 1. Select the "Minimal" preset which uses Pure prompt
        // We need to find the Minimal preset specifically
        let preset = PRESET_REGISTRY.iter()
            .find(|p| p.name == "Minimal")
            .expect("Minimal preset not found");
            
        // 2. Run create_from_preset
        create_from_preset("test-minimal", preset, false)?;
        
        // 3. Verify directory structure
        let zprof_dir = home_path.join(".zsh-profiles");
        let profile_dir = zprof_dir.join("profiles").join("test-minimal");
        
        assert!(profile_dir.exists(), "Profile directory should exist");
        assert!(profile_dir.join("profile.toml").exists(), "Manifest should exist");
        assert!(profile_dir.join(".zshrc").exists(), ".zshrc should exist");
        
        // 4. Verify framework installation (Zap)
        // Note: install_framework for Zap clones the repo. 
        // In this test env without network mocking, it might fail or succeed depending on network.
        // However, our implementation of install_framework might be doing real clones.
        // If it fails, the test fails.
        // But wait, the previous tests in installer.rs were ignored because of network.
        // If this test runs in CI without network, it will fail.
        // But the requirement was to add integration tests.
        // Let's check if the directory exists at least.
        assert!(profile_dir.join(".zap").exists(), "Zap framework directory should be created");
        
        // 5. Verify prompt engine installation (Pure)
        // Our implementation for Pure clones the repo.
        assert!(profile_dir.join(".pure").exists(), "Pure prompt directory should be created");
        
        Ok(())
    })
}

#[test]
fn test_create_from_preset_performance_starship() -> Result<()> {
    run_with_isolated_home(|home_path| {
        // 1. Select the "Performance" preset which uses Starship
        let preset = PRESET_REGISTRY.iter()
            .find(|p| p.name == "Performance")
            .expect("Performance preset not found");
            
        // 2. Run create_from_preset
        create_from_preset("test-perf", preset, false)?;
        
        // 3. Verify directory structure
        let zprof_dir = home_path.join(".zsh-profiles");
        let profile_dir = zprof_dir.join("profiles").join("test-perf");
        
        assert!(profile_dir.exists());
        
        // 4. Verify Starship placeholder
        // Our implementation for Starship creates a .config directory
        assert!(profile_dir.join(".config").exists(), "Starship config directory should be created");
        
        Ok(())
    })
}
