{
  inputs.flake-compat = { url = "github:edolstra/flake-compat"; flake = false; };
  outputs = { self, nixpkgs, flake-compat }: {
    devShells.x86_64-linux.default = with import nixpkgs { system = "x86_64-linux"; };  mkShell {
      buildInputs = [
        openssl
        pkg-config
        rust-analyzer
        rustup
        trunk
        wasm-bindgen-cli
      ];
    };
  };
}
