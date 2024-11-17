## Unreleased (72acd77..4fc5aca)
#### Bug Fixes
- **(basename)** actually print the NUL char - (e0526a6) - tangowithfoxtrot
- **(clear)** didn't work on some terms without the newline char - (d299842) - tangowithfoxtrot
- **(echo)** correctly parse backslash and hex escapes - (78fe7f3) - tangowithfoxtrot
- **(env)** `env` command should be able to actually set env vars :) - (6ac9e21) - tangowithfoxtrot
- **(env)** rm useless if condition - (7237714) - tangowithfoxtrot
- **(sh)** shell should render an additional newline before the prompt - (c447e27) - tangowithfoxtrot
- **(sh)** prompt should render to stderr - (b3eaa20) - tangowithfoxtrot
- **(sh)** crash when `exit invalid-arg` - (3259c69) - tangowithfoxtrot
- **(sh)** sh not installable - (c0e220a) - tangowithfoxtrot
- **(sh)** just use libc::geteuid() - (6b0c36d) - tangowithfoxtrot
- **(uname)** just cheat the OS name; this is _usually_ correct - (4fd7ec1) - tangowithfoxtrot
- **(uname)** match GNU uname -s for kernel arg - (261f92c) - tangowithfoxtrot
- **(which)** panic in very weird situation in which PATH is missing, but shell somehow works; this occurs in environments like: `env -i -- sh` - (22182a3) - tangowithfoxtrot
- try just downloading all of the artifacts - (4fc5aca) - tangowithfoxtrot
- panicking test on Linux - (2892e4c) - tangowithfoxtrot
- linking errors for riscv target - (70a9e4c) - tangowithfoxtrot
- this clone() is no longer needed - (27e0ce3) - tangowithfoxtrot
- print the actual OS name; only print kernel when called - (ca913ce) - tangowithfoxtrot
- don't append a space to the last output - (5daae05) - tangowithfoxtrot
- `std::fs::exists` doesn't work for MUSL targets, apparently - (8c11c89) - tangowithfoxtrot
- use executable dir, not current user dir - (95b1a88) - tangowithfoxtrot
- only use color if output is a TTY - (72acd77) - tangowithfoxtrot
#### Features
- **(cat)** support stdin and `-` - (d4d7a69) - tangowithfoxtrot
- **(cat)** cat from stdin - (c15039b) - tangowithfoxtrot
- **(dirname)** add `dirname` - (69a36e3) - tangowithfoxtrot
- **(env)** add `--argv0` - (4fb4a3f) - tangowithfoxtrot
- **(env)** implement `chdir`, `ignore_env`, and `unset` - (86feed8) - tangowithfoxtrot
- **(expand)** partial implementation of `expand` - (e7a9d1e) - tangowithfoxtrot
- build workflow - (616fb55) - tangowithfoxtrot
- questionable implementation of ls - (8736f67) - tangowithfoxtrot
- container image now contains an interactive shell - (7afce7b) - tangowithfoxtrot
- add a very basic shell - (dcb9a84) - tangowithfoxtrot
- add Dockerfile - (1720e90) - tangowithfoxtrot
- `--install-with-sudo` - (b052884) - tangowithfoxtrot
#### Miscellaneous Chores
- add acknowledgements and a little more color - (6375025) - tangowithfoxtrot
- remove non-goal; we're makin' a shell - (720a5b9) - tangowithfoxtrot
- add sensible aliases for args - (ec2ed1a) - tangowithfoxtrot
#### Refactoring
- anyhow is what the cool kids use, i think - (d689eea) - tangowithfoxtrot
- try to make imports less icky - (5966e95) - tangowithfoxtrot
#### Tests
- add pre-commit hook - (997db35) - tangowithfoxtrot
- add very basic tests - (c552da6) - tangowithfoxtrot


