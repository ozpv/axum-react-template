{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
	rustup
    cargo-binutils
    cargo-watch
    postgresql
    sqlx-cli
    nodejs_23
  ];
}

