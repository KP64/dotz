{
  description = "Fully featured flake ❄️ for rusty 🦀 development";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      debug = true;

      systems = inputs.nixpkgs.lib.systems.flakeExposed;

      imports = [ flake-parts.flakeModules.partitions ];

      partitionedAttrs = {
        checks = "dev";
        devShells = "dev";
        formatter = "dev";
      };

      partitions.dev = {
        extraInputsFlake = ./dev;
        module.imports = [ ./dev ];
      };

      perSystem =
        {
          self',
          inputs',
          lib,
          pkgs,
          ...
        }:
        let
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain inputs'.fenix.packages.stable.toolchain;

          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;
            nativeBuildInputs = lib.optional pkgs.stdenv.isLinux pkgs.mold;
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          dotz = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        in
        {
          packages.default = dotz;

          checks = self'.packages // {
            dotzClippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets";
              }
            );
            dotzDoc = craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; });
            dotzDeny = craneLib.cargoDeny { inherit (commonArgs) src; };
            dotzNextest = craneLib.cargoNextest (commonArgs // { inherit cargoArtifacts; });
          };
        };
    };
}
