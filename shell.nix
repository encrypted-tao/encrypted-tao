{ nixpkgs ? import <nixpkgs> {  } }:
 
let
  pkgs = [
    nixpkgs.rustup
    nixpkgs.clang
    nixpkgs.libiconv
  ];
 
in
  nixpkgs.stdenv.mkDerivation {
    name = "env";
    buildInputs = pkgs;
  }
