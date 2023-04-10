# encrypted-tao
Simplified Implementation of Facebook's TAO, but encrypted

## Setup Instructions
1. Install nix [https://nixos.org/download.html](https://nixos.org/download.html)
```
$ sh <(curl -L https://nixos.org/nix/install) --no-daemon
```

2. Launch `nix-shell`:
```
$ nix-shell
```

3. Build the project
```
$ cargo build
```

4. Run the project
To run the TAO server:
```
$ ./tao-server
```

To run the TAO CLI:
```
$ ./tao-cli
```
