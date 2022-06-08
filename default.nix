with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "tracklib-rwgps";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    ruby_2_7
  ];
  shellHook = ''
    mkdir -p .nix-gems
    export GEM_HOME=$PWD/.nix-gems
    export GEM_PATH=$GEM_HOME
    export PATH=$GEM_HOME/bin:$PATH
  '';
}
