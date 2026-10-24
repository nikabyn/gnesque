{pkgs ? import <nixpkgs> {}}:
pkgs.writeShellApplication {
  name = "gnesque-shell";

  runtimeInputs = [];

  text = ''
    export LD_LIBRARY_PATH="${
      pkgs.lib.makeLibraryPath [
        pkgs.wayland
        pkgs.vulkan-loader
      ]
    }''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

    exec ~/dev/gnesque/target/release/gnesque-shell "$@"
  '';
}
