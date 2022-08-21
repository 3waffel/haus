{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devshell.url = "github:numtide/devshell";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    devshell,
    naersk,
    fenix,
    nix-filter,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            devshell.overlay
            fenix.overlay
            (final: prev: rec {
              rustWithComponents = prev.fenix.complete.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ];
              wasm-toolchain = with prev.fenix;
                combine [
                  rustWithComponents
                  targets.wasm32-unknown-unknown.latest.rust-std
                ];
            })
          ];
        };
        rust-wasm = with pkgs;
          naersk.lib.${system}.override {
            cargo = wasm-toolchain;
            rustc = wasm-toolchain;
          };
      in {
        packages.wasm = rust-wasm.buildPackage {
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          src = nix-filter.lib {
            root = ./.;
            include = [
              "Cargo.lock"
              "Cargo.toml"
              (nix-filter.lib.inDirectory "src")
            ];
          };
          copyLibs = true;
        };

        devShells.default = pkgs.devshell.mkShell {
          imports = [
            (pkgs.devshell.importTOML ./devshell.toml)
          ];
          env = [
            {
              name = "PKG_CONFIG_PATH";
              value = "${pkgs.openssl.dev}/lib/pkgconfig";
            }
          ];
        };
      }
    );
}
