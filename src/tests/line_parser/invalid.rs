use super::super::super::rfc2812;
use super::super::super::line_parser;

#[test]
#[should_panic(expected = "(left == right) && (right == left)")]
fn localhost_is_not_user() {
    let res = rfc2812::irc_msg(":localhost NAME :test");
    assert_eq!(res.unwrap(),
        line_parser::Message {
            prefix: Some(line_parser::Prefix::User {
                nickname: "localhost",
                user: None,
                host: None
            }),
            command: line_parser::Command::Verb("NAME"),
            params: Some(vec!["test"])
        });
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
fn numeric_too_long() {
    let res = rfc2812::irc_msg(":irc.example.com 4242 :Invalid numeric (too long)");
    assert!(res.is_ok());
}
