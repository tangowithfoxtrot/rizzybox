```
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

A simple BusyBox clone written in Rust.

Things like the `cat` and `echo` commands are mostly just a wrapper around the `bat` crate for now.

## Goals

1. Do something fun in Rust
2. Learn more about `coreutils`
3. Provide fancier alternatives to GNU coreutils with things like syntax-highlighting

## Non-goals

1. Be a full, drop-in replacement for BusyBox. Writing a shell is a lot of work :sweat_smile:
