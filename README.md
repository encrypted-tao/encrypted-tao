# encrypted-tao
Simplified Implementation of Facebook's TAO, but encrypted

## Optional Instructions
1. Install `nix` if you want [https://nixos.org/download.html](https://nixos.org/download.html)
```
$ sh <(curl -L https://nixos.org/nix/install) --no-daemon
```

2. Install `just` if you want [https://github.com/casey/just](https://github.com/casey/just)

3. With this, run
```
$ just nix-build  // to build inside a nix-shell
$ just build      // if you already have rust/cargo locally
$ just clean      // to remove artifacts created by build
```

## Instructions
Use either `nix` and `just` to build, or if you have rust/cargo locally, then

1. Build the project
```
$ cargo build
```

2. Create some convenient symlinks (this is already done for you if you use `just`
```
$ ln -s ./target/debug/tao-interactive ./tao-interactive
$ ln -s ./target/debug/tao-cli ./tao-cli
$ ln -s ./target/debug/tao-server ./tao-server
```

4. Run the project
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

To run the TAO interactive program:
```
$ ./tao-interactive <host> <port>
```


## Stuff left to do:
- Implement query encryption/decryption 
- Implement caching mechanism
- Create more realistic and larger dataset (and visualize it)
- Implement/setup benchmarking suite
- Run evaluations
