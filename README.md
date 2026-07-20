# jaqi

![Build status](https://github.com/vlasky/jaqi/actions/workflows/check.yml/badge.svg)
[![Rust 1.70+](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

jaqi is a fast, `jq`-compatible JSON processor.
It is a fork of [jaq](https://github.com/01mf02/jaq),
Michael Färber's reimplementation of [`jq`](https://jqlang.github.io/jq/).
Like jaq, it can be used as a drop-in replacement for `jq`, and
it supports the data formats YAML, CBOR, TOML, and XML.

This fork tracks jaq and adds a set of
correctness, safety, and performance fixes that are
maintained here (see [Changes relative to jaq](#changes-relative-to-jaq)).
For everything else, jaqi behaves like jaq;
jaq's [manual](https://gedenkt.at/jaq/manual/) and
[README](https://github.com/01mf02/jaq) document the shared feature set.


## Relationship to jaq

jaqi exists to carry a handful of improvements that were
developed against jaq and are maintained independently in this repository.
It is not affiliated with or endorsed by the jaq project.
jaq remains the upstream, and all credit for the design and
the overwhelming majority of the code belongs to
Michael Färber and the jaq contributors.

If you want the original, use [jaq](https://github.com/01mf02/jaq).
If you want these specific changes, jaqi is here for you.


## Changes relative to jaq

**Correctness and safety**

- Three arithmetic operations that could abort or panic the process on
  ordinary input no longer do: overflowing string repetition
  (`"ab" * 9223372036854775807`), `length` of `isize::MIN`, and
  converting a very large epoch to a timestamp.
- The CBOR and XML readers no longer overflow the stack on deeply nested
  input; nesting beyond a fixed depth now yields a normal, catchable
  parse error instead of aborting.
- The bundled web playground escapes error messages and reflected filter
  source, closing a cross-site-scripting hole.

**Performance**

- Decimal numbers cache their parsed value, roughly halving the time to
  sort large arrays of floats.
- Decimal literals in a program are parsed once at compile time rather
  than on every evaluation.
- Output is flushed per value only when writing to a terminal, which
  greatly speeds up piped and redirected output (an `--unbuffered` flag
  restores per-value flushing for `jq` parity).
- HTML/CSV/TSV escaping uses simple byte loops instead of building an
  Aho-Corasick automaton per call, speeding up `@html`, `@csv`, and
  `@tsv` by one to two orders of magnitude and dropping a dependency.


## Installation

### From source

You need a Rust toolchain; see <https://rustup.rs/>.

    $ cargo install --locked --git https://github.com/vlasky/jaqi jaqi

This places the executable at `~/.cargo/bin/jaqi`.

If you have cloned this repository, you can also build it directly:

    $ cargo build --release            # places binary into target/release/jaqi
    $ cargo install --locked --path jaqi

### Homebrew

    $ brew install vlasky/tap/jaqi

### Binaries

Prebuilt binaries for Linux, macOS, and Windows are on the
[releases page](https://github.com/vlasky/jaqi/releases).


## License

jaqi is licensed under the MIT license, the same as jaq; see
[`LICENSE-MIT`](LICENSE-MIT). It is a fork of
[01mf02/jaq](https://github.com/01mf02/jaq), copyright Michael Färber and
the jaq contributors, with modifications copyright Vlad Lasky.

The jaq project's security audits, fuzzing infrastructure, and funding
(from [NLnet](https://nlnet.nl/)) belong to jaq. jaqi builds on that
audited core and adds the changes listed above; the additions have not
been separately audited.
