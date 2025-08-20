{ self, inputs, ... }:
{
  perSystem =
    {
      inputs',
      lib,
      pkgs,
      ...
    }:
    let
      toolchain = inputs'.fenix.packages.fromToolchainFile {
        file = "${self}/rust-toolchain.toml";
        sha256 = "sha256-+9FmLhAOezBZCOziO0Qct1NOrfpjNsXxc/8I0c7BdKE=";
      };
      naersk' = pkgs.callPackage inputs.naersk {
        cargo = toolchain;
        rustc = toolchain;
      };

      manifest = (lib.importTOML "${self}/Cargo.toml").package;
    in
    {
      packages.default = naersk'.buildPackage {
        pname = manifest.name;
        inherit (manifest) version;

        src = lib.fileset.toSource {
          root = ../.;
          fileset = lib.fileset.unions [
            ../.cargo
            ../Cargo.toml
            ../Cargo.lock

            ../src
          ];
        };

        nativeBuildInputs = lib.optional pkgs.stdenv.isLinux pkgs.mold;

        meta = {
          description = "A colorscript that gradually fills your screen with (a) character.";
          homepage = "https://github.com/KP64/dotz";
          license = lib.licenses.unlicense;
          mainProgram = "dotz";
        };
      };
    };
}
