{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
  };

  outputs =
    inputs@{
      flake-parts,
      nixpkgs,
      rust-overlay,
      crane,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.platforms.all;
      perSystem =
        {
          pkgs,
          ...
        }:
        let
          overlays = [ (import rust-overlay) ];
          pkgs' = pkgs.appendOverlays overlays;
          toolchainFor =
            p:
            p.rust-bin.selectLatestNightlyWith (
              toolchain:
              toolchain.default.override {
                targets = [ "wasm32-unknown-unknown" ];
              }
            );
          craneLib = (crane.mkLib pkgs').overrideToolchain toolchainFor;
        in
        {
          packages.default = pkgs'.callPackage ./package.nix { inherit craneLib; };
          devShells.default =
            with pkgs';
            mkShell {
              packages = [
                ((toolchainFor pkgs').override {
                  extensions = [
                    "rust-src"
                    "rust-analyzer"
                  ];
                })
                alejandra
                taplo
              ];
            };
        };
    };
}
