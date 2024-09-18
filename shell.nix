{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = [
    pkgs.rustc
    pkgs.rustfmt
    pkgs.rust-analyzer
  ];
}
