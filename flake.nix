{
  description = "Cli helper for spotify playlists";


  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";


  outputs = { nixpkgs, ... }: let

    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };

    buildInputs = with pkgs; [
      openssl
    ];
    nativeBuildInputs = with pkgs; [
      cargo
      cargo-edit
      rustc
      pkg-config
    ];

  in {

    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      pname = "spotify-manager";
      version = "1.0.0";
      src = ./.;

      inherit buildInputs nativeBuildInputs;

      useFetchCargoVendor = true;
      cargoHash = "";
    };


    devShells.${system}.default = pkgs.stdenv.mkDerivation {
      name = "rust";

      inherit buildInputs nativeBuildInputs;
    };
  };
}
