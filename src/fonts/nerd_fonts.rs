//! Nerd Font registry and data models
//!
//! Provides a curated collection of Nerd Fonts for modern prompt engines that
//! require glyphs and icons. Each font includes complete metadata, download URLs,
//! and recommendations for specific prompt engines.

use crate::prompts::engine::PromptEngine;

/// Font file format type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FontFormat {
    /// TrueType font (.ttf)
    TrueType,
    /// OpenType font (.otf)
    OpenType,
}

/// Nerd Font metadata and download information
///
/// Each font in the registry includes all necessary information for
/// detection, download, installation, and user-facing display.
#[derive(Debug, Clone)]
pub struct NerdFont {
    /// Static identifier (e.g., "firacode")
    pub id: &'static str,
    /// Full font name (e.g., "FiraCode Nerd Font")
    pub name: &'static str,
    /// Display variant name (e.g., "FiraCode Nerd Font Mono")
    pub display_name: &'static str,
    /// Human-readable description of font characteristics
    pub description: &'static str,
    /// Preview characters showcasing glyphs (e.g., "⚡ ⬢  →  ✓  ")
    pub preview_chars: &'static str,
    /// GitHub release download URL
    pub download_url: &'static str,
    /// Font file format
    pub file_format: FontFormat,
    /// Whether this font is recommended for general use
    pub recommended: bool,
    /// Prompt engines this font is specifically recommended for
    pub recommended_for: &'static [PromptEngine],
}

/// Registry of curated Nerd Fonts
///
/// All fonts are from the official Nerd Fonts GitHub releases (v3.1.1).
/// URLs point to the latest stable release with complete glyph coverage.
pub static NERD_FONTS: &[NerdFont] = &[
    NerdFont {
        id: "firacode",
        name: "FiraCode Nerd Font",
        display_name: "FiraCode Nerd Font Mono",
        description: "Programming ligatures, clean and modern design, excellent for code",
        preview_chars: "⚡ ⬢  →  ✓   λ ≡",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/FiraCode.zip",
        file_format: FontFormat::TrueType,
        recommended: true,
        recommended_for: &[
            PromptEngine::Starship,
            PromptEngine::OhMyPosh,
            PromptEngine::Spaceship,
        ],
    },
    NerdFont {
        id: "jetbrainsmono",
        name: "JetBrainsMono Nerd Font",
        display_name: "JetBrainsMono Nerd Font Mono",
        description: "Designed for developers, increased letter height, excellent readability",
        preview_chars: "⚙  ⎇  ⬡  ✔   ∞ ⌘",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/JetBrainsMono.zip",
        file_format: FontFormat::TrueType,
        recommended: true,
        recommended_for: &[
            PromptEngine::Powerlevel10k,
            PromptEngine::Starship,
            PromptEngine::OhMyPosh,
        ],
    },
    NerdFont {
        id: "meslo",
        name: "Meslo Nerd Font",
        display_name: "MesloLGS Nerd Font",
        description: "Customized Menlo with added line spacing, recommended by Powerlevel10k",
        preview_chars: "➜  ✓  ✗    ",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/Meslo.zip",
        file_format: FontFormat::TrueType,
        recommended: true,
        recommended_for: &[PromptEngine::Powerlevel10k],
    },
    NerdFont {
        id: "hack",
        name: "Hack Nerd Font",
        display_name: "Hack Nerd Font Mono",
        description: "Designed for source code, clear at small sizes, classic look",
        preview_chars: "⚡ ⎇    ⟳  ✔ ∴",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/Hack.zip",
        file_format: FontFormat::TrueType,
        recommended: false,
        recommended_for: &[PromptEngine::Starship, PromptEngine::OhMyPosh],
    },
    NerdFont {
        id: "cascadiacode",
        name: "CascadiaCode Nerd Font",
        display_name: "CaskaydiaCove Nerd Font Mono",
        description: "Microsoft's modern monospace, includes programming ligatures",
        preview_chars: "⚡   ⬢  ⎇  ✓  ≡ ∞",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/CascadiaCode.zip",
        file_format: FontFormat::TrueType,
        recommended: false,
        recommended_for: &[PromptEngine::Starship, PromptEngine::OhMyPosh],
    },
    NerdFont {
        id: "ubuntumono",
        name: "UbuntuMono Nerd Font",
        display_name: "UbuntuMono Nerd Font",
        description: "Ubuntu's monospace font, clean and readable, familiar to Linux users",
        preview_chars: "⚡ →  ⬢  ✓  ⎇  ≡",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/UbuntuMono.zip",
        file_format: FontFormat::TrueType,
        recommended: false,
        recommended_for: &[PromptEngine::Starship, PromptEngine::Spaceship],
    },
];

/// Get all fonts in the registry
pub fn get_all_fonts() -> &'static [NerdFont] {
    NERD_FONTS
}

/// Get only recommended fonts from the registry
pub fn get_recommended_fonts() -> Vec<&'static NerdFont> {
    NERD_FONTS.iter().filter(|f| f.recommended).collect()
}

/// Get a font by its ID
///
/// # Arguments
/// * `id` - The font identifier (e.g., "firacode", "jetbrainsmono")
///
/// # Returns
/// The font if found, None otherwise
pub fn get_font_by_id(id: &str) -> Option<&'static NerdFont> {
    NERD_FONTS.iter().find(|f| f.id == id)
}

/// Get fonts recommended for a specific prompt engine
///
/// # Arguments
/// * `engine` - The prompt engine to get recommendations for
///
/// # Returns
/// Vector of fonts recommended for the given engine
pub fn get_fonts_for_engine(engine: &PromptEngine) -> Vec<&'static NerdFont> {
    NERD_FONTS
        .iter()
        .filter(|f| f.recommended_for.contains(engine))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_has_six_fonts() {
        assert_eq!(NERD_FONTS.len(), 6);
    }

    #[test]
    fn test_get_all_fonts() {
        let fonts = get_all_fonts();
        assert_eq!(fonts.len(), 6);
    }

    #[test]
    fn test_get_recommended_fonts() {
        let recommended = get_recommended_fonts();
        assert_eq!(recommended.len(), 3);
        assert!(recommended.iter().all(|f| f.recommended));

        // Verify specific fonts are recommended
        let ids: Vec<&str> = recommended.iter().map(|f| f.id).collect();
        assert!(ids.contains(&"firacode"));
        assert!(ids.contains(&"jetbrainsmono"));
        assert!(ids.contains(&"meslo"));
    }

    #[test]
    fn test_get_font_by_id() {
        // Test valid IDs
        let fira = get_font_by_id("firacode");
        assert!(fira.is_some());
        assert_eq!(fira.unwrap().name, "FiraCode Nerd Font");

        let jetbrains = get_font_by_id("jetbrainsmono");
        assert!(jetbrains.is_some());
        assert_eq!(jetbrains.unwrap().name, "JetBrainsMono Nerd Font");

        // Test invalid ID
        let invalid = get_font_by_id("nonexistent");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_get_fonts_for_engine() {
        // Starship should have multiple recommendations
        let starship_fonts = get_fonts_for_engine(&PromptEngine::Starship);
        assert!(!starship_fonts.is_empty());
        assert!(starship_fonts.iter().any(|f| f.id == "firacode"));

        // Powerlevel10k should have specific recommendations including Meslo
        let p10k_fonts = get_fonts_for_engine(&PromptEngine::Powerlevel10k);
        assert!(!p10k_fonts.is_empty());
        assert!(p10k_fonts.iter().any(|f| f.id == "meslo"));

        // Pure doesn't require Nerd Fonts, so no specific recommendations
        let pure_fonts = get_fonts_for_engine(&PromptEngine::Pure);
        assert_eq!(pure_fonts.len(), 0);
    }

    #[test]
    fn test_all_fonts_have_valid_metadata() {
        for font in NERD_FONTS.iter() {
            // All fields should be non-empty
            assert!(!font.id.is_empty(), "Font ID should not be empty");
            assert!(!font.name.is_empty(), "Font name should not be empty");
            assert!(
                !font.display_name.is_empty(),
                "Font display name should not be empty"
            );
            assert!(
                !font.description.is_empty(),
                "Font description should not be empty"
            );
            assert!(
                !font.preview_chars.is_empty(),
                "Font preview chars should not be empty"
            );
            assert!(
                !font.download_url.is_empty(),
                "Font download URL should not be empty"
            );

            // URL should be valid GitHub release URL
            assert!(
                font.download_url
                    .starts_with("https://github.com/ryanoasis/nerd-fonts/releases/"),
                "Font download URL should be from official Nerd Fonts releases: {}",
                font.download_url
            );

            // URL should include version
            assert!(
                font.download_url.contains("/v3.1.1/"),
                "Font download URL should use version v3.1.1: {}",
                font.download_url
            );

            // All fonts in registry should be TrueType
            assert_eq!(
                font.file_format,
                FontFormat::TrueType,
                "All registry fonts should be TrueType"
            );
        }
    }

    #[test]
    fn test_font_ids_are_unique() {
        let mut ids = std::collections::HashSet::new();
        for font in NERD_FONTS.iter() {
            assert!(
                ids.insert(font.id),
                "Font ID '{}' appears more than once",
                font.id
            );
        }
    }

    #[test]
    fn test_recommended_fonts_have_recommendations() {
        for font in NERD_FONTS.iter() {
            if font.recommended {
                assert!(
                    !font.recommended_for.is_empty(),
                    "Recommended font '{}' should have at least one engine recommendation",
                    font.id
                );
            }
        }
    }

    #[test]
    fn test_all_engines_with_nerd_font_requirement_have_recommendations() {
        let engines_requiring_fonts = vec![
            PromptEngine::Starship,
            PromptEngine::Powerlevel10k,
            PromptEngine::OhMyPosh,
            PromptEngine::Spaceship,
        ];

        for engine in engines_requiring_fonts {
            let fonts = get_fonts_for_engine(&engine);
            assert!(
                !fonts.is_empty(),
                "Engine {:?} requires Nerd Fonts but has no recommendations",
                engine
            );
        }
    }
}
