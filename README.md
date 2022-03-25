Papyrust is a *very basic* Rust script runner aimed at near zero launch latency. It was written mainly because other such runners I used, which I shall not name, all have at least 100 ms delay before launching the actual script, presumably due to invoking `cargo build` without even checking for changes in source code.

How to use:

- Install Papyrust (make sure that `$HOME/.cargo/bin` is in your `PATH`):
  ```sh
  cargo install papyrust
  ```
- Create a Cargo project for your Rust script, let's call it `x` for simplicity:
  ```sh
  cargo init x
  ```
- In the project directory create a shell script named `x.pp` with Papyrust shebang and make it executable:
  ```sh
  cd x
  echo "#\!/usr/bin/env papyrust > x.pp'
  chmod +x x.pp
  ```

Now you can simply launch your script directly
```sh
./x.pp
```
or put it in your `$HOME/bin`, `/usr/local/bin` or whatever
```sh
ln -s /home/coolhacker/devel/x/x.pp ~/bin
```
if you want to be cool like me and launch it from anywhere.

And the whole point of Papyrust is this:
```sh
# First time
time x
   Compiling x v0.1.0 (/home/coolhacker/devel/x)
    Finished release [optimized] target(s) in 0.40s
x  0,34s user 0,08s system 80% cpu 0,515 total

# From now on, until you change the source code
time x
x  0,00s user 0,00s system 86% cpu 0,003 total
```

When I said "very basic" in the beginning of this very serious document, that wasn't a joke.

1. Your script must be a full-fledged Cargo project, even if it does absolutely nothing.
1. The script must be inside your project directory - that's how Papyrust determines where the project is.
1. The script's base name must match the name of the binary that Cargo produces. That is, if the binary is called `supermegafancyemptyproject`, you're stuck with `supermegafancyemptyproject.pp` inside your `supermegafancyemptyproject` project directory. Yeah. Actually, the extension doesn't matter, since Papyrust ignores it, so you don't have to insert `pp` into your every script name.
1. Papyrust only builds your project if any file in `src` subdirectory is newer than the binary produced by cargo, or if the binary is missing. So if you put some extra files in `src` after the project was build, e.g. unused modules, the project will be recompiled every time you launch your script. You see, Cargo is too smart and caches the binary along with its last modification time, which will be in the past, because the relevant sources haven't changed, and I don't really want to parse all the `mod` declarations in your project to figure out which of the sources are relevant.
1. It only compiles your script in release mode.

So, basically, it just barely works, is aimed exclusively at people too lazy to run `cargo build --release` or `cargo install --path .` themselves and, obviously, is not the best example of state-of-the-art idiomatic Rust code.

Good luck 😄
