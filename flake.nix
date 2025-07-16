{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";
    flake-utils.url = "github:numtide/flake-utils";

    cargo2nix = {
        url = "github:cargo2nix/cargo2nix";
        inputs.nixpkgs.follows = "nixpkgs";
        inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = inputs: with inputs; # pass through all inputs and bring them into scope

    # Build the output set for each default system and map system sets into
    # attributes, resulting in paths such as:
    # nix build .#packages.x86_64-linux.<name>
    flake-utils.lib.eachDefaultSystem (system:

      # let-in expressions, very similar to Rust's let bindings.  These names
      # are used to express the output but not themselves paths in the output.
      let

        # create nixpkgs that contains rustBuilder from cargo2nix overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = overlayList;
        };
        overlayList = [ cargo2nix.overlays.default ];

        # create the workspace & dependencies package set
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.83.0";
          packageFun = import ./Cargo.nix;
        };

      in rec {
        # this is the output (recursive) set (expressed for each system)

        # A Nixpkgs overlay that provides a 'streemtech2obs' package.
        overlays.default = final: prev: { streemtech2obs = final.callPackage ./package.nix { rustPkgs = rustPkgs; }; };

        # the packages in `nix build .#packages.<system>.<name>`
        packages = {
          # nix build .#streemtech2obs
          # nix build .#packages.x86_64-linux.streemtech2obs
          streemtech2obs = (rustPkgs.workspace.streemtech2obs {});
          # nix build
          default = packages.streemtech2obs; # rec
        };

        nixosModules = import ./nixos-modules {
          overlays = overlayList;
        };
      }
    ) // {
      nixosConfigurations.container = nixpkgs.lib.nixosSystem rec {
        system = "x86_64-linux";

        specialArgs = { inherit inputs; };

        modules = [
          self.nixosModules.${system}.default
          ({ pkgs, config, inputs, ... }: {
            # Only allow this to boot as a container
            boot.isContainer = true;

            networking.hostName = "streemtech2obs";

            services.streemtech2obs = {
              enable = true;
            };

            system.stateVersion = "24.11";
          })
        ];
      };
    };
}