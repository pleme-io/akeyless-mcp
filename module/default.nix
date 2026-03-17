# akeyless-mcp home-manager module -- MCP server + CLI
#
# Namespace: services.akeyless_mcp.*
#
# Provides:
#   - MCP server entry (consumed by claude/anvil for all AI agents)
#   - CLI binary in PATH
#   - Config file generation (~/.config/akeyless_mcp/akeyless_mcp.yaml)
#   - Env propagation: AKEYLESS_MCP_CONFIG passed to MCP server process
#
# Usage:
#   services.akeyless_mcp.package = inputs.akeyless_mcp.packages.${system}.default;
#   services.akeyless_mcp.enable = true;
#   services.akeyless_mcp.mcp.enable = true;
{ hmHelpers }:
{
  lib,
  config,
  pkgs,
  ...
}:
with lib; let
  inherit (hmHelpers) mkMcpOptions mkMcpServerEntry;
  cfg = config.services.akeyless_mcp;
  mcpCfg = cfg.mcp;
  homeDir = config.home.homeDirectory;

  defaultApiKeyFile = "${homeDir}/.config/akeyless_mcp/api-key";

  resolvedApiKeyFile =
    if cfg.settings.apiKeyFile != null
    then cfg.settings.apiKeyFile
    else defaultApiKeyFile;

  configFile = pkgs.writeText "akeyless_mcp.yaml"
    (builtins.toJSON ({
      api_url = cfg.settings.apiUrl;
      api_key_file = resolvedApiKeyFile;
    }));

  mcpEnv = optionalAttrs cfg.settings.propagateApiKey {
    AKEYLESS_MCP_CONFIG = "${configFile}";
  };
in {
  options.services.akeyless_mcp = {
    enable = mkEnableOption "akeyless_mcp -- CLI + MCP server";

    package = mkOption {
      type = types.package;
      description = ''
        The akeyless_mcp binary package. Must be set explicitly from your flake input:
          services.akeyless_mcp.package = inputs.akeyless_mcp.packages.''${system}.default;
      '';
    };

    mcp = mkMcpOptions {
      defaultPackage = pkgs.hello;
    };

    settings = {
      apiUrl = mkOption {
        type = types.str;
        default = "https://api.akeyless.io";
        description = "API base URL.";
      };

      apiKeyFile = mkOption {
        type = types.nullOr types.str;
        default = null;
        description = ''
          Path to file containing the API key.
          When null, defaults to ~/.config/akeyless_mcp/api-key.
        '';
      };

      propagateApiKey = mkOption {
        type = types.bool;
        default = true;
        description = ''
          Pass config file path to the MCP server process via AKEYLESS_MCP_CONFIG env.
          Ensures the MCP server can find the API key when launched by Claude
          Code or other MCP clients that don't inherit user environment.
        '';
      };
    };
  };

  config = mkMerge [
    {
      services.akeyless_mcp.mcp.package = mkDefault cfg.package;
    }

    (mkIf cfg.enable {
      home.packages = [ cfg.package ];

      xdg.configFile."akeyless_mcp/akeyless_mcp.yaml".source = configFile;
    })

    (mkIf mcpCfg.enable {
      services.akeyless_mcp.mcp.serverEntry = mkMcpServerEntry ({
        command = "${mcpCfg.package}/bin/akeyless_mcp";
      } // optionalAttrs (mcpEnv != {}) {
        env = mcpEnv;
      });
    })
  ];
}
