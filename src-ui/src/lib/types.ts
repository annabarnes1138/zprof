/**
 * TypeScript type definitions for Tauri IPC
 *
 * These types must match the Rust types in src-tauri/src/types.rs
 */

/** Profile information for display in profile list */
export interface ProfileInfo {
  /** Profile name (unique identifier) */
  name: string;
  /** Framework name (oh-my-zsh, zimfw, etc.) */
  framework: string;
  /** Prompt mode: "prompt_engine" or "framework_theme" */
  prompt_mode: string;
  /** Prompt engine name (if using prompt_engine mode) */
  prompt_engine?: string;
  /** Framework theme name (if using framework_theme mode) */
  framework_theme?: string;
  /** Whether this profile is currently active */
  active: boolean;
  /** ISO 8601 timestamp when profile was created */
  created_at: string;
  /** Number of enabled plugins */
  plugin_count: number;
}

/** Prompt mode information discriminated union */
export type PromptModeInfo =
  | {
      type: "prompt_engine";
      engine: string;
    }
  | {
      type: "framework_theme";
      theme: string;
    };

/** Full profile details including configuration */
export interface ProfileDetails {
  /** Profile name */
  name: string;
  /** Framework name */
  framework: string;
  /** Prompt mode discriminator */
  prompt_mode: PromptModeInfo;
  /** List of enabled plugins */
  plugins: string[];
  /** Environment variables */
  env_vars: Record<string, string>;
  /** ISO 8601 timestamp when created */
  created_at: string;
  /** ISO 8601 timestamp when last modified */
  modified_at: string;
}

/** Profile creation/update configuration */
export interface ProfileConfig {
  /** Profile name (must be unique, lowercase, alphanumeric + hyphens) */
  name: string;
  /** Framework to use */
  framework: string;
  /** Prompt mode: "prompt_engine" or "framework_theme" */
  prompt_mode: string;
  /** Prompt engine name (if prompt_mode = "prompt_engine") */
  prompt_engine?: string;
  /** Framework theme (if prompt_mode = "framework_theme") */
  framework_theme?: string;
  /** List of plugin names to enable */
  plugins: string[];
  /** Environment variables to set */
  env_vars: Record<string, string>;
}

/** Framework information */
export interface FrameworkInfo {
  /** Framework name */
  name: string;
  /** Human-readable description */
  description: string;
  /** Whether this framework supports themes */
  supports_themes: boolean;
  /** Whether this framework supports plugins */
  supports_plugins: boolean;
}

/** Plugin information */
export interface PluginInfo {
  /** Plugin name */
  name: string;
  /** Description of what the plugin does */
  description: string;
  /** Category (git, docker, utility, etc.) */
  category: string;
  /** Framework this plugin is for */
  framework: string;
}

/** Theme information */
export interface ThemeInfo {
  /** Theme name */
  name: string;
  /** Description of the theme */
  description: string;
  /** Framework this theme is for */
  framework: string;
  /** Optional URL to preview image */
  preview_url?: string;
}

/** Prompt engine information */
export interface PromptEngineInfo {
  /** Engine name */
  name: string;
  /** Description of the engine */
  description: string;
  /** Whether this engine requires Nerd Fonts */
  nerd_font_required: boolean;
  /** Whether this engine works across multiple shells */
  cross_shell: boolean;
  /** Whether this engine supports async rendering */
  async_rendering: boolean;
  /** Optional URL to preview image */
  preview_url?: string;
  /** Whether this engine is already installed */
  installed?: boolean;
}
