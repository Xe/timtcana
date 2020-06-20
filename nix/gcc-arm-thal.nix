{ pkgs ? import <nixpkgs> { }, sources ? import ./sources.nix
, stdenv ? pkgs.stdenv, fetchurl ? pkgs.fetchurl }:

stdenv.mkDerivation rec {
  name = "armv7l-linux-musleabihf-cross";
  src = fetchurl {
    url = "http://musl.cc/arm-linux-musleabihf-cross.tgz";
    sha256 = "0pqbimgw4kxd2an46absx5h36pi5i8l09g3v8qdj8z53d8j2v69r";
  };
  phases = "buildPhase installPhase";
  buildPhase = ''
    tar xzf $src
    ls -la
    mv arm-linux-musleabihf-cross/* .
  '';
  installPhase = ''
    mkdir -p $out
    cp -vrf * $out
  '';
}
