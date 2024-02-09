{
	inputs = {
			nixpkgs.url = "nixpkgs/nixos-23.11";
			utils.url = "github:numtide/flake-utils";
			naersk.url = "github:nix-community/naersk";
		};

	outputs = { self, nixpkgs, utils, naersk }: 
		utils.lib.eachDefaultSystem (system: let
			pkgs = import nixpkgs { inherit system; };
			naersk = pkgs.callPackage naersk {};
		in {
			packages.default = naersk.buildPackage ./daemon ;
			devShells.default = with pkgs; mkShell {
				buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy pkg-config dbus ];
				RUST_SRC_PATH = rustPlatform.rustLibSrc;
			};
		});
}

