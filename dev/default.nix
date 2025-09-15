{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];

  perSystem =
    { self', pkgs, ... }:
    {
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

      devShells.default = pkgs.mkShell {
        name = "dotz";

        inputsFrom = [ self'.packages.default ];

        packages = with pkgs; [
          # Nix lsp ❄️
          nil

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
