{
  description = "Deterministic Zellij config and layout renderer from Yazelix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      mkPkgs = system: nixpkgs.legacyPackages.${system};
      configPackPackage =
        system: pkgs:
        let
          rustToolchain = fenix.packages.${system}.combine [
            fenix.packages.${system}.stable.cargo
            fenix.packages.${system}.stable.rustc
          ];
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
          source = pkgs.lib.cleanSourceWith {
            name = "yazelix-zellij-config-pack-source";
            src = ./.;
            filter =
              path: _type:
              let
                relativePath = pkgs.lib.removePrefix ((toString ./.) + "/") (toString path);
              in
              relativePath != "target"
              && !pkgs.lib.hasPrefix "target/" relativePath
              && relativePath != ".git"
              && !pkgs.lib.hasPrefix ".git/" relativePath;
          };
        in
        rustPlatform.buildRustPackage {
          pname = "yazelix_zellij_config_pack";
          version = "0.1.0";

          src = source;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.findutils ];

          installPhase = ''
            runHook preInstall

            renderer_bin="$(find target -type f -path '*/release/yazelix_zellij_config_pack' -perm -111 | head -n 1)"
            if [ -z "$renderer_bin" ]; then
              echo "could not find built yazelix_zellij_config_pack binary" >&2
              exit 1
            fi
            install -Dm755 "$renderer_bin" "$out/bin/yazelix_zellij_config_pack"
            mkdir -p "$out/share/yazelix_zellij_config_pack"
            cp -R layouts "$out/share/yazelix_zellij_config_pack/layouts"
            install -Dm644 config_metadata/zellij_layout_families.toml \
              "$out/share/yazelix_zellij_config_pack/config_metadata/zellij_layout_families.toml"
            install -Dm644 README.md "$out/share/doc/yazelix_zellij_config_pack/README.md"
            install -Dm644 LICENSE "$out/share/doc/yazelix_zellij_config_pack/LICENSE"

            runHook postInstall
          '';

          doInstallCheck = true;
          nativeInstallCheckInputs = [
            pkgs.coreutils
            pkgs.findutils
            pkgs.gnugrep
          ];
          installCheckPhase = ''
            runHook preInstallCheck

            test -x "$out/bin/yazelix_zellij_config_pack"
            test -f "$out/share/yazelix_zellij_config_pack/layouts/yzx_side.kdl"
            test -f "$out/share/yazelix_zellij_config_pack/layouts/yzx_side.swap.kdl"
            test -f "$out/share/yazelix_zellij_config_pack/layouts/fragments/swap_sidebar_open.kdl"
            test -f "$out/share/yazelix_zellij_config_pack/config_metadata/zellij_layout_families.toml"
            "$out/bin/yazelix_zellij_config_pack" --schema-version | grep -q '^1$'

            runHook postInstallCheck
          '';

          passthru = {
            rendererSchemaVersion = 1;
            layoutsPath = "share/yazelix_zellij_config_pack/layouts";
            layoutFamiliesPath = "share/yazelix_zellij_config_pack/config_metadata/zellij_layout_families.toml";
          };

          meta = {
            description = "Deterministic Zellij config and layout renderer from Yazelix";
            homepage = "https://github.com/luccahuguet/yazelix-zellij-config-pack";
            license = pkgs.lib.licenses.asl20;
            mainProgram = "yazelix_zellij_config_pack";
          };
        };
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = mkPkgs system;
          configPack = configPackPackage system pkgs;
        in
        {
          default = configPack;
          yazelix_zellij_config_pack = configPack;
          yazelix-zellij-config-pack = configPack;
        }
      );

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.yazelix_zellij_config_pack}/bin/yazelix_zellij_config_pack";
        };
        yazelix_zellij_config_pack = {
          type = "app";
          program = "${self.packages.${system}.yazelix_zellij_config_pack}/bin/yazelix_zellij_config_pack";
        };
      });

      checks = forAllSystems (system: {
        yazelix_zellij_config_pack = self.packages.${system}.yazelix_zellij_config_pack;
      });
    };
}
