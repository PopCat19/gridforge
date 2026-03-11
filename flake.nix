# flake.nix
#
# Purpose: Nix flake for Gridforge development environment
#
# This flake provides a devShell with Tauri Linux build dependencies
{
  description = "Gridforge - Tauri SolidJS application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Rust toolchain
            rustup
            cargo
            rustfmt
            clippy

            # Tauri Linux dependencies (ABI version 4.1)
            webkitgtk_4_1
            gtk3
            libsoup_3
            glib-networking
            gsettings-desktop-schemas

            # Build dependencies
            pkg-config
            openssl
            cmake
            ninja

            # Frontend build tools
            bun
            nodejs

            # Linux desktop dependencies
            libadwaita
            libappindicator
            librsvg
            pango
            cairo
            gdk-pixbuf
          ];

          # Required environment variables
          env = {
            XDG_DATA_DIRS = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas:/usr/share";
            WEBKIT_DISABLE_COMPOSITING_MODE = "1";
          };

          shellHook = ''
            echo "Gridforge dev shell activated"
            echo "Run 'bun tauri dev' to start development"
          '';
        };
      }
    );
}
