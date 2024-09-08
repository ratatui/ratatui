{pkgs ? import <nixpkgs> {}, ...}:
pkgs.mkShell {
  buildInputs = with pkgs.buildPackages; [
    pkg-config
    rust-bin.stable.latest.default
    llvmPackages_latest.bintools
  ];

  RUSTFLAGS = builtins.map (a: ''-L ${a}/lib'') [
    pkgs.llvmPackages_latest.bintools
  ];
  RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain.toml;
}
