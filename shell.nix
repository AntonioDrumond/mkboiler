{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustc
    rustfmt
    cargo
    clippy
    gcc
  ];

  shellHook = ''
    echo ""
  '';

}
