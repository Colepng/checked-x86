{
  pkgs ? import (fetchTarball "https://nixos.org/channels/nixpkgs-unstable/nixexprs.tar.xz") { },
}:
pkgs.mkShell {
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = with pkgs.buildPackages; [
    (haskellPackages.liquid-fixpoint_0_9_6_3_5.overrideAttrs (_: {
      doCheck = false;
    }))
    z3
    rustup
  ];
}
