{
  inputs = {
    # Pin to your stable channel
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

    fenix = {
      url = "github:nix-community/fenix";
      # Follow your stable nixpkgs to avoid pulling in unstable
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenixPkgs = fenix.packages.${system};
      in
      {
        devShells.default = pkgs.mkShell {

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
            with pkgs;
            [
              openssl
              libxcb
              # libX11
              # libXcursor
              # libXrandr
              # libXi
              wayland
              # wayland-protocols
              libxkbcommon
              libGL
              vulkan-loader
              # mesa
            ]

          );

          buildInputs = with pkgs; [
            openssl
            libxcb
            # Also add these common X11 deps that often come with libxcb:
            # libX11
            # libXcursor
            # libXrandr
            # libXi
            wayland
            # wayland-protocols
            libxkbcommon
            libGL
            vulkan-loader
            # mesa
          ];

          nativeBuildInputs = [
            # Combine only the components you need
            (fenixPkgs.combine [
              fenixPkgs.stable.cargo
              fenixPkgs.stable.clippy
              fenixPkgs.stable.rustc
              fenixPkgs.stable.rustfmt
              fenixPkgs.stable.rust-src # needed for rust-analyzer goto-def
            ])
            fenixPkgs.rust-analyzer
            pkgs.cargo-watch
            pkgs.cargo-edit
            pkgs.pkg-config
          ];

          # Help rust-analyzer find the stdlib source
          RUST_SRC_PATH = "${fenixPkgs.stable.rust-src}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = 1;
        };
      }
    );

}
