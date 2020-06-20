let
  pkgs = import <nixpkgs> { };
  gcc-thal = pkgs.callPackage ./nix/gcc-arm-thal.nix { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup gcc-thal
  ];
}
