{ nixpkgs ? import <nixpkgs> {  } }:
 
let
  pkgs = [
    nixpkgs.rustup
    nixpkgs.clang
    nixpkgs.libiconv
    nixpkgs.openssl_1_1
    nixpkgs.pkg-config
    nixpkgs.just
  ];
 
in
  nixpkgs.stdenv.mkDerivation {
    name = "env";
    buildInputs = pkgs;
  }
