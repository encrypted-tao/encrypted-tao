@nix-build:
  nix-shell --run "cargo build" 
  ln -s ./target/debug/tao-interactive ./tao-interactive
  ln -s ./target/debug/tao-cli ./tao-cli
  ln -s ./target/debug/tao-server ./tao-server

@nix-format:
  nix-shell --run "cargo fmt"

@format:
  cargo fmt

@build:
  cargo build
  ln -s ./target/debug/tao-interactive ./tao-interactive
  ln -s ./target/debug/tao-cli ./tao-cli
  ln -s ./target/debug/tao-server ./tao-server

@clean:
  rm ./tao-interactive ./tao-server ./tao-cli
