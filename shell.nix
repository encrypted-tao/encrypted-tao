{ nixpkgs ? import <nixpkgs> {  } }:
 
let
  pkgs = [
    nixpkgs.rustup
    nixpkgs.clang
    nixpkgs.libiconv
    nixpkgs.darwin.apple_sdk.frameworks.Security
    nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
  ];
 
in
  nixpkgs.stdenv.mkDerivation {
    name = "env";
    buildInputs = pkgs;
  }
