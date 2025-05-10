{
  description = "Fully featured flake ❄️ for rusty 🦀 development";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      imports = [ inputs.treefmt-nix.flakeModule ];

      perSystem =
        { pkgs, system, ... }:
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          treefmt.programs = {
            deadnix.enable = true;
            statix.enable = true;
            nixfmt = {
              enable = true;
              strict = true;
            };

            prettier.enable = true;

            shfmt.enable = true;
            shellcheck.enable = true;

            rustfmt.enable = true;

            taplo.enable = true;
          };

          packages.default = pkgs.callPackage ./package.nix { };

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              # Nix lsp ❄️
              nil

              # For the extra reinforcement 😂
              cargo-mommy

              # file watcher 👀
              bacon

              # Next gen testing 🧪
              cargo-nextest
              cargo-flamegraph
              cargo-mutants
              cargo-tarpaulin

              # License 📜
              cargo-deny

              # supply chain ⛓️
              cargo-vet
              cargo-auditable
              cargo-crev

              # Dependencies 📦
              cargo-udeps
              cargo-machete

              # Tasks 🛠️
              cargo-make
              cargo-chef
              cargo-cross

              # Unsafe ☢️
              cargo-geiger

              # Inner workings ⚙️
              cargo-show-asm
              cargo-expand

              # misc ❔
              cargo-msrv
              cargo-release
            ];

            nativeBuildInputs = [ (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) ];
          };
        };
    };
}
