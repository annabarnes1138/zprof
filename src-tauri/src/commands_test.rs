#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_get_prompt_engines_returns_all() {
        let engines = get_prompt_engines().expect("Should get engines");
        assert_eq!(engines.len(), 5, "Should return 5 engines");

        // Verify all engines have required fields
        for engine in &engines {
            assert!(!engine.name.is_empty(), "Engine name should not be empty");
            assert!(!engine.description.is_empty(), "Engine description should not be empty");
        }
    }

    #[test]
    fn test_get_prompt_engines_includes_metadata() {
        let engines = get_prompt_engines().expect("Should get engines");

        // Find Starship engine
        let starship = engines.iter().find(|e| e.name == "Starship");
        assert!(starship.is_some(), "Should include Starship");

        let starship = starship.unwrap();
        assert!(starship.cross_shell, "Starship should be cross-shell");
        assert!(starship.async_rendering, "Starship should support async rendering");
        assert!(starship.nerd_font_required, "Starship should require Nerd Fonts");
    }

    #[test]
    fn test_check_engine_installed_with_invalid_engine() {
        let result = check_engine_installed("InvalidEngine".to_string());
        assert!(result.is_err(), "Should fail with invalid engine name");
    }

    #[test]
    fn test_get_frameworks_returns_all() {
        let frameworks = get_frameworks().expect("Should get frameworks");
        assert_eq!(frameworks.len(), 5, "Should return 5 frameworks");

        // Verify all have descriptions
        for fw in &frameworks {
            assert!(!fw.name.is_empty());
            assert!(!fw.description.is_empty());
        }
    }
}
