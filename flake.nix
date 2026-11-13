{
  description = "land-rs: convex-optimisation rocket landing guidance (G-FOLD) and Bevy simulation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, nixpkgs, fenix }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);

      perSystem =
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          # Honours rust-toolchain.toml (pinned nightly).
          rustToolchain = fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-AOIpGWS97DZfKgz3LM9rtfVwGcQ9mkaTRFJpkLZ5zyQ=";
          };

          # `cargo patch-crate` — applies patches/ to target/patch per
          # [workspace.metadata.patch] in Cargo.toml. Not in nixpkgs.
          patch-crate = pkgs.rustPlatform.buildRustPackage {
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
            pname = "patch-crate";
            version = "0.1.13";
            src = pkgs.fetchCrate {
              pname = "patch-crate";
              version = "0.1.13";
              hash = "sha256-uRvsZuKlnkaeJ5uJFpdhF1wp2rvJ/QSzS5bhLKlndxw=";
            };
            cargoHash = "sha256-H6c+MfXd2RfRmkYIVBmYJDT4RvBlNLT9twEkMhHjA3U=";
          };

          # Typst `@preview` packages pinned in docs/*.typ, vendored because the
          # build sandbox has no network (typst would otherwise download them).
          # tar instead of fetchzip: the tarballs have multiple root entries.
          fetchTypstPackage =
            p:
            pkgs.runCommand "${p.name}-${p.version}" { } ''
              mkdir -p $out
              tar -xzf ${
                pkgs.fetchurl {
                  url = "https://packages.typst.org/preview/${p.name}-${p.version}.tar.gz";
                  hash = p.hash;
                }
              } -C $out
            '';

          typstPackagePath = pkgs.linkFarm "typst-preview-packages" (
            map (p: {
              name = "preview/${p.name}/${p.version}";
              path = fetchTypstPackage p;
            }) [
              { name = "cetz"; version = "0.3.4"; hash = "sha256-T0tajTEdUZ50mUCnZv5QUh5AwEESnhyRrwxC5h8wdRQ="; }
              { name = "oxifmt"; version = "0.2.1"; hash = "sha256-FvrCkjAyxZcn3gHoTULKxF6HkNoo31bv/KSfTeQbCdk="; }
              { name = "glossarium"; version = "0.5.3"; hash = "sha256-SQobBEp64IgVnBF1s7jqut62C+fmmAhF/y0f46FuBVs="; }
              { name = "codly"; version = "1.2.0"; hash = "sha256-uFqx1JvdkgQ3d0wIppaImqF/mqq1sxhWVx8EA416Vik="; }
                { name = "codly-languages"; version = "0.1.1"; hash = "sha256-jn9ZoQsNVe57zfhec+uy2iQufPk5Ffz4jzWwwvk9Fr8="; }
                { name = "cheq"; version = "0.2.2"; hash = "sha256-XDVTg5XFh2I1TwAEVHAvu+oB29PCEVz3NGXe/GFWq3Q="; }
                { name = "equate"; version = "0.3.1"; hash = "sha256-g9rZo1/1HcHksg091lRomxs9/qP6numjScVOwuxkSEA="; }
                { name = "fletcher"; version = "0.5.7"; hash = "sha256-4dsOOcz3aAAF8Wa9pA7WvZ28we5OxGXPaCOOEcC8aZQ="; }
                { name = "showybox"; version = "2.0.4"; hash = "sha256-LUFeOK2FFDuwynpB9Hzcu/MYagAqnGN849987XElhY8="; }
              ]
          );

          # Typst pinned to 0.13.1: the docs' `@preview` packages (codly 1.2.0
          # etc.) predate the typst 0.14+/0.15 API; nixpkgs' typst is too new.
          typstAsset =
            {
              "x86_64-linux" = "typst-x86_64-unknown-linux-musl";
              "aarch64-linux" = "typst-aarch64-unknown-linux-musl";
              "aarch64-darwin" = "typst-aarch64-apple-darwin";
            }
            .${system};
          typst = pkgs.runCommand "typst-0.13.1" { } ''
            mkdir -p $out/bin
            tar -xJf ${
              pkgs.fetchurl {
                url = "https://github.com/typst/typst/releases/download/v0.13.1/${typstAsset}.tar.xz";
                hash = "sha256-fSFL/v/C5YXcQi0aCdKxRJaUISgejH9deEtl/Gm1Zz8=";
              }
            } -C $out/bin --strip-components=1 --wildcards '*/typst'
          '';

          # Runtime libs required by Bevy/wgpu (Wayland + X11 + audio + Vulkan).
          bevyLibs = with pkgs; [
            udev
            alsa-lib
            fontconfig
            libGL
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            wayland
          ];
        in
        {
          devShells.default = pkgs.mkShell {
            packages =
              [
                rustToolchain
                fenix.packages.${system}.rust-analyzer
                patch-crate
                typst
                pkgs.pkg-config
                # .cargo/config.toml sets linker = "clang" and fuse-ld=/usr/bin/mold;
                # neither exists on NixOS, so provide both and override the mold path.
                pkgs.clang
                pkgs.mold
              ]
              ++ bevyLibs;

            env = {
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath bevyLibs;
              # Overrides the absolute /usr/bin/mold from .cargo/config.toml.
              RUSTFLAGS = "-C link-arg=-fuse-ld=mold";
            };

            shellHook = ''
              # good_lp must be patched before the first build.
              if [ ! -d "$PWD/target/patch/good_lp-1.13.0" ]; then
                cargo patch-crate
              fi
            '';
          };

          packages.docs =
            pkgs.runCommand "land-rs-docs"
              {
                src = ./docs;
                nativeBuildInputs = [ typst ];
                FONTPATH = "${./crates/land_sim/assets/fonts}";
                TYPST_PACKAGE_PATH = "${typstPackagePath}";
              }
              ''
                export TYPST_FONT_PATHS="$FONTPATH"
                mkdir -p $out
                typst compile --root $src $src/main.typ $out/land-rs.pdf
              '';
        };
    in
    {
      devShells = forAllSystems (system: (perSystem system).devShells);
      packages = forAllSystems (system: (perSystem system).packages);
    };
}
