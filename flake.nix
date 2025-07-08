{
  description = "SRA flake for Linux kernel development";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        rust-toolchain =
          fenix.packages.${system}.stable.withComponents
          [
            "cargo"
            "clippy"
            "rust-docs"
            "rust-std"
            "rustc"
            "rustfmt"
            "rust-src"
            "rust-analyzer"
          ];
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rust-toolchain
            python3
            pyright
            python3Packages.pandas
            python3Packages.plotnine
            python3Packages.paramiko
          ];
        };
      }
    );
}
