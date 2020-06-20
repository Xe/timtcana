{ pkgs ? import <nixpkgs> { }, sources ? import ./sources.nix, stdenv ? pkgs.stdenv }:

stdenv.mkDerivation rec {
  name = "armv7l-linux-musleabihf-cross";
  src = sources.gcc-thal;
  phases = "buildPhase installPhase";
  buildPhase = ''
    tar xzf $src
    mv armv7l-linux-musleabihf-cross/* .
  '';
  installPhase = ''
    mkdir -p $out
    cp -vrf * $out
  '';
}
