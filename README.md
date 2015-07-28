# rust-caliburn [![Build Status][ci-build-stat]][ci-link] [![Coverage Status][cov-stat]][cov-link]
A strictly compliant parser for the RFC2812 (IRC Client Protocol) specification,
written in Rust. Currently targets Rust 1.1 and above.

## Notice
rust-caliburn is currently not production ready and should be considered
unstable. It is also not feature complete.

### Performance
rust-caliburn uses [rust-peg][rust-peg] as its parser backend, which implements
PEG but does not currently support memoization - as a result, parts of the
parsing, especially splitting into space-delimited parameters, have run times that grow very quickly:
```
test tests::line_parser::benchmarks::bench_complex_parse   ... bench:       5,307 ns/iter (+/- 517)
test tests::line_parser::benchmarks::bench_ludicrous_parse ... bench:      16,464 ns/iter (+/- 3,608)
test tests::line_parser::benchmarks::bench_moderate_parse  ... bench:       4,336 ns/iter (+/- 2,489)
test tests::line_parser::benchmarks::bench_simple_parse    ... bench:       1,391 ns/iter (+/- 180)
```

Performance figures are from `rustc-1.3.0-nightly (82d40cb2b 2015-07-24)` on
Windows (since `cargo bench` does not currently run on beta or stable), so may
not be representative of stable or beta, or Rust on other OSes.

## Usage
rust-caliburn is *not* an IRC client library; it does not implement the logic
behind the protocol, it only validates IRC client message lines and parses them
into a machine-readable form. It is intended to be a ready-to-use solution for
IRC servers and clients to use so that they comply with IRC interop standards
without significant work.

### Parsing lines
TODO: Add instructions

There are examples of using the parser in unit tests available `tests/mod.rs`.

## Compliance
rust-caliburn follows the RFC2812 spec as closely as possible for anything but
exceptional circumstances, including:

 * Assuming that prefixes of the form `:localhost` are servers, not users

 * IPv4 addresses (as well as all IPv6 forms of IPv6 addresses are not strictly
   validated (IPs like `999.999.999.999`, while invalid, are accepted)

 * IPv6 addresses are not allowed to have the fully abbreviated (::) empty
   segment notation (use 0000 or 0 instead)

 * Any command may have no more than 15 parameters

The only deviations from RFC2812 are:

 * No fixed length limit to nicknames - it is technically against
   the RFC2812 specification to accept nicknames longer than 10 chars, but
   the vast majority of IRCds and clients in current use do not follow this
   behaviour to the point where enforcing compliance may affect use of the
   library
   
 * Forward-slash (0x2F) are allowed in hosts since some networks (like Freenode)
   use this for host cloaks

 * Lower case may be used in IPv6 addresses (but not mixed with upper case)
 
 * IRCds commonly use ~ to prefix idents that are unverified, so support those
   in the ident even though it's techincally not allowed
   
 * Some networks (freenooooode!) allow the use of . at the _end_ of hostnames
   which don't map to real ones (such as `services.`), so we begrudgingly
   support that
   
## Implementation status

### What is currently implemented

  * Basic parsing (Commands, user strings, server strings, parameters)

### What remains to be done

#### Communication
  * Serializing rfc2812_types::Message back into IRC message strings

#### Input validation specification to be completed for:
  * Channels and channel keys
  * Mask expressions
  * Command-level validation
  * Allowing idents to have ~, many networks use this to indicate unverified
    idents

#### Code hygiene
  * Increased unit test coverage
  * Better commenting in PEG spec
    
[ci-build-stat]: https://travis-ci.org/ceph3us/rust-caliburn.svg?branch=master
[ci-link]: https://travis-ci.org/ceph3us/rust-caliburn
[rust-peg]: https://github.com/kevinmehall/rust-peg
[cov-stat]: https://coveralls.io/repos/ceph3us/rust-caliburn/badge.svg?branch=master&service=github
[cov-link]: https://coveralls.io/github/ceph3us/rust-caliburn?branch=master
