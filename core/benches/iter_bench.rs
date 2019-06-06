#[macro_use]
extern crate bencher;

use bencher::{black_box, Bencher};

use celma_core::parser::and::AndOperation;
use celma_core::parser::char::char;
use celma_core::parser::char::not_char;
use celma_core::parser::core::any;
use celma_core::parser::core::eos;
use celma_core::parser::or::OrOperation;
use celma_core::parser::parser::Combine;
use celma_core::parser::parser::Parse;
use celma_core::parser::repeat::RepeatOperation;
use celma_core::parser::response::Response::Reject;
use celma_core::parser::response::Response::Success;
use celma_core::stream::iterator_stream::IteratorStream;
use celma_core::stream::stream::Len;
use celma_core::stream::stream::Stream;

// -------------------------------------------------------------------------------------------------
// Basic benchmarks
// -------------------------------------------------------------------------------------------------

const SIZE: usize = 1024;

fn basic_any(bencher: &mut Bencher) {
    let string = "a".repeat(SIZE);
    let data = string.as_str();

    let parser = any().opt_rep().and(eos());

    do_parse(parser, bencher, IteratorStream::new(data.chars()));
}

fn basic_a(bencher: &mut Bencher) {
    let string = "a".repeat(SIZE);
    let data = string.as_str();

    let parser = char('a').opt_rep().and(eos());

    do_parse(parser, bencher, IteratorStream::new(data.chars()));
}

fn basic_a_or_b(bencher: &mut Bencher) {
    let string = "ab".repeat(1024 * 1024);
    let data = string.as_str();

    let parser = char('a').or(char('b')).opt_rep().and(eos());

    do_parse(parser, bencher, IteratorStream::new(data.chars()));
}

fn basic_a_and_b(bencher: &mut Bencher) {
    let string = "ab".repeat(SIZE);
    let data = string.as_str();

    let parser = char('a').and(char('b')).opt_rep().and(eos());

    do_parse(parser, bencher, IteratorStream::new(data.chars()));
}

fn basic_delimited_string(bencher: &mut Bencher) {
    let string = "\"hello\"".repeat(SIZE);
    let data = string.as_str();

    let parser = char('"')
        .and(not_char('"').opt_rep())
        .and(char('"'))
        .opt_rep()
        .and(eos());

    do_parse(parser, bencher, IteratorStream::new(data.chars()));
}

// -------------------------------------------------------------------------------------------------

fn do_parse<P, A, S>(parser: P, bencher: &mut Bencher, stream: S)
where
    P: Parse<A, S> + Combine<A>,
    S: Stream + Len,
{
    bencher.bytes = stream.len() as u64;

    bencher.iter(|| match parser.parse(black_box(stream.clone())) {
        Success(_, _, _) => (),
        Reject(_, _) => panic!("Cannot parse stream"),
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
