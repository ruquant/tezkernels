{
  pkgs,
  nix-filter,
}:
with pkgs;
with ocamlPackages;
  buildDunePackage {
    pname = "outbox-daemon";
    version = "0.0.1";
    src = nix-filter {
      root = ../.;
      include = [
        "outbox-daemon"
        "dune"
        "dune-project"
        "outbox-daemon.opam"
      ];
    };
    duneVersion = "3";

    propagatedBuildInputs = with ocamlPackages; [
      tezos-alpha.smart-rollup-client
      tezos-alpha.protocol
      tezos-base
      tezos-clic
    ];

    checkInputs = with ocamlPackages; [
      ppx_expect
    ];
  }
