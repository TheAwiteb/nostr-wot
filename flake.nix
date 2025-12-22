{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          packages = [
            git-cliff
            taplo
          ];

          nativeBuildInputs = [
            (lib.hiPrio rust-bin.nightly."2025-08-07".rustfmt)
            rust-bin.stable.latest.default
            rust-analyzer
          ];
        };
      }
    );
}
