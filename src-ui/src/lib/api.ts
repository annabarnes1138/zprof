/**
 * API client for Tauri IPC commands
 *
 * This module provides a typed wrapper around Tauri's invoke() function
 * for communicating with the Rust backend.
 */

import { invoke } from "@tauri-apps/api/core";
import type {
  ProfileInfo,
  ProfileDetails,
  ProfileConfig,
  FrameworkInfo,
  PluginInfo,
  ThemeInfo,
  PromptEngineInfo,
} from "./types";

/**
 * List all profiles with their basic information
 *
 * @returns Array of profile information
 * @throws Error if profiles cannot be loaded
 */
export async function listProfiles(): Promise<ProfileInfo[]> {
  try {
    return await invoke<ProfileInfo[]>("list_profiles");
  } catch (error) {
    console.error("Failed to list profiles:", error);
    throw new Error(`Failed to load profiles: ${error}`);
  }
}

/**
 * Get detailed information for a specific profile
 *
 * @param name - Profile name
 * @returns Detailed profile information
 * @throws Error if profile not found or cannot be loaded
 */
export async function getProfile(name: string): Promise<ProfileDetails> {
  try {
    return await invoke<ProfileDetails>("get_profile", { name });
  } catch (error) {
    console.error(`Failed to get profile '${name}':`, error);
    throw new Error(`Failed to load profile: ${error}`);
  }
}

/**
 * Get the currently active profile name
 *
 * @returns Active profile name or null if no profile is active
 * @throws Error if config cannot be read
 */
export async function getActiveProfile(): Promise<string | null> {
  try {
    return await invoke<string | null>("get_active_profile");
  } catch (error) {
    console.error("Failed to get active profile:", error);
    return null;
  }
}

/**
 * Create a new profile from configuration
 *
 * @param config - Profile configuration
 * @returns Created profile name
 * @throws Error if profile creation fails or profile already exists
 */
export async function createProfile(config: ProfileConfig): Promise<string> {
  try {
    return await invoke<string>("create_profile", { config });
  } catch (error) {
    console.error("Failed to create profile:", error);
    throw new Error(`Failed to create profile: ${error}`);
  }
}

/**
 * Delete a profile
 *
 * @param name - Profile name to delete
 * @throws Error if profile cannot be deleted (e.g., is active)
 */
export async function deleteProfile(name: string): Promise<void> {
  try {
    await invoke("delete_profile", { name });
  } catch (error) {
    console.error(`Failed to delete profile '${name}':`, error);
    throw new Error(`Failed to delete profile: ${error}`);
  }
}

/**
 * Activate a profile (switch to it)
 *
 * @param name - Profile name to activate
 * @throws Error if profile cannot be activated (e.g., not found)
 */
export async function activateProfile(name: string): Promise<void> {
  try {
    await invoke("activate_profile", { name });
  } catch (error) {
    console.error(`Failed to activate profile '${name}':`, error);
    throw new Error(`Failed to activate profile: ${error}`);
  }
}

/**
 * Get list of available frameworks
 *
 * @returns Array of framework information
 * @throws Error if frameworks cannot be loaded
 */
export async function getFrameworks(): Promise<FrameworkInfo[]> {
  try {
    return await invoke<FrameworkInfo[]>("get_frameworks");
  } catch (error) {
    console.error("Failed to get frameworks:", error);
    throw new Error(`Failed to load frameworks: ${error}`);
  }
}

/**
 * Get available plugins for a specific framework
 *
 * @param framework - Framework name (oh-my-zsh, zimfw, etc.)
 * @returns Array of plugin information for this framework
 * @throws Error if plugins cannot be loaded
 */
export async function getPlugins(framework: string): Promise<PluginInfo[]> {
  try {
    return await invoke<PluginInfo[]>("get_plugins", { framework });
  } catch (error) {
    console.error(`Failed to get plugins for ${framework}:`, error);
    throw new Error(`Failed to load plugins: ${error}`);
  }
}

/**
 * Get available themes for a specific framework
 *
 * @param framework - Framework name (oh-my-zsh, zimfw, etc.)
 * @returns Array of theme information for this framework
 * @throws Error if themes cannot be loaded
 */
export async function getThemes(framework: string): Promise<ThemeInfo[]> {
  try {
    return await invoke<ThemeInfo[]>("get_themes", { framework });
  } catch (error) {
    console.error(`Failed to get themes for ${framework}:`, error);
    throw new Error(`Failed to load themes: ${error}`);
  }
}

/**
 * Get available prompt engines
 *
 * @returns Array of prompt engine information
 * @throws Error if engines cannot be loaded
 */
export async function getPromptEngines(): Promise<PromptEngineInfo[]> {
  try {
    return await invoke<PromptEngineInfo[]>("get_prompt_engines");
  } catch (error) {
    console.error("Failed to get prompt engines:", error);
    throw new Error(`Failed to load prompt engines: ${error}`);
  }
}

/**
 * Check if a prompt engine is installed
 *
 * @param engine - Engine name to check
 * @returns True if the engine is installed, false otherwise
 * @throws Error if check fails
 */
export async function checkEngineInstalled(engine: string): Promise<boolean> {
  try {
    return await invoke<boolean>("check_engine_installed", { engine });
  } catch (error) {
    console.error(`Failed to check if ${engine} is installed:`, error);
    return false;
  }
}

/**
 * Install a prompt engine
 *
 * @param engine - Engine name to install
 * @throws Error if installation fails
 */
export async function installPromptEngine(engine: string): Promise<void> {
  try {
    await invoke("install_prompt_engine", { engine });
  } catch (error) {
    console.error(`Failed to install ${engine}:`, error);
    throw new Error(`Installation failed: ${error}`);
  }
}
