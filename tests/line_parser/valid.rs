extern crate caliburn;

use caliburn::rfc2812;
use caliburn::line_parser;

use std::path;
use std::io;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn parse_no_prefix() {
    let res = rfc2812::irc_msg("NICK test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::NoPrefix,
            command: line_parser::Command::Verb("NICK"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_server() {
    let res = rfc2812::irc_msg(":irc.example.com NICK test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::Server("irc.example.com"),
            command: line_parser::Command::Verb("NICK"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_server_ip4_host() {
    let res = rfc2812::irc_msg(":127.0.0.1 NICK test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::Server("127.0.0.1"),
            command: line_parser::Command::Verb("NICK"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_server_ip6_host_upper() {
    let res = rfc2812::irc_msg(":AAAA:AAAA:AAAA:AAAA:AAAA:AAAA:AAAA:AAAA NICK test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::Server("AAAA:AAAA:AAAA:AAAA:AAAA:AAAA:AAAA:AAAA"),
            command: line_parser::Command::Verb("NICK"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_server_ip6_host() {
    let res = rfc2812::irc_msg(":aaaa:aaaa:aaaa:aaaa:aaaa:aaaa:aaaa:aaaa NICK test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::Server("aaaa:aaaa:aaaa:aaaa:aaaa:aaaa:aaaa:aaaa"),
            command: line_parser::Command::Verb("NICK"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_server_ip4_host_as_ip6_lower() {
    let res = rfc2812::irc_msg(":0:0:0:0:0:ffff:127.0.0.1 NICK test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::Server("0:0:0:0:0:ffff:127.0.0.1"),
            command: line_parser::Command::Verb("NICK"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_server_ip4_host_as_ip6_upper() {
    let res = rfc2812::irc_msg(":0:0:0:0:0:FFFF:127.0.0.1 NICK test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::Server("0:0:0:0:0:FFFF:127.0.0.1"),
            command: line_parser::Command::Verb("NICK"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_server_ip4_host_as_ip6_nil() {
    let res = rfc2812::irc_msg(":0:0:0:0:0:0:127.0.0.1 NICK test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::Server("0:0:0:0:0:0:127.0.0.1"),
            command: line_parser::Command::Verb("NICK"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_user() {
    let res = rfc2812::irc_msg(":test!user@isp.user.example.com NAME test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test",
                user: Some("user"),
                host: Some("isp.user.example.com")
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_user_ip4_host() {
    let res = rfc2812::irc_msg(":test!user@127.0.0.1 NAME test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test",
                user: Some("user"),
                host: Some("127.0.0.1")
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_user_invalid_ip4_host() {
    let res = rfc2812::irc_msg(":test!user@999.999.999.999 NAME test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test",
                user: Some("user"),
                host: Some("999.999.999.999")
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_user_ip4_host_as_ip6_upper() {
    let res = rfc2812::irc_msg(":test!user@0:0:0:0:0:FFFF:127.0.0.1 NAME test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test",
                user: Some("user"),
                host: Some("0:0:0:0:0:FFFF:127.0.0.1")
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_user_ip4_host_as_ip6_lower() {
    let res = rfc2812::irc_msg(":test!user@0:0:0:0:0:ffff:127.0.0.1 NAME test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test",
                user: Some("user"),
                host: Some("0:0:0:0:0:ffff:127.0.0.1")
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_user_ip4_host_as_ip6_nil() {
    let res = rfc2812::irc_msg(":test!user@0:0:0:0:0:0:127.0.0.1 NAME test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test",
                user: Some("user"),
                host: Some("0:0:0:0:0:0:127.0.0.1")
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_user_ip6_host_lower() {
    let res = rfc2812::irc_msg(":test!user@aaaa:aaaa:aaaa:aaaa:aaaa:aaaa:aaaa:aaaa NAME test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test",
                user: Some("user"),
                host: Some("aaaa:aaaa:aaaa:aaaa:aaaa:aaaa:aaaa:aaaa")
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_user_ip6_host_upper() {
    let res = rfc2812::irc_msg(":test!user@AAAA:AAAA:AAAA:AAAA:AAAA:AAAA:AAAA:AAAA NAME test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test",
                user: Some("user"),
                host: Some("AAAA:AAAA:AAAA:AAAA:AAAA:AAAA:AAAA:AAAA")
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_prefixed_user_no_ident() {
    let res = rfc2812::irc_msg(":test@isp.user.example.com NAME :test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test",
                user: None,
                host: Some("isp.user.example.com")
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_user_with_hat() {
    let res = rfc2812::irc_msg(":test^ NAME :test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test^",
                user: None,
                host: None
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_user_with_underscore() {
    let res = rfc2812::irc_msg(":test_ NAME :test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::User {
                nickname: "test_",
                user: None,
                host: None
            },
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn parse_localhost_is_server() {
    let res = rfc2812::irc_msg(":localhost NAME :test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::Server("localhost"),
            command: line_parser::Command::Verb("NAME"),
            params: vec!["test".to_string()]
        });
}

#[test]
fn numeric_response_prefixed() {
    let res = rfc2812::irc_msg(":irc.example.com 481 :Permission Denied- You're not an IRC operator");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::Server("irc.example.com"),
            command: line_parser::Command::Numeric(481),
            params: vec!["Permission Denied- You're not an IRC operator".to_string()]
        });
}

#[test]
fn numeric_response_unprefixed() {
    let res = rfc2812::irc_msg("481 :Permission Denied- You're not an IRC operator");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::NoPrefix,
            command: line_parser::Command::Numeric(481),
            params: vec!["Permission Denied- You're not an IRC operator".to_string()]
        });
}

#[test]
fn numeric_response_isnt_octal() {
    let res = rfc2812::irc_msg(":irc.example.com 077 :Should be 77, not 63");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: line_parser::Prefix::Server("irc.example.com"),
            command: line_parser::Command::Numeric(77),
            params: vec!["Should be 77, not 63".to_string()]
        });
}

// Parses a sample connection in src/tests/connection-sample.txt
// Does not check results for correctness!
#[test]
fn can_parse_whole_connection() {
    let mut path = path::PathBuf::from(".");
    path.push("tests");
    path.push("connection-sample.txt");

    let finalpath = path.clone();

    let testfile = File::open(finalpath).unwrap_or_else(|e| {
    	panic!("Failed to open file {:?}: {}", path, e);
    });

    let lines = io::BufReader::new(&testfile).lines();

    for line in lines {
    	let input = line.unwrap_or_else(|e| {
	    panic!("Couldn't read next line: {}", e);
	});
	println!("Line: {}", input);

	let res = rfc2812::irc_msg(&input);

	assert!(res.is_ok());
	println!("Parsed to: {:?}", res.unwrap());

	println!("");
    }
}