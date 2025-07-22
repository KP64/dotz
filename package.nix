{
  self,
  lib,
  makeRustPlatform,
  mold,
  rust-bin,
  stdenv,
}:
let
  rustToolchain = rust-bin.fromRustupToolchainFile "${self}/rust-toolchain.toml";
  manifest = (lib.importTOML "${self}/Cargo.toml").package;
in
(makeRustPlatform rec {
  rustc = rustToolchain;
  cargo = rustc;
}).buildRustPackage
  rec {
    pname = manifest.name;
    inherit (manifest) version;

    src = self;
    cargoLock.lockFile = "${self}/Cargo.lock";

    nativeBuildInputs = lib.optional stdenv.isLinux mold;

    useNextest = true;

    meta = {
      description = "A colorscript that gradually fills your screen with (a) character.";
      homepage = "https://github.com/KP64/dotz";
      license = lib.licenses.unlicense;
      maintainers = with lib.maintainers; [ KP64 ];
      mainProgram = pname;
    };
  }
