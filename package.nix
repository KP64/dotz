{
  self,
  lib,
  rustPlatform,
  pkg-config,
  mold,
  rust-bin,
}:

rustPlatform.buildRustPackage rec {
  pname = "dotz";
  version = "0.1.0";

  src = self;

  cargoLock.lockFile = "${self}/Cargo.lock";

  nativeBuildInputs = [
    pkg-config
    mold
    (rust-bin.fromRustupToolchainFile "${self}/rust-toolchain.toml")
  ];

  useNextest = true;

  meta = {
    description = "A colorscript that gradually fills your screen with (a) character.";
    homepage = "https://github.com/KP64/dotz";
    license = lib.licenses.unlicense;
    maintainers = with lib.maintainers; [ KP64 ];
    mainProgram = pname;
  };
}
