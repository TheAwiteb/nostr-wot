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
        MSRV = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.rust-version;
        msrvToolchain = pkgs.rust-bin.stable.${MSRV};
      in
      with pkgs;
      {
        devShells.default = mkShell {
          packages = [
            git-cliff
            taplo
          ];

          nativeBuildInputs = [
            (lib.hiPrio rust-bin.nightly."2025-12-10".rustfmt)
            rust-bin.stable.latest.default
            rust-analyzer
          ];
        };

        apps.msrv = {
          type = "app";
          program = "${writeScript "run-script" ''
            #!${stdenv.shell}
            echo "Build with MSRV"
            ${msrvToolchain.rustc}/bin/rustc --version
            ${msrvToolchain.cargo}/bin/cargo build
          ''}";
        };
      }
    );
}
