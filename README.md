# rust-caliburn
A strictly compliant parser for the RFC2812 (IRC Client Protocol) specification,
written in Rust.

## Notice
rust-caliburn is currently not production ready and should be considered
unstable. It is also not feature complete.

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

 * IP addresses are not strictly validated (IPs like `999.999.999.999`, while
      invalid, are accepted)

 * Any command may have no more than 15 parameters

The only deviations from RFC2812 are:

 * No fixed length limit to nicknames - it is technically against
   the RFC2812 specification to accept nicknames longer than 10 chars, but
   the vast majority of IRCds and clients in current use do not follow this
   behaviour to the point where enforcing compliance may affect use of the
   library

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
  * IPv6 addresses

#### Code hygiene
  * Increased unit test coverage
  * Better commenting in PEG spec

#### Decisions to be made
  * Deviation from RFC2812 may be made to allow forward-slash (0x2F) since
    some networks (like Freenode) use this in host cloaks
