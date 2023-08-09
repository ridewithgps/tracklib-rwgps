with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "tracklib-rwgps";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    ruby_3_1
  ];
  shellHook = ''
    mkdir -p .nix-gems
    export GEM_HOME=$PWD/.nix-gems
    export GEM_PATH=$GEM_HOME
    export PATH=$GEM_HOME/bin:$PATH
  '';
}
