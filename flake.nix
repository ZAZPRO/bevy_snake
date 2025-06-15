{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        fenix_pkg = with fenix.packages.${system};
          combine [
            (stable.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            targets.wasm32-unknown-unknown.stable.rust-std
          ];
        libs = with pkgs; [
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXScrnSaver
            xorg.libXcursor
            xorg.libXext
            xorg.libXfixes
            xorg.libXi
            xorg.libXrandr
            libGL
            alsa-lib
            vulkan-headers
            mesa
        ];
        libPath = pkgs.lib.makeLibraryPath libs;
      in {
        nixpkgs.overlays = [fenix.overlays.default];
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            wayland
            alsa-lib
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            libxkbcommon
            libGL
            udev
            vulkan-headers
            mesa
          ];
          packages = with pkgs; [
            fenix_pkg
            rust-analyzer
            simple-http-server
            pkg-config
          ];

          LD_LIBRARY_PATH = libPath;
        };
      }
    );
}
