# encrypted-tao
Simplified Implementation of Facebook's TAO, but encrypted

## Set up
1. Install `nix` if you want [https://nixos.org/download.html](https://nixos.org/download.html) (Recommended)
```
$ sh <(curl -L https://nixos.org/nix/install) --no-daemon
```
Otherwise use [`rustup`](https://github.com/rust-lang/rustup) to install the rust toolchain.

To test this on an M1 MacOS, add the following to the list of `nixpkgs` in `shell.nix`:
```
nixpkgs.darwin.apple_sdk.frameworks.Security
nixpkgs.darwin.apple_sdk.frameworks.SystemConfiguration
```
One of our dependencies related to cryptography only works on x86 machines. On an M1 Mac, you can probably
only test the `eval-nonencrypted` branch.

2. `cd` into the project directory (where `shell.nix` resides) and run
```
$ nix-shell
```
Then, inside the nix-shell set up the rust toolchain
```
$ rustup install stable
$ rustup default stable
```

3. With this, still inside the nix-shell, you can now run
```
$ just nix-build  // to build inside a nix-shell
$ just build      // if you already have rust/cargo locally
$ just clean      // to remove artifacts created by build
```

## Instructions
Use either `nix` and `just` to build, or if you have rust/cargo locally, then
1. Enter nix shell environment:
```
$ nix-shell
```

2. Build the project inside it
```
$ just build
```

3. Run the project

Make sure your .env is setup:
```
DATABASE_HOST=
DATABASE_PORT_NUM=
DATABASE_NAME=
DATABASE_USERNAME=
DATABASE_PASSWORD=
SERVER_ADDR=
SERVER_PORT=
```

To run the TAO server:
```
$ ./tao-server /path/to/.env
```

To run a query using the TAO CLI:
```
$ ./tao-cli --help  // for instructions
$ ./tao-cli "[your query]"
```

To run the TAO interactive client:
```
$ ./tao-interactive <host> <port>
```
