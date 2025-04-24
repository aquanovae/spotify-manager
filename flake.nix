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
      version = "1.0.2";
      src = ./.;
      useFetchCargoVendor = true;
      cargoHash = "sha256-DpI8eOv6BiLn94FOE30CNSdNQrHeO9QnTlxbEQHanH8=";
    };
    devShells.${system}.default = pkgs.stdenv.mkDerivation {
      inherit buildInputs nativeBuildInputs;
      name = "rust";
    };
  };
}
