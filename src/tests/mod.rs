// Unit tests to ensure RFC2812 compliance
// Copyright (c) 2015, Michael Holmes <holmesmich@gmail.com>. All right reserved.
// See included LICENSE.md in for licensing info.

use super::rfc2812;
use super::rfc2812_types;
use test::Bencher;

#[test]
fn parse_no_prefix() {
    let res = rfc2812::irc_msg("NICK :test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        rfc2812_types::Message {
            prefix: None,
            command: rfc2812_types::Command::Verb("NICK"),
            params: Some(vec!["test"])
        });
}

#[test]
fn parse_prefixed_server() {
    let res = rfc2812::irc_msg(":irc.example.com NICK :test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        rfc2812_types::Message {
            prefix: Some(rfc2812_types::Prefix::Server("irc.example.com")),
            command: rfc2812_types::Command::Verb("NICK"),
            params: Some(vec!["test"])
        });
}

#[test]
fn parse_prefixed_user() {
    let res = rfc2812::irc_msg(":test!user@isp.user.example.com NAME :test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        rfc2812_types::Message {
            prefix: Some(rfc2812_types::Prefix::User {
                nickname: "test",
                user: Some("user"),
                host: Some("isp.user.example.com")
            }),
            command: rfc2812_types::Command::Verb("NAME"),
            params: Some(vec!["test"])
        });
}

#[test]
fn parse_prefixed_user_no_ident() {
    let res = rfc2812::irc_msg(":test@isp.user.example.com NAME :test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        rfc2812_types::Message {
            prefix: Some(rfc2812_types::Prefix::User {
                nickname: "test",
                user: None,
                host: Some("isp.user.example.com")
            }),
            command: rfc2812_types::Command::Verb("NAME"),
            params: Some(vec!["test"])
        });
}

#[test]
fn parse_user_with_hat() {
    let res = rfc2812::irc_msg(":test^ NAME :test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        rfc2812_types::Message {
            prefix: Some(rfc2812_types::Prefix::User {
                nickname: "test^",
                user: None,
                host: None
            }),
            command: rfc2812_types::Command::Verb("NAME"),
            params: Some(vec!["test"])
        });
}

#[test]
fn parse_user_with_underscore() {
    let res = rfc2812::irc_msg(":test_ NAME :test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        rfc2812_types::Message {
            prefix: Some(rfc2812_types::Prefix::User {
                nickname: "test_",
                user: None,
                host: None
            }),
            command: rfc2812_types::Command::Verb("NAME"),
            params: Some(vec!["test"])
        });
}

#[test]
fn parse_localhost_is_server() {
    let res = rfc2812::irc_msg(":localhost NAME :test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        rfc2812_types::Message {
            prefix: Some(rfc2812_types::Prefix::Server("localhost")),
            command: rfc2812_types::Command::Verb("NAME"),
            params: Some(vec!["test"])
        });
}

#[test]
#[should_panic]
fn parse_localhost_is_not_user() {
    let res = rfc2812::irc_msg(":localhost NAME :test");
    assert_eq!(res.unwrap(),
        rfc2812_types::Message {
            prefix: Some(rfc2812_types::Prefix::User {
                nickname: "localhost",
                user: None,
                host: None
            }),
            command: rfc2812_types::Command::Verb("NAME"),
            params: Some(vec!["test"])
        });
}

#[test]
#[should_panic]
fn parse_prefixed_user_fail_if_ident_without_host() {
    let res = rfc2812::irc_msg(":test!user NAME :test");
    assert!(res.is_ok());
}

#[test]
#[should_panic]
fn fail_if_too_many_args() {
    let res = rfc2812::irc_msg(":irc.example.com A B C D E F G H I J K L M N O P Q R S :T");
    assert!(res.is_ok());
}

#[test]
#[should_panic]
fn fail_if_middle_param_has_colon() {
    let res = rfc2812::irc_msg(":irc.example.com HELL:O :WORLD");
    assert!(res.is_ok());
}

#[bench]
fn bench_simple_parse(b: &mut Bencher) {
    b.iter(|| rfc2812::irc_msg("NICK :test"));
}

#[bench]
fn bench_moderate_parse(b: &mut Bencher) {
    b.iter(|| rfc2812::irc_msg(":test!user@isp.user.example.com NAME :test"));
}

#[bench]
fn bench_complex_parse(b: &mut Bencher) {
    b.iter(|| rfc2812::irc_msg(":testtesttest!useruseruser@a.really.really.really.long.isp.user.example.com ABCDABCDABCDABCD EFGHEFGHEFGHEFGH IJKLIJKLIJKLIJKL MNOPMNOPMNOPMNOP QRSTQRSTQRSTQRST UVWXUVWXUVWXUVWX YZ01YZ01YZ01YZ01 2345234523452345 6789678967896789 !@?#!@?#!@?#!@?# }{{}}}{{}}}{{}}}{{}} $%^&$%^&$%^&$%^& +=-'+=-'+=-'+=-' ~|.,~|.,~|.,~|., :The quick brown fox jumped over the lazy dog. The quick brown fox jumped over the lazy dog. The quick brown fox jumped over the lazy dog. The quick brown fox jumped over the lazy dog. ABCDEFGHIJK"));
}
