# wherewasi
A very simple commandline application that reads a single line and appends it to a file `/home/brian/.wherewasi`.

# Usage

Download the released binary here:
[wherewasi binary](https://github.com/poppingtonic/wherewasi/releases/download/0.1.0/wherewasi).

```bash
$ wherewasi 
Remember to do this when you restart your machine

 (Press CTRL+D when finished)

<enter your stuff and press enter to finish>
```

# Building 

You'll need [rust](https://github.com/rust-lang/rust). You can get it
using [rustup](https://rustup.rs).

```bash 
  $ git clone https://github.com/poppingtonic/wherewasi.git /path/to/wherewasi 
  $ cd /path/to/wherewasi 
  $ cargo build --release 
  $ ln -s /path/to/wherewasi/target/wherewasi ~/bin/wherewasi 
```
