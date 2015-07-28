//extern crate test;

extern crate caliburn;

use self::caliburn::rfc2812;
use self::caliburn::line_parser;

#[test]
#[should_panic(expected = "assertion failed: `(left == right)`")]
fn localhost_is_not_user() {
    let testcase = line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "localhost",
                user: None,
                host: None
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        };

    let res = rfc2812::irc_msg(":localhost NAME :test").unwrap();
    println!("Testcase: {:?}, Parsed: {:?}", testcase, res);
    assert_eq!(res,
        testcase);
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn prefixed_user_ident_without_host() {
    let res = rfc2812::irc_msg(":test!user NAME :test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn too_many_args() {
    let res = rfc2812::irc_msg(":irc.example.com A B C D E F G H I J K L M N O P Q R S :T");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn middle_param_has_colon() {
    let res = rfc2812::irc_msg(":irc.example.com HELL:O :WORLD");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn numeric_too_short() {
    let res = rfc2812::irc_msg(":irc.example.com 42 :Invalid numeric (too short)");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn parse_prefixed_user_ip4_host_as_ip6_invalid_ip4() {
    let res = rfc2812::irc_msg(":test!user@0:0:0:0:0:FFFF:127.0..1 NAME test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn parse_prefixed_user_ip4_host_invalid_empty_segment() {
    let res = rfc2812::irc_msg(":test!127.0..1 NAME test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn parse_prefixed_user_ip4_host_invalid_fewer_segments() {
    let res = rfc2812::irc_msg(":test!127.0.1 NAME test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn parse_prefixed_user_ip6_host_invalid_fewer_segments() {
    let res = rfc2812::irc_msg(":test!aaaa:aaaa:aaaa:aaaa:aaaa NAME test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn parse_prefixed_user_ip6_host_invalid_mixed_case() {
    let res = rfc2812::irc_msg(":test!aaaa:aaaa:aaaa:aaaa:aaaa:AAAA:AAAA:AAAA NAME test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn parse_prefixed_user_ip6_host_invalid_empty_segment() {
    let res = rfc2812::irc_msg(":test!aaaa:aaaa:aaaa:aaaa:aaaa:aaaa::aaaa NAME test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn command_with_invalid_char() {
    let res = rfc2812::irc_msg("NAME!EMAN test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn numeric_with_invalid_char() {
    let res = rfc2812::irc_msg("1N2 test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn empty_line_should_fail() {
    let res = rfc2812::irc_msg("");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn host_only() {
    let res = rfc2812::irc_msg(":test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn user_only() {
    let res = rfc2812::irc_msg(":test!test@test");
    assert!(res.is_ok());
}

#[test]
#[should_panic(expected = "res.is_ok()")]
fn no_command() {
    let res = rfc2812::irc_msg(":test!test@test :hello");
    assert!(res.is_ok());
}

#[test]
fn check_failed_parse_throws_error() {
   let res = rfc2812::irc_msg("!!!!!");
   match res {
   	 Ok(_)	=> panic!("Didn't throw parse error on invalid line!"),
	 Err(e) => println!("{}", e) // So it's not optimised away
   }
}