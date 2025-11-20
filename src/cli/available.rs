use anyhow::Result;
use clap::{Args, Subcommand};

use crate::frameworks::{plugin, theme, FrameworkType};

/// List available frameworks, plugins, and themes
#[derive(Debug, Args)]
pub struct AvailableArgs {
    #[command(subcommand)]
    pub resource: AvailableResource,
}

#[derive(Debug, Subcommand)]
pub enum AvailableResource {
    /// List all supported frameworks
    Frameworks,
    /// List all available plugins (optionally filtered by framework)
    Plugins {
        /// Framework to filter plugins by
        #[arg(short, long)]
        framework: Option<String>,
    },
    /// List all available themes (optionally filtered by framework)
    Themes {
        /// Framework to filter themes by
        #[arg(short, long)]
        framework: Option<String>,
    },
}

pub fn execute(args: AvailableArgs) -> Result<()> {
    match args.resource {
        AvailableResource::Frameworks => list_frameworks(),
        AvailableResource::Plugins { framework } => list_plugins(framework),
        AvailableResource::Themes { framework } => list_themes(framework),
    }
}

fn list_frameworks() -> Result<()> {
    println!("\nSupported Frameworks:\n");

    let frameworks = vec![
        ("oh-my-zsh", "Popular community-driven framework with extensive plugins"),
        ("zimfw", "Fast and modular Zsh framework"),
        ("prezto", "Configuration framework with sensible defaults"),
        ("zinit", "Flexible plugin manager with turbo mode"),
        ("zap", "Minimal plugin manager focused on speed"),
    ];

    for (name, description) in frameworks {
        println!("  {} - {}", name, description);
    }
    println!();

    Ok(())
}

fn list_plugins(framework_filter: Option<String>) -> Result<()> {
    if let Some(fw) = &framework_filter {
        println!("\nAvailable Plugins for {}:\n", fw);
        let framework_type = parse_framework(fw)?;
        let plugins = plugin::get_plugins_for_framework(&framework_type);

        if plugins.is_empty() {
            println!("  No plugins available for this framework.");
        } else {
            // Separate recommended and other plugins
            let mut recommended = vec![];
            let mut others = vec![];

            for plugin in plugins {
                if plugin.compatibility.is_recommended_for(&framework_type) {
                    recommended.push(plugin);
                } else {
                    others.push(plugin);
                }
            }

            if !recommended.is_empty() {
                println!("Recommended:");
                for plugin in recommended {
                    println!("  ✓ {} - {}", plugin.name, plugin.description);
                }
                println!();
            }

            if !others.is_empty() {
                println!("Available:");
                for plugin in others {
                    println!("    {} - {}", plugin.name, plugin.description);
                }
            }
        }
    } else {
        println!("\nAll Available Plugins:\n");

        // Group plugins by category
        use std::collections::HashMap;
        let mut by_category: HashMap<String, Vec<&crate::frameworks::Plugin>> = HashMap::new();

        for plugin in plugin::PLUGIN_REGISTRY.iter() {
            let category = format!("{:?}", plugin.category);
            by_category.entry(category).or_insert_with(Vec::new).push(plugin);
        }

        // Sort categories
        let mut categories: Vec<_> = by_category.keys().collect();
        categories.sort();

        for category in categories {
            println!("{}:", category);
            let mut plugins = by_category[category].clone();
            plugins.sort_by_key(|p| p.name);

            for plugin in plugins {
                // Show which frameworks support this plugin
                let frameworks: Vec<String> = plugin.compatibility.supported_managers
                    .iter()
                    .map(|m| m.framework.name().to_string())
                    .collect();

                println!("  {} - {} ({})",
                    plugin.name,
                    plugin.description,
                    frameworks.join(", ")
                );
            }
            println!();
        }
    }

    println!("\nTip: Use '--framework <name>' to filter by framework");
    println!();

    Ok(())
}

fn list_themes(framework_filter: Option<String>) -> Result<()> {
    if let Some(fw) = &framework_filter {
        println!("\nAvailable Themes for {}:\n", fw);
        let framework_type = parse_framework(fw)?;
        let themes = theme::get_themes_for_framework(&framework_type);

        if themes.is_empty() {
            println!("  No themes available for this framework.");
        } else {
            // Separate recommended and other themes
            let mut recommended = vec![];
            let mut others = vec![];

            for theme in themes {
                if theme.compatibility.is_recommended_for(&framework_type) {
                    recommended.push(theme);
                } else {
                    others.push(theme);
                }
            }

            if !recommended.is_empty() {
                println!("Recommended:");
                for theme in recommended {
                    println!("  ✓ {} - {}", theme.name, theme.description);
                    println!("    Preview: {}", theme.preview);
                }
                println!();
            }

            if !others.is_empty() {
                println!("Available:");
                for theme in others {
                    println!("    {} - {}", theme.name, theme.description);
                    println!("      Preview: {}", theme.preview);
                }
            }
        }
    } else {
        println!("\nAll Available Themes:\n");

        for theme in theme::THEME_REGISTRY.iter() {
            // Show which frameworks support this theme
            let frameworks: Vec<String> = theme.compatibility.supported_managers
                .iter()
                .map(|m| m.framework.name().to_string())
                .collect();

            println!("  {} - {}", theme.name, theme.description);
            println!("    Preview: {}", theme.preview);
            println!("    Frameworks: {}", frameworks.join(", "));
            println!();
        }
    }

    println!("Tip: Use '--framework <name>' to filter by framework");
    println!();

    Ok(())
}

fn parse_framework(name: &str) -> Result<FrameworkType> {
    match name.to_lowercase().as_str() {
        "oh-my-zsh" | "ohmyzsh" => Ok(FrameworkType::OhMyZsh),
        "zimfw" | "zim" => Ok(FrameworkType::Zimfw),
        "prezto" => Ok(FrameworkType::Prezto),
        "zinit" => Ok(FrameworkType::Zinit),
        "zap" => Ok(FrameworkType::Zap),
        _ => anyhow::bail!("Unknown framework: {}. Supported: oh-my-zsh, zimfw, prezto, zinit, zap", name),
    }
}
