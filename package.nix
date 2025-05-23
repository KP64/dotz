{
  lib,
  rustPlatform,
  stdenv,
  darwin,
  pkg-config,
  openssl,
}:

rustPlatform.buildRustPackage rec {
  pname = "dotz";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ pkg-config ];

  buildInputs = [ openssl ] ++ (lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.Security);

  useNextest = true;

  meta = {
    description = "A colorscript that gradually fills your screen with (a) character.";
    homepage = "https://github.com/KP64/dotz";
    license = lib.licenses.unlicense;
    maintainers = with lib.maintainers; [ KP64 ];
    mainProgram = pname;
  };
}
