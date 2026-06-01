{
  perSystem =
    { config, pkgs, ... }:
    let
      wildStdenv = pkgs.useWildLinker pkgs.gcc16Stdenv;
    in
    {
      devShells.default = pkgs.mkShell.override { stdenv = wildStdenv; } {
        name = "dotz";

        inputsFrom = builtins.attrValues config.packages;

        packages = with pkgs; [
          # Nix lsp ❄️
          nil

          # vhs

          # Next gen testing 🧪
          cargo-nextest

          # License 📜
          cargo-deny

          # Dependencies 📦
          cargo-edit
          cargo-machete
        ];
      };
    };
}
