{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
  buildInputs = [
    # rustc
    # cargo
    # rust-analyzer
    # cargo-watch # TODO: try it
    # rustfmt

    pkg-config
    fuse3
  ];
}
