{
  # build: nix build .
  # update lockfile: nix flake update
  # run: ./result/bin/rizzybox
  description = "RizzyBox Nix Flake";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2405.635979+rev-4eb33fe664af7b41a4c446f87d20c9a0a6321fa3.tar.gz";
    # Provides helpers for Rust toolchains
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      # Systems supported
      allSystems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "x86_64-darwin" # 64-bit Intel macOS
        "aarch64-darwin" # 64-bit ARM macOS
      ];

      # Helper to provide system-specific attributes
      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            # Provides Nixpkgs with a rust-bin attribute for building Rust toolchains
            rust-overlay.overlays.default
            # Uses the rust-bin attribute to select a Rust toolchain
            self.overlays.default
          ];
        };
      });
    in
    {
      overlays.default = final: prev: {
        rustToolchain = final.rust-bin.stable."1.82.0".default;
      };

      packages = forAllSystems ({ pkgs }: {
        default =
          let
            rustPlatform = pkgs.makeRustPlatform {
              cargo = pkgs.rustToolchain;
              rustc = pkgs.rustToolchain;
            };
          in
          rustPlatform.buildRustPackage {
            name = "rizzybox";
            src = ./.;
            nativeBuildInputs = [ pkgs.darwin.apple_sdk.frameworks.Security ];
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
      });
    };
}
