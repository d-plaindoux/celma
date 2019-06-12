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

use bencher::{Bencher, black_box};

use celma_core::parser::char::{digit, char_in_set};
use celma_core::parser::core::eos;
use celma_core::parser::response::Response::{Reject, Success};
use celma_core::stream::char_stream::CharStream;
use celma_core::stream::stream::Stream;
use celma_plugin::parsec_rules;

parsec_rules!(
    let json:{()}    = S (string | null | boolean | array | object | number) S
    let number:{()}  = NUMBER                                   -> { () }
    let string:{()}  = STRING                                   -> { () }
    let null:{()}    = "null"                                   -> { () }
    let boolean:{()} = ("true"|"false")                         -> { () }
    let array:{()}   = '[' (json (',' json)*)? ']'              -> { () }
    let object:{()}  = '{' (attr (',' attr)*)? '}'              -> { () }
    let attr:{()}    = S STRING S ":" json                      -> { () }

    let STRING:{()}  = '"' (^'"')* '"'                          -> { () }
    let NUMBER:{()}  = INT ('.' NAT)? (('E'|'e') INT)?          -> { () }
    let INT:{()}     = ('-'|'+')? _=NAT                         -> { () }
    let NAT:{()}     = digit+                                   -> { () }
    let S:{()}       = {char_in_set(vec!(' ','\t','\r','\n'))}* -> { () }
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

fn parse(b: &mut Bencher, buffer: &str)
{
    b.iter(|| {
        let buffer = black_box(buffer);

        let response = json().and_left(eos()).check(CharStream::new(buffer));

        match response {
            Success(_, _, _) => (),
            Reject(s, _) => panic!("parse error at {:?}", s.position())
        }
    });
}

benchmark_group!(
    benches,
    json_data,
    json_canada_pest,
    json_canada_nom,
    //json_apache
);

benchmark_main!(benches);