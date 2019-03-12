# Celma 

[![Build Status](https://travis-ci.org/d-plaindoux/celma.svg?branch=master)](https://travis-ci.org/d-plaindoux/celma)

[Celma ("k")noun "channel" (KEL) in Quenya](https://www.elfdict.com/w/kelma)

Celma is a generalised parser combinator implementation. Generalised means not an implementation 
restricted to a stream of characters.

## Overview

Celma provides usual parser combinator like `returns`, `fails`, `eos`, `satisfy`, `fmap`,
`bind`, `and` and `or`. The main difference is the nature of the data analysed. In the 
seminal paper the source is a string. In this approach It's a parametric stream. Therefore
it's possible to parse stream of `char`, `u8` or `user defined tokens`. One direct application
is the capability to design parser based on pipelined parsers and separate parsers regarding
their semantic level.        

# License

Copyright 2019 D. Plaindoux.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
