{
  perSystem =
    { config, pkgs, ... }:
    {
      devShells.default = pkgs.mkShell {
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
