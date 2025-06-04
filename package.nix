{
  self,
  lib,
  rustPlatform,
  pkg-config,
}:

rustPlatform.buildRustPackage rec {
  pname = "dotz";
  version = "0.1.0";

  src = self;

  cargoLock.lockFile = "${self}/Cargo.lock";

  nativeBuildInputs = [ pkg-config ];

  useNextest = true;

  meta = {
    description = "A colorscript that gradually fills your screen with (a) character.";
    homepage = "https://github.com/KP64/dotz";
    license = lib.licenses.unlicense;
    maintainers = with lib.maintainers; [ KP64 ];
    mainProgram = pname;
  };
}
