with import <nixpkgs> {};
mkShell {
  buildInputs = [
    cargo
    rustc

    # Dev
    rls           # Language Server
    rustfmt
    rustPackages.clippy
  ];

  RUST_BACKTRACE = 1;
}
