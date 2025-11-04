```powershell
8888888b.  d8b                            888888b.
888   Y88b Y8P                            888  "88b
888    888                                888  .88P
888   d88P 888 88888888 88888888 888  888 8888888K.   .d88b.  888  888
8888888P"  888    d88P     d88P  888  888 888  "Y88b d88""88b `Y8bd8P'
888 T88b   888   d88P     d88P   888  888 888    888 888  888   X88K
888  T88b  888  d88P     d88P    Y88b 888 888   d88P Y88..88P .d8""8b.
888   T88b 888 88888888 88888888  "Y88888 8888888P"   "Y88P"  888  888
                                      888
                                 Y8b d88P
                                  "Y88P"
```

A multicall coreutil binary written in Rust.

## Usage

### Debug scratch or distroless container images

1. Locate your Docker plugins directory. This is usually in
   `~/.docker/cli-plugins` or `/usr/lib/docker/cli-plugins`. If you are unsure,
   you can inspect the output of `docker info` or use the following one-liner:

```sh
dirname $(docker info | grep -iA3 Plugins | grep Path | awk '{ print $2 }' | head -n1)
```

2. Symlink Rizzybox to `"$docker_plugins_dir/docker-debug"` (or
   `"$docker_plugins_dir/docker-rebug"`, if you're already using `docker-debug`)

```sh
sudo ln -sf $(which rizzybox) "$docker_plugins_dir/docker-debug"
```

3. Use the Rizzybox shell to get an interactive session into a minimal container
   image:

```sh
docker debug tangowithfoxtrot/scratch

# or, if you symlinked it as docker-rebug:
docker rebug tangowithfoxtrot/scratch
```

## Goals

1. Do something fun in Rust.
2. Learn more about `coreutils`.
3. Provide fancier alternatives to GNU coreutils with things like
   syntax-highlighting and robust completions.
4. Create original coreutils. See the [stem](./src/command/stem.rs) command for
   an example.

### Acknowledgements

- [bubble-shell](https://github.com/JoshMcguigan/bubble-shell); for
  demonstrating basic shell logic
- [Bat](https://github.com/sharkdp/bat); enables niceties like
  syntax-highlighting in `cat`, `echo`, and others
