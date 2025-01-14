{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
	rustc
	cargo
	clippy
	rustfmt
	lld
    cargo-binutils
    cargo-watch
    postgresql
    sqlx-cli
    nodejs_23
  ];
}

