with import <nixpkgs> {};
  mkShell {
    buildInputs = [
      cargo
      rustfmt
      clippy
      rust-analyzer

      rustc
      pkg-config
      gtk4
      gtk4-layer-shell
      libadwaita
    ];
  }
