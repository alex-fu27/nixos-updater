{
	inputs = {
			nixpkgs.url = "nixpkgs/nixos-23.11";
			utils.url = "github:numtide/flake-utils";
			naersk.url = "github:nix-community/naersk";
			fenix = {
				url = "github:nix-community/fenix";
				inputs.nixpkgs.follows = "nixpkgs";
			};
		};

	outputs = { self, nixpkgs, utils, naersk, fenix }: 
		utils.lib.eachDefaultSystem (system: let
			pkgs = import nixpkgs { inherit system; };
			naersk = pkgs.callPackage naersk {};
		in {
			packages.default = naersk.buildPackage ./daemon ;
			devShells.default = with pkgs; mkShell {
				buildInputs = [
					fenix.packages.${system}.latest.toolchain pre-commit
					wrapGAppsHook4
					meson
					gtk4
					desktop-file-utils
					ninja
					libadwaita
					libsecret
					pkg-config dbus ];
				RUST_SRC_PATH = rustPlatform.rustLibSrc;
			};
		});
}

