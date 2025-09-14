{
  # build: nix build .
  # update lockfile: nix flake update
  # run: ./result/bin/rizzybox
  description = "RizzyBox Nix Flake";

  inputs = {
    nixpkgs.url = "https://api.flakehub.com/f/NixOS/nixpkgs/0.2505.809873+rev-9a094440e02a699be5c57453a092a8baf569bdad.tar.gz";
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
        rustToolchain = final.rust-bin.stable.latest.default;
      };

      packages = forAllSystems ({ pkgs }: {
        default =
          let
            rustPlatform = pkgs.makeRustPlatform {
              cargo = pkgs.rustToolchain;
              rustc = pkgs.rustToolchain;
            };
            securityFramework = if pkgs.stdenv.isDarwin then [ pkgs.darwin.apple_sdk.frameworks.Security ] else [];
          in
          rustPlatform.buildRustPackage {
            name = "rizzybox";
            src = ./.;
            nativeBuildInputs = securityFramework;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
      });
    };
}
