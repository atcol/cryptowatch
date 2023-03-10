{
  description = "EPSG Coordinate Reference System tools & data";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
        };
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = [
            pkgconfig
            cargo-generate
            cargo-geiger
            tokei
            rust
            watchexec
            openssl
            docker-compose
            bacon
            protobuf
          ];
        };
      }
    );
}
