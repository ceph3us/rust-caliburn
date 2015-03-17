use super::super::super::rfc2812;
use test::Bencher;

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
    b.iter(|| rfc2812::irc_msg(":test!user@isp.user.example.com USER test 0 * :Test user"));
}

#[bench]
fn bench_ludicrous_parse(b: &mut Bencher) {
    b.iter(|| rfc2812::irc_msg(":testtesttest!useruseruser@a.really.really.really.long.isp.user.example.com ABCDABCDABCDABCD EFGHEFGHEFGHEFGH IJKLIJKLIJKLIJKL MNOPMNOPMNOPMNOP QRSTQRSTQRSTQRST UVWXUVWXUVWXUVWX YZ01YZ01YZ01YZ01 2345234523452345 6789678967896789 !@?#!@?#!@?#!@?# }{{}}}{{}}}{{}}}{{}} $%^&$%^&$%^&$%^& +=-'+=-'+=-'+=-' ~|.,~|.,~|.,~|., :The quick brown fox jumped over the lazy dog. The quick brown fox jumped over the lazy dog. The quick brown fox jumped over the lazy dog. The quick brown fox jumped over the lazy dog. ABCDEFGHIJK"));
}
