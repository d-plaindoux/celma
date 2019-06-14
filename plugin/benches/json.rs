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

use celma_core::parser::and::AndOperation;
use celma_core::parser::char::{digit, space};
use celma_core::parser::core::eos;
use celma_core::parser::literal::delimited_string;
use celma_core::parser::parser::Parse;
use celma_core::parser::response::Response::{Reject, Success};
use celma_core::stream::iterator_stream::IteratorStream;
use celma_core::stream::position::Position;
use celma_core::stream::stream::Stream;
use celma_plugin::parsec_rules;

#[derive(Clone)]
pub enum JSON {
    Number(f64),
    String(String),
    Null,
    Bool(bool),
    Array(Vec<JSON>),
    Object(Vec<(String, JSON)>),
}

fn mk_vec<E>(a: Option<(E, Vec<E>)>) -> Vec<E> {
    if a.is_none() {
        Vec::new()
    } else {
        let (a, v) = a.unwrap();
        let mut r = v;
        r.insert(0, a);
        r
    }
}

fn mk_f64(a: Vec<char>) -> f64 {
    a.into_iter().collect::<String>().parse().unwrap()
}

parsec_rules!(
    //-------------------------------------------------------------------------
    let json:{JSON}          = S _=(string | null | boolean  | array | object | number) S
    //-------------------------------------------------------------------------
    let number:{JSON}        = f=NUMBER                                 -> {JSON::Number(f)}
    let string:{JSON}        = s=STRING                                 -> {JSON::String(s)}
    let null:{JSON}          = "null"                                   -> {JSON::Null}
    let boolean:{JSON}       = b=("true"|"false")                       -> {JSON::Bool(b=="true")}
    let array:{JSON}         = ('[' S a=(_=json _=(',' _=json)*)? ']')  -> {JSON::Array(mk_vec(a))}
    let object:{JSON}        = ('{' S a=(_=attr _=(',' _=attr)*)? '}')  -> {JSON::Object(mk_vec(a))}
    let attr:{(String,JSON)} = (S s=STRING S ":" j=json)                -> {(s,j)}
    //-------------------------------------------------------------------------
    let STRING:{String}      = delimited_string
    //-------------------------------------------------------------------------
    let NUMBER:{f64}         = c=#(INT ('.' NAT)? (('E'|'e') INT)?)     -> {mk_f64(c)}
    let INT:{()}             = ('-'|'+')? NAT                           -> {}
    let NAT:{()}             = digit+                                   -> {}
    //-------------------------------------------------------------------------
    let S:{()}               = space*                                   -> {}
    //-------------------------------------------------------------------------
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

        let response = json().and_left(eos()).parse(stream);

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
