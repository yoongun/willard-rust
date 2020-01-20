![crate v0.1.0 badge](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg?longCache=true)
![test status badge](https://github.com/cfr2ak/willard/workflows/test/badge.svg)
![build status badge](https://github.com/cfr2ak/willard/workflows/build/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# :cat: willard

Heterogeneous quantum computer simulator in Rust

## F.D.C. willard: the greatest physicist ever exist in cat history

https://en.wikipedia.org/wiki/F._D._C._Willard

## How to use

### Declare the use of this module

Add this line to the top of your project

```rust
extern crate willard;
use willard;
```

### Make a new qubit

```rust
let mut qubit = willard::Qubit::default();
```

This will give you a qubit with state |0>

#### Measure it

```rust
let result = willard::measure(&mut qubit);
```

### Applying gates

#### Single qubit gates

```rust
// Hadamard gate
willard::gate::h(&mut qubit);

// x, y, z gate
willard::gate::x(&mut qubit);
willard::gate::y(&mut qubit);
willard::gate::z(&mut qubit);

// not gate (same as x gate)
willard::gate::not(&mut qubit);

// square root of not gate
willard::gate::sqrt_not(&mut qubit);

// phase gate
willard::gate::phase(&mut qubit, <degree in radian of f32 type>);
```
