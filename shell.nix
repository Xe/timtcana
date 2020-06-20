let
  pkgs = import <nixpkgs> { };
  gcc-thal = pkgs.callPackage ./nix/gcc-arm-thal.nix { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup gcc-thal
  ];

  CC="armv7l-linux-musleabihf-gcc";
}
