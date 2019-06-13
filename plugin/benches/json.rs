/*
   Copyright 2019 Didier Plaindoux

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

use bencher::{black_box, Bencher};

use celma_core::parser::and::AndOperation;
use celma_core::parser::char::{digit, space};
use celma_core::parser::core::eos;
use celma_core::parser::literal::delimited_string;
use celma_core::parser::parser::Parse;
use celma_core::parser::response::Response::{Reject, Success};
use celma_core::stream::stream::Stream;
use celma_plugin::parsec_rules;
use celma_core::stream::iterator_stream::IteratorStream;
use celma_core::stream::position::Position;

parsec_rules!(
    let json:{()}    = S (string | null | boolean  | array | object | number) S
    //------------------------------------------------------------------------
    let number:{()}  = try NUMBER                                       -> {}
    let string:{()}  = STRING                                           -> {}
    let null:{()}    = "null"                                           -> {}
    let boolean:{()} = ("true"|"false")                                 -> {}
    let array:{()}   = '[' S (json (',' json)*)? ']'                    -> {}
    let object:{()}  = '{' S (attr (',' attr)*)? '}'                    -> {}
    let attr:{()}    = S STRING S ":" json                              -> {}
    //------------------------------------------------------------------------
    let STRING:{()}  = delimited_string                                 -> {}
    //------------------------------------------------------------------------
    let NUMBER:{()}  = INT ('.' NAT)? (('E'|'e') INT)?                  -> {}
    let INT:{()}     = ('-'|'+')? NAT                                   -> {}
    let NAT:{()}     = digit+                                           -> {}
    //------------------------------------------------------------------------
    let S:{()}       = space*                                           -> {}
);

// -------------------------------------------------------------------------------------------------
// JSon benchmarks
// -------------------------------------------------------------------------------------------------

fn json_data(b: &mut Bencher) {
    let data = include_str!("data/data.json");
    b.bytes = data.len() as u64;
    parse(b, data)
}

// -------------------------------------------------------------------------------------------------

fn json_canada_pest(b: &mut Bencher) {
    let data = include_str!("data/canada_pest.json");
    b.bytes = data.len() as u64;
    parse(b, data)
}

// -------------------------------------------------------------------------------------------------

fn json_canada_nom(b: &mut Bencher) {
    let data = include_str!("data/canada_nom.json");
    b.bytes = data.len() as u64;
    parse(b, data)
}

// -------------------------------------------------------------------------------------------------

fn json_apache(b: &mut Bencher) {
    let data = include_str!("data/apache_builds.json");
    b.bytes = data.len() as u64;
    parse(b, data)
}

// -------------------------------------------------------------------------------------------------

fn parse(b: &mut Bencher, buffer: &str) {
    b.iter(|| {
        let buffer = black_box(buffer);
        let stream = IteratorStream::new_with_position(buffer.chars(), <usize>::new());

        let response = json().and_left(eos()).check(stream);

        match response {
            Success(_, _, _) => (),
            Reject(s, _) => panic!("parse error at {:?}", s.position()),
        }
    });
}

benchmark_group!(
    benches,
    json_data,
    json_canada_pest,
    json_canada_nom,
    json_apache
);

benchmark_main!(benches);
