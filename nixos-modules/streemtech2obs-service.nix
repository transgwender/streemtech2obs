{
  config,
  pkgs,
  lib ? pkgs.lib,
  ...
}:

with lib;

let

  cfg = config.services.streemtech2obs;

in

{
  # Interface
  options = {
    services.streemtech2obs = rec {
      enable = mkOption {
        type = types.bool;
        default = false;
        description = ''
          Whether to run the service
        '';
      };
    };
  };

  # Implementation
  config = mkIf cfg.enable {

    users.extraGroups.streemtech2obs = {};
    users.extraUsers.streemtech2obs = {
      description = "streemtech2obs";
      group = "streemtech2obs";
      isSystemUser = true;
      useDefaultShell = true;
    };

    environment.systemPackages = [ pkgs.streemtech2obs ];

    systemd.services.streemtech2obs = {
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        Restart = "on-failure";
        ExecStart = "+${pkgs.streemtech2obs}/bin/streemtech2obs /etc/streemtech2obs/config -u";
        User = "streemtech2obs";
        RuntimeDirectory = "streemtech2obs";
        RuntimeDirectoryMode = "0755";
        StateDirectory = "streemtech2obs";
        StateDirectoryMode = "0700";
        CacheDirectory = "streemtech2obs";
        CacheDirectoryMode = "0750";
        StandardOutput = "journal";
      };
    };
  };
}