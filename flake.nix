{
  description = "akeyless_mcp -- The purpose of this application is to provide access to Akeyless API.";

  nixConfig = {
    allow-import-from-derivation = true;
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    crate2nix.url = "github:nix-community/crate2nix";
    flake-utils.url = "github:numtide/flake-utils";
    substrate = {
      url = "github:pleme-io/substrate";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crate2nix,
    flake-utils,
    substrate,
    devenv,
  }:
    (import "${substrate}/lib/rust-tool-release-flake.nix" {
      inherit nixpkgs crate2nix flake-utils devenv;
    }) {
      toolName = "akeyless_mcp";
      src = self;
      repo = "pleme-io/akeyless_mcp";
      crateOverrides = {
        rmcp = attrs: {
          CARGO_CRATE_NAME = "rmcp";
        };
      };

      # Migration to substrate module-trio + anvil-MCP wiring.
      #
      # Legacy module surface (preserved drop-in):
      #   services.akeyless_mcp.{enable,package}
      #   services.akeyless_mcp.settings.{apiUrl,apiKeyFile,propagateApiKey}
      #   services.akeyless_mcp.mcp.{enable,scopes,agents,package,...}
      #
      # The trio replaces:
      #   - enable/package via the standard surface
      #   - mcp.* via withAnvilMcp (the trio's first-class anvil
      #     registration — replaces the legacy mkMcpOptions/
      #     mkMcpServerEntry pair). Anvil registration writes to
      #     blackmatter.components.anvil.mcp.servers.akeyless_mcp.
      #
      # The trio's withShikumiConfig adds a `services.<name>.settings`
      # attrs mkOption that conflicts with the legacy nested settings.*
      # shape, so we keep the existing typed settings.{apiUrl,
      # apiKeyFile,propagateApiKey} as bespoke extraHmOptions and
      # emit the YAML config + AKEYLESS_MCP_CONFIG anvil-env wiring
      # ourselves in extraHmConfigFn.
      module = {
        description = "akeyless_mcp — CLI + MCP server for Akeyless API";
        hmNamespace = "services";

        withAnvilMcp = true;
        anvilDefaultEnable = false;

        extraHmOptions = {
          settings = {
            apiUrl = nixpkgs.lib.mkOption {
              type = nixpkgs.lib.types.str;
              default = "https://api.akeyless.io";
              description = "API base URL.";
            };
            apiKeyFile = nixpkgs.lib.mkOption {
              type = nixpkgs.lib.types.nullOr nixpkgs.lib.types.str;
              default = null;
              description = ''
                Path to file containing the API key.
                When null, defaults to ~/.config/akeyless_mcp/api-key.
              '';
            };
            propagateApiKey = nixpkgs.lib.mkOption {
              type = nixpkgs.lib.types.bool;
              default = true;
              description = ''
                Pass config file path to the MCP server process via AKEYLESS_MCP_CONFIG env.
                Ensures the MCP server can find the API key when launched by Claude
                Code or other MCP clients that don't inherit user environment.
              '';
            };
          };
        };

        # YAML config + AKEYLESS_MCP_CONFIG env propagation. Mirrors the
        # legacy module's logic: builds config file from typed settings,
        # writes ~/.config/akeyless_mcp/akeyless_mcp.yaml, and (when
        # propagateApiKey is true) injects the path into the anvil MCP
        # server's environment so Claude / Cursor / OpenCode can resolve
        # the API key.
        extraHmConfigFn = { cfg, pkgs, lib, config, ... }:
          let
            homeDir = config.home.homeDirectory;
            defaultApiKeyFile = "${homeDir}/.config/akeyless_mcp/api-key";
            resolvedApiKeyFile =
              if cfg.settings.apiKeyFile != null
              then cfg.settings.apiKeyFile
              else defaultApiKeyFile;
            configFile = pkgs.writeText "akeyless_mcp.yaml"
              (builtins.toJSON {
                api_url = cfg.settings.apiUrl;
                api_key_file = resolvedApiKeyFile;
              });
            mcpCfg = config.services.akeyless_mcp.mcp or null;
            anvilEntry =
              if mcpCfg != null && mcpCfg.enable && cfg.settings.propagateApiKey
              then {
                blackmatter.components.anvil.mcp.servers.akeyless_mcp.env = {
                  AKEYLESS_MCP_CONFIG = "${configFile}";
                };
              }
              else { };
          in lib.mkMerge [
            (lib.mkIf cfg.enable {
              xdg.configFile."akeyless_mcp/akeyless_mcp.yaml".source = configFile;
            })
            anvilEntry
          ];
      };
    };
}
