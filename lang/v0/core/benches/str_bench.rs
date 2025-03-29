/*
   Copyright 2019-2025 Didier Plaindoux

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

#[macro_use]
extern crate bencher;

use bencher::{Bencher, black_box};

use celma_v0_core::parser::and::AndOperation;
use celma_v0_core::parser::char::a_char;
use celma_v0_core::parser::core::any;
use celma_v0_core::parser::core::eos;
use celma_v0_core::parser::literal::delimited_string;
use celma_v0_core::parser::or::OrOperation;
use celma_v0_core::parser::repeat::RepeatOperation;
use celma_v0_core::parser::response::Response::Reject;
use celma_v0_core::parser::response::Response::Success;
use celma_v0_core::parser::specs::Combine;
use celma_v0_core::parser::specs::Parse;
use celma_v0_core::stream::char_stream::CharStream;
use celma_v0_core::stream::position::CharIndex;
use celma_v0_core::stream::specs::Len;
use celma_v0_core::stream::specs::Stream;

// -------------------------------------------------------------------------------------------------
// Basic benchmarks
// -------------------------------------------------------------------------------------------------

const SIZE: usize = 1024 * 32;

fn basic_any(bencher: &mut Bencher) {
    let string = "a".repeat(SIZE);
    let data = string.as_str();

    let parser = any().opt_rep().and(eos());

    do_parse(
        parser,
        bencher,
        CharStream::new_with_position(data, CharIndex::default()),
    );
}

fn basic_a(bencher: &mut Bencher) {
    let string = "a".repeat(SIZE);
    let data = string.as_str();

    let parser = a_char('a').opt_rep().and(eos());

    do_parse(
        parser,
        bencher,
        CharStream::new_with_position(data, CharIndex::default()),
    );
}

fn basic_a_or_b(bencher: &mut Bencher) {
    let string = "ab".repeat(1024 * 1024);
    let data = string.as_str();

    let parser = a_char('a').or(a_char('b')).opt_rep().and(eos());

    do_parse(
        parser,
        bencher,
        CharStream::new_with_position(data, CharIndex::default()),
    );
}

fn basic_a_and_b(bencher: &mut Bencher) {
    let string = "ab".repeat(SIZE);
    let data = string.as_str();

    let parser = a_char('a').and(a_char('b')).opt_rep().and(eos());

    do_parse(
        parser,
        bencher,
        CharStream::new_with_position(data, CharIndex::default()),
    );
}

fn basic_delimited_string(bencher: &mut Bencher) {
    let string = "\"hello world!\"".repeat(SIZE);
    let data = string.as_str();

    let parser = delimited_string().opt_rep().and(eos());

    do_parse(
        parser,
        bencher,
        CharStream::new_with_position(data, CharIndex::default()),
    );
}

// -------------------------------------------------------------------------------------------------

fn do_parse<P, A, S>(parser: P, bencher: &mut Bencher, stream: S)
where
    P: Parse<A, S> + Combine<A>,
    S: Stream + Len,
{
    bencher.bytes = stream.len() as u64;

    bencher.iter(|| match parser.check(black_box(stream.clone())) {
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
