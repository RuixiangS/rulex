<div align="center">

![Pomsky dog with orange glasses logo](https://raw.githubusercontent.com/pomsky-lang/pomsky/main/assets/logo.svg)

# Pomsky

A portable<sup><a href="#portability">1</a></sup>, modern regular expression language

**[documentation](https://pomsky-lang.org/docs/get-started/introduction/) · [website](https://pomsky-lang.org) · [Discord server](https://discord.gg/uwap2uxMFp)**

</div>

[![Test][test-badge]][test-link] [![Coverage Status][coverage-badge]][coverage-link]
[![Crates.io][crates-badge]][crates-link]

[test-badge]: https://github.com/pomsky-lang/pomsky/actions/workflows/test.yml/badge.svg
[test-link]: https://github.com/pomsky-lang/pomsky/actions/workflows/test.yml
[coverage-badge]: https://coveralls.io/repos/github/pomsky-lang/pomsky/badge.svg?branch=main
[coverage-link]: https://coveralls.io/github/pomsky-lang/pomsky?branch=main
[crates-badge]: https://img.shields.io/crates/v/pomsky
[crates-link]: https://crates.io/crates/pomsky

## Examples

On the left are Pomsky expressions, on the right is the compiled regex:

```py
# String
'hello world'                 # hello world

# Greedy repetition
'hello'{1,5}                  # (?:hello){1,5}
'hello'*                      # (?:hello)*
'hello'+                      # (?:hello)+

# Lazy repetition
'hello'{1,5} lazy             # (?:hello){1,5}?
'hello'* lazy                 # (?:hello)*?
'hello'+ lazy                 # (?:hello)+?

# Alternation
'hello' | 'world'             # hello|world

# Character classes
['aeiou']                     # [aeiou]
['p'-'s']                     # [p-s]

# Named character classes
[word] [space] [n]            # \w\s\n

# Combined
[w 'a' 't'-'z' U+15]          # [\wat-z\x15]

# Negated character classes
!['a' 't'-'z']                # [^at-z]

# Unicode
[Greek] U+30F Grapheme        # \p{Greek}\u030F\X

# Anchors, boundaries
^ $                           # ^$
% 'hello' !%                  # \bhello\B

# Non-capturing groups
'terri' ('fic' | 'ble')       # terri(?:fic|ble)

# Capturing groups
:('test')                     # (test)
:name('test')                 # (?P<name>test)

# Lookahead/lookbehind
(>> 'foo' | 'bar')            # (?=foo|bar)
(<< 'foo' | 'bar')            # (?<=foo|bar)
(!>> 'foo' | 'bar')           # (?!foo|bar)
(!<< 'foo' | 'bar')           # (?<!foo|bar)

# Backreferences
:('test') ::1                 # (test)\1
:name('test') ::name          # (?P<name>test)\1

# Ranges
range '0'-'999'               # 0|[1-9][0-9]{0,2}
range '0'-'255'               # 0|1[0-9]{0,2}|2(?:[0-4][0-9]?|5[0-5]?|[6-9])?|[3-9][0-9]?

# Inline regex
regex '[\w[^abg]]'            # [\w[^abg]]
```

## Variables

```rust
let operator = '+' | '-' | '*' | '/';
let number = '-'? [digit]+;

number (operator number)*
```

## Usage

Read **[the book][book]** to get started, or check out the [CLI program](./pomsky-bin/), the
[Rust library](./pomsky-lib/) and the [procedural macro](./pomsky-macro/).

## Why use this instead of normal regexes?

Normal regexes are very concise, but when they get longer, they get increasingly difficult to
understand. By default, they don't have comments, and whitespace is significant. Then there's the
plethora of sigils and backslash escapes that follow no discernible system:
`(?<=) (?P<>) .?? \N \p{} \k<> \g''` and so on. And with various inconsistencies between regex
implementations, it's the perfect recipe for confusion.

Pomsky solves these problems with a new, simpler but also more powerful syntax:

- It's not whitespace sensitive and allows comments
- Text must appear in quotes. This makes expressions longer, but also much easier to read
- Non-capturing groups are the default
- More intuitive, consistent syntax
- Variables to make expressions DRY

## Compatibility

Pomsky is currently compatible with PCRE, JavaScript, Java, .NET, Python, Ruby and Rust. The regex
flavor must be specified during compilation, so Pomsky can ensure that the produced regex works as
desired on the targeted regex engine.

**Note**: You should enable Unicode support in your regex engine, if it isn't enabled by default.
This is [explained here][enable-unicode].

## Portability

Pomsky aims to be as portable as possible, polyfilling Unicode and unsupported features where feasible. That said, there are some cases where portability is not possible:

- Some features (e.g. lookaround, backreferences, Unicode properties) aren't supported in every flavor. Pomsky fails to compile when you're using an unsupported feature.

- `\b` (word boundaries) are not Unicode aware in JavaScript. Pomsky therefore only allows word boundaries when Unicode is disabled.

- `\w` in .NET handles Unicode incorrectly, with no way to polyfill it properly. This means that in .NET, `[word]` only matches the `L`, `Mn`, `Nd`, and `Pc` general categories, instead of `Alphabetic`, `M`, `Nd`, `Pc` and `Join_Control`.

- In .NET, `.`, `Codepoint` and character classes (e.g. `[Latin]`) only match a single UTF-16 code unit rather than a codepoint.

- `[space]` matches slightly different code points in JavaScript than in Java. This will be fixed.

- Backreferences behave differently in JavaScript and Python when the referenced group has no captured text. There is nothing we can do about it, but we could add a warning for this in the future.

## Security

**Never compile or execute an untrusted Pomsky expression on your critical infrastructure**.
This may make you vulnerable for denial of service attacks, like the
[Billion Laughs attack][billion-lols].

[Read more][security]

## Diagnostics

Pomsky looks for mistakes and displays helpful diagnostics:

- It shows an error if you use a feature not supported by the targeted regex flavor
- It detects syntax errors and shows suggestions on how to resolve them
- It parses backslash escapes (which are not allowed in a Pomsky expression) and explains what to
  write instead
- It looks for likely mistakes and displays warnings
- It looks for patterns that can be very slow for certain inputs and are susceptible to
  Denial-of-Service attacks _(coming soon)_

## Comparison with other projects

I wrote an in-depth comparison with similar projects, which [you can find here][comparison].

## Code of Conduct

The Code of Conduct [can be found here](./CODE_OF_CONDUCT.md).

## Contributing

You can contribute by using Pomsky and providing feedback. If you find a bug or have a question,
please create an issue.

I also gladly accept code contributions. [More information](./CONTRIBUTING.md)

## Sponsor this project

[Go to sponsors page](https://github.com/sponsors/Aloso/)

## License

Dual-licensed under the [MIT license][mit-license] or the [Apache 2.0 license][apache-2-license].

[book]: https://pomsky-lang.org/docs/get-started/introduction/
[enable-unicode]: https://pomsky-lang.org/docs/get-started/enable-unicode/
[billion-lols]: https://en.wikipedia.org/wiki/Billion_laughs_attack
[security]: https://pomsky-lang.org/docs/reference/security/
[comparison]: https://pomsky-lang.org/docs/reference/comparison/
[mit-license]: https://opensource.org/licenses/MIT
[apache-2-license]: https://opensource.org/licenses/Apache-2.0
