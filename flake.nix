{
  description = "TFHE-rs examples development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }: 
    let
      # Systems supported
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      
      # Helper function for creating system-specific packages
      forAllSystems = function:
        nixpkgs.lib.genAttrs systems (system: function system);

    in {
      # Development shells for each system
      devShells = forAllSystems (system:
        let
          # Apply the rust overlay to nixpkgs
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          
          # Specify Rust version
          rustVersion = pkgs.rust-bin.stable."1.86.0";
        in {
          default = pkgs.mkShell {
            name = "tfhe-rs-examples-dev";
            buildInputs = [
              # Rust with specific version and components
              (rustVersion.default.override {
                extensions = [ "rust-src" "rust-analyzer" ];
              })
              
              # Just command runner
              pkgs.just
              
              # Useful development tools
              pkgs.pkg-config
              pkgs.openssl.dev
            ];
            
            # Set environment variables if needed
            RUST_SRC_PATH = "${rustVersion.rust-src}/lib/rustlib/src/rust/library";
            
            shellHook = ''
              echo "TFHE-rs examples development environment loaded"
              echo "Rust version: $(rustc --version)"
              echo "Just version: $(just --version)"
              echo ""
              echo "Available just commands:"
              just --list
            '';
          };
        }
      );
      
      # Make the devShell available as a package as well
      packages = forAllSystems (system: {
        default = self.devShells.${system}.default.inputDerivation;
      });
      
      # Overlay for other projects to use
      overlays.default = final: prev: {};
    };
} 