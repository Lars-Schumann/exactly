{
  description = "rust dev shell for tablebased";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))

          clippy
          bacon

          cargo-msrv

          nixfmt
          nixd

        ];
        env = {
          RUST_BACKTRACE = "1";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };

        shellHook = ''
          export PS1="\[\e[1;33m\][exactly]\[\e[0m\] $PS1"

          alias msrv="cargo msrv find --linear -- cargo test"

          export RUSTFLAGS="-Znext-solver=globally"
        '';
      };
    };
}