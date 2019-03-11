#![feature(repeat_generic_slice)]

#[macro_use]
extern crate bencher;

use bencher::{black_box, Bencher};

use celma::parser::and::AndOperation;
use celma::parser::core::any;
use celma::parser::core::eos;
use celma::parser::or::OrOperation;
use celma::parser::parser::Combine;
use celma::parser::parser::Parse;
use celma::parser::repeat::RepeatOperation;
use celma::parser::response::Response::Reject;
use celma::parser::response::Response::Success;
use celma::parser::satisfy::Satisfy;
use celma::stream::stream::Len;
use celma::stream::stream::Stream;
use celma::stream::vec_stream::VecStream;

// -------------------------------------------------------------------------------------------------
// Basic benchmarks
// -------------------------------------------------------------------------------------------------

const SIZE: usize = 1024;

fn u8<S>(v: char) -> impl Parse<u8, S> + Combine<u8>
where
    S: Stream<Item = u8>,
{
    Satisfy::new(move |&u| u as char == v)
}

fn not_u8<S>(v: char) -> impl Parse<u8, S> + Combine<u8>
where
    S: Stream<Item = u8>,
{
    Satisfy::new(move |&u| u as char != v)
}

fn basic_any(bencher: &mut Bencher) {
    let data = b"a".to_vec().repeat(SIZE);

    let parser = any().opt_rep().and(eos());

    do_parse(parser, bencher, VecStream::new(&data));
}

fn basic_a(bencher: &mut Bencher) {
    let data = b"a".to_vec().repeat(SIZE);

    let parser = u8('a').opt_rep().and(eos());

    do_parse(parser, bencher, VecStream::new(&data));
}

fn basic_a_or_b(bencher: &mut Bencher) {
    let data = b"ab".to_vec().repeat(SIZE);

    let parser = u8('a').or(u8('b')).opt_rep().and(eos());

    do_parse(parser, bencher, VecStream::new(&data));
}

fn basic_a_and_b(bencher: &mut Bencher) {
    let data = b"ab".to_vec().repeat(SIZE);

    let parser = u8('a').and(u8('b')).opt_rep().and(eos());

    do_parse(parser, bencher, VecStream::new(&data));
}

fn basic_delimited_string(bencher: &mut Bencher) {
    let data = b"\"hello\"".to_vec().repeat(SIZE);

    let parser = u8('"')
        .and(not_u8('"').opt_rep())
        .and(u8('"'))
        .opt_rep()
        .and(eos());

    do_parse(parser, bencher, VecStream::new(&data));
}

// -------------------------------------------------------------------------------------------------

fn do_parse<P, A, S>(parser: P, bencher: &mut Bencher, stream: S)
where
    P: Parse<A, S> + Combine<A>,
    S: Stream + Len,
{
    bencher.bytes = stream.len().unwrap() as u64;

    bencher.iter(|| match parser.parse(black_box(stream.clone())) {
        Success(_, _, _) => (),
        Reject(_) => panic!("Cannot parse stream"),
    });
}

// -------------------------------------------------------------------------------------------------

benchmark_group!(
    benches,
    basic_any,
    basic_a,
    basic_a_or_b,
    basic_a_and_b,
    basic_delimited_string
);
benchmark_main!(benches);
