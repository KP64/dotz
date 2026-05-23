{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];

  perSystem = {
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
  };
}
