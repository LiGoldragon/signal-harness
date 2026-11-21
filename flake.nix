{
  description = "The subscription every Harness Nexus serves: Subscribe, Subscribed and Changed.";
  inputs.base.url = "github:LiGoldragon/build-base";
  inputs.ethos.url = "github:LiGoldragon/ethos-zero";
  outputs = { self, base, ethos }: base.lib.workspaceOutputs {
    cargoNix = ./Cargo.nix;
    extraChecks = { pkgs, toolchain, ... }: {
      fmt = pkgs.runCommand "signal-harness-fmt" { nativeBuildInputs = [ toolchain ]; } ''
        rustfmt --edition 2024 --check ${./src}/lib.rs
        touch "$out"
      '';
      generated = pkgs.runCommand "signal-harness-generated" {
        nativeBuildInputs = [ ethos.packages.${pkgs.stdenv.hostPlatform.system}.ethos-zero ];
      } ''
        ethos-zero "Generate.{ ${./ethos}/signal.ethos $PWD/generated }"
        diff -u ${./src/generated/signal.rs} generated/signal.rs
        touch "$out"
      '';
    };
    memberChecks = { check, ... }: {
      signal-harness = check "signal-harness" [ "datom" ];
    };
  };
}
