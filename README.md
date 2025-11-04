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

### As a multicall binary

Rizzybox consists of a handful of coreutil implementations, as well as a few
original coreutils (see the [stem](./src/command/stem.rs) command for an
example). Each one can be executed with `rizzybox <util-name>`. For a list of
supported commands, you can run `rizzybox --help` or `rizzybox --list`.

### As rudimentary coreutils

Rizzybox can be symlinked or "installed" in such a way that enables you to
execute the various supported coreutils without needing to invoked them as
`rizzybox <util-name>`. This allows you to simply run `cat` to invoke
`rizzybox cat`. To do so, simply run
`rizzybox --install-self <path-to-desired-installation-dir>`. The installation
dir must be in `$PATH` to be used this way, and you will need write permissions
to that directory or invoke it with `sudo` or `doas`.

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
   syntax-highlighting and robust completions. For now, perfect GNU
   compatibility is not a goal. In the future, this may change, but if you would
   like a Rust coreutil project that _does_ strive for GNU compatibility, please
   check out [uutils](https://github.com/uutils/coreutils).
4. Create original coreutils.

### Acknowledgements

- [bubble-shell](https://github.com/JoshMcguigan/bubble-shell); for
  demonstrating basic shell logic
- [Bat](https://github.com/sharkdp/bat); enables niceties like
  syntax-highlighting in `cat`, `echo`, and others
