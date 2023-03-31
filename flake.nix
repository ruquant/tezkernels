{
  nixConfig = {
    extra-substituters = ["https://tezos.nix-cache.workers.dev"];
    extra-trusted-public-keys = ["tezos-nix-cache.marigold.dev-1:4nS7FPPQPKJIaNQcbwzN6m7kylv16UCWWgjeZZr2wXA="];
  };

  inputs = {
    nixpkgs.follows = "tezos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    nix-filter.url = "github:numtide/nix-filter";
    rust-overlay.url = "github:oxalica/rust-overlay";
    tezos.url = "github:marigold-dev/tezos-nix/d4hines/smart-rollup-libs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    nix-filter,
    rust-overlay,
    tezos,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default tezos.overlays.trunk];
      };
      outbox-daemon = import ./nix/outbox-daemon.nix {
        inherit pkgs;
        nix-filter = nix-filter.lib;
      };
    in {
      packages = {inherit outbox-daemon;};
      devShell = let
       packageLibDirs =
        builtins.filter builtins.pathExists (
          builtins.map (package: "${package}/lib/${package.pname}") packages
        );
       packageIncludeArgs = builtins.map (dir: "-I${dir}") packageLibDirs;

        mkFrameworkFlags = frameworks:
          pkgs.lib.concatStringsSep " " (
            pkgs.lib.concatMap
            (
              framework: [
                "-F${pkgs.darwin.apple_sdk.frameworks.${framework}}/Library/Frameworks"
                "-framework ${framework}"
              ]
            )
            frameworks
          );

        NIX_LDFLAGS = pkgs.lib.optional pkgs.stdenv.isDarwin (
          mkFrameworkFlags [
            "CoreFoundation"
            "IOKit"
            "AppKit"
          ]
        );
        NIX_CFLAGS_COMPILE =
          # Silence errors (-Werror) for unsupported flags on MacOS.
          pkgs.lib.optionals
          pkgs.stdenv.isDarwin
          ["-Wno-unused-command-line-argument"]
          ++
          # Make sure headers files are in scope.
          packageIncludeArgs;
      in
        pkgs.mkShell {
          inputsFrom = [outbox-daemon];
          shellHook = ''
            export NEX_LDFLAGS=${NIX_LDFLAGS}
            export NIX_CFLAGS_COMPILE=${NIX_CFLAGS_COMPILE}
            export CC=$(which clang)
          '';
          packages = with pkgs; [
            alejandra
            rustfmt
            rust-analyzer
            wabt
            clang
            tezos.packages.${system}.trunk-octez-smart-rollup-wasm-debugger

            cargo-make

            # MDX dependencies
            ocaml
            dune_3
            ocamlPackages.mdx

            (rust-bin.stable."1.66.0".default.override {
              targets = ["wasm32-unknown-unknown"];
            })
          ];
        };
    });
}
