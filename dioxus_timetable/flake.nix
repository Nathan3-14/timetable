{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      perSystem =
        {
          pkgs,
          ...
        }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rust-toolchain = pkgs.symlinkJoin {
            name = "rust-toolchain";
            paths = with pkgs; [
              rustc
              cargo
              cargo-watch
              rust-analyzer
              rustPlatform.rustcSrc
              cargo-dist
              cargo-tarpaulin
              cargo-insta
              cargo-machete
              cargo-edit
            ];
          };

          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            glib-networking
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
            wasm-bindgen-cli
            lld_20
            xdotool
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            cargo
            cargo-tauri
            nodejs
          ];
        in
        {
          # Rust package
          packages.default = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            RUST_BACKTRACE = "full";

            nativeBuildInputs = nativeBuildInputs;
            buildInputs = buildInputs;
          };

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            RUST_BACKTRACE = "full";
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

            packages =
              nativeBuildInputs
              ++ buildInputs
              ++ [
                rust-toolchain
              ]
              ++ (with pkgs; [
                clippy
                dioxus-cli
              ]);
          };

        };
    };
}
