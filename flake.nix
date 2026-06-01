{
  description = "Fully featured flake ❄️ for rusty 🦀 development";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

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
          config,
          self',
          inputs',
          lib,
          pkgs,
          ...
        }:
        let
          craneLib = lib.pipe pkgs [
            inputs.crane.mkLib
            (clib: clib.overrideScope (_: _: { stdenvSelector = _: self'.devShells.default.stdenv; }))
            (clib: clib.overrideToolchain inputs'.fenix.packages.stable.toolchain)
          ];

          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          dotz = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        in
        {
          packages.default = dotz;

          checks = config.packages // {
            audit = craneLib.cargoAudit {
              inherit (commonArgs) src;
              inherit (inputs) advisory-db;
            };
            clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets";
              }
            );
            deny = craneLib.cargoDeny { inherit (commonArgs) src; };
            doc = craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; });
            nextest = craneLib.cargoNextest (commonArgs // { inherit cargoArtifacts; });
            tarpaulin = craneLib.cargoTarpaulin (commonArgs // { inherit cargoArtifacts; });
          };
        };
    };
}
