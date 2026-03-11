// Bridge - Unified IPC module
//
// Purpose: Main entry point for components/stores to access IPC functionality
//
// Re-exports commands and events, provides unified configuration

// DTOs - re-export for convenience
export * from "./dto";

// Commands - re-export all command functions
export * from "./commands";

// Events - re-export all event utilities
export * from "./events";

// ==================== BRIDGE CONFIGURATION ====================

export interface BridgeConfig {
  /**
   * Enable debug logging for IPC calls
   * @default false
   */
  debug: boolean;

  /**
   * Timeout in milliseconds for command invocations
   * @default 30000
   */
  commandTimeout: number;
}

const DEFAULT_CONFIG: BridgeConfig = {
  debug: false,
  commandTimeout: 30000,
};

let currentConfig: BridgeConfig = { ...DEFAULT_CONFIG };

/**
 * Get the current bridge configuration
 */
export function getConfig(): BridgeConfig {
  return { ...currentConfig };
}

/**
 * Update bridge configuration
 */
export function setConfig(config: Partial<BridgeConfig>): void {
  currentConfig = { ...currentConfig, ...config };
}

/**
 * Reset bridge configuration to defaults
 */
export function resetConfig(): void {
  currentConfig = { ...DEFAULT_CONFIG };
}

// ==================== VERSION INFO ====================

export const BRIDGE_VERSION = "0.1.0";
