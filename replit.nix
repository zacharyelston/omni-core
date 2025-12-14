{ pkgs }: {
  deps = [
    # Rust toolchain
    pkgs.rustc
    pkgs.cargo
    pkgs.rust-analyzer
    pkgs.clippy
    pkgs.rustfmt

    # Build dependencies
    pkgs.openssl
    pkgs.pkg-config
  ];

  env = {
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  };
}
