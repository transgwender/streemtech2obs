{ overlays }:

rec {
  streemtech2obs = import ./streemtech2obs-service.nix;
  default = streemtech2obs;
}