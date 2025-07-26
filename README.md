# Dotz

A colorscript that gradually fills your screen with (a) character.

![Preview](./demo.gif)

## Installation

### Compile from source

To download the source code, build the dotz binary, and install it in `$HOME/.cargo/bin` run:

```sh
cargo install --locked --git https://github.com/KP64/dotz
```

### Nix

#### Run without installing it

```sh
nix run github:KP64/dotz -- [OPTIONS]
```

#### Flakes

```nix
# flake.nix
{
  inputs.dotz.url = "github:KP64/dotz";
}

# your configuration
{ inputs, pkgs, ... }:
{
  environment.systemPackages = [ inputs.dotz.packages.${pkgs.system}.default ];
}
```

## Usage

```txt
A colorscript that gradually fills your screen with (a) character.

Usage: dotz [OPTIONS] [COMMAND]

Commands:
  fill-screen  Fill the screen immediately
  infinite     Continuously print characters
  random       Randomly color individual cells over time
  spaced       Print a Character every few spaces
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --char <CHAR>  The character to be printed [default: .]
  -h, --help         Print help
  -V, --version      Print version
```

## (Un)License

dotz is released into the public domain.
See the [UNLICENSE](./UNLICENSE) for more details.
