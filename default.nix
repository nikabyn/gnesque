let
  pkgs = import <nixpkgs> {};
  wildStdenv = pkgs.useWildLinker pkgs.stdenv;
in
  pkgs.mkShell.override {stdenv = wildStdenv;} {
    nativeBuildInputs = with pkgs; [
      rustc
      cargo
      rustfmt
      clippy
      rust-analyzer
      wild
      watchexec

      pkg-config
      fontconfig
    ];

    buildInputs = with pkgs; [
      wayland
      wayland-protocols
      libxkbcommon
      vulkan-loader
      libpulseaudio
      linux-pam
    ];

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.vulkan-loader
      pkgs.wayland
    ];

    packages = with pkgs; [
      wayland-utils
      jq
    ];
  }
