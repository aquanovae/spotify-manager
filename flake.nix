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
      inherit buildInputs nativeBuildInputs;
      pname = "spotify-manager";
      version = "1.0.1";
      src = ./.;
      useFetchCargoVendor = true;
      cargoHash = "sha256-axy2UO3YmtuukMxjX8lEHFA0vYjO5JJJuJHQI81bG3g=";
    };
    devShells.${system}.default = pkgs.stdenv.mkDerivation {
      inherit buildInputs nativeBuildInputs;
      name = "rust";
    };
  };
}
