{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix";
    cargo2nix.inputs.nixpkgs.follows = "nixpkgs";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.61.0";
          packageFun = import ./Cargo.nix;
          extraRustComponents = [ "clippy" "rustfmt" ];
        };

        workspaceShell = rustPkgs.workspaceShell {
          packages = with pkgs; [
            # CI.
            cargo-audit
            cargo-auditable
            cargo-deny
            codespell
            eclint

            # Development.
            cargo-expand
            cargo-watch
          ];
        };
      in rec {
        devShell = workspaceShell;

        packages = {
          cli = (rustPkgs.workspace.cargo-free {}).bin;
          lib = (rustPkgs.workspace.cargo-free {});
        };
        default = packages.cli;
      }
    );
}
