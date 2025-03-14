/*
 * Copyright 2019-2025 Didier Plaindoux
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use bencher::{Bencher, benchmark_group, benchmark_main, black_box};
use celma_v0_core::parser::and::AndOperation;
use celma_v0_core::parser::char::{alpha, digit};
use celma_v0_core::parser::core::eos;
use celma_v0_core::parser::response::Response::{Reject, Success};
use celma_v0_core::parser::specs::Parse;
use celma_v0_core::stream::char_stream::CharStream;
use celma_v0_core::stream::position::Position;
use celma_v0_core::stream::specs::Stream;
use celma_v0_macro::parsec_rules;

parsec_rules!(
    let http_header = request (header)* EOL -> {}
    let request = VERB S URI S VERSION EOL -> {}
    let header = NAME ':' S VALUE EOL -> {}
);

parsec_rules!(
    let VERB = ("GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "CONNECT" | "PATCH") -> {}
    let URI  = ^(' ')+ -> {}
    let VERSION = "HTTP/" digit+ ('.' digit+)? -> {}
    let S  = (' ' | '\t')+ -> {}
    let EOL = ('\r'? '\n') -> {}
    let NAME = (alpha | '-')+ -> {}
    let VALUE = ^EOL+ -> {} // Not precise enough
);

// -------------------------------------------------------------------------------------------------
// HTTP benchmarks
// -------------------------------------------------------------------------------------------------

fn http_data(b: &mut Bencher) {
    let data = include_str!("data/request.http");
    b.bytes = data.len() as u64;
    parse(b, data)
}

fn parse(b: &mut Bencher, buffer: &str) {
    let stream = CharStream::new_with_position(buffer, <usize>::new());

    b.iter(|| {
        let response = http_header()
            .and_left(eos())
            .parse(black_box(stream.clone()));

        match response {
            Success(_, _, _) => (),
            Reject(s, _) => panic!("parse error for {:?} at {:?}", s.next().0, s.position()),
        }
    });
}

benchmark_group!(benches, http_data,);

benchmark_main!(benches);
