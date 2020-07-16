## Data Types

* rust is _strongly + statically_ typed
* must know types of all values during compilation
* automatic type inference
* _Scalar_ vs _Compound_

### Scalar Types

#### 1. Integer

* `i` for signed, `u` for unsigned
* followed by bits: `8`, `16`, `32`, `64`, `128`
* overflow handled only during debugging
* use `Wrapper` for intentional wrapping
* use `_` for readability

```rust
let x: i32 = 56;
let y: u64 = 100_000
```

#### 2. Float

* `f32` and `f64`

```rust
let x: f32 = 2.3;
let y: f64: 3.6;
```

#### 3. Char

* supports Unicode

```rust
let char = '👀';
```

#### 4. Boolean

* simple `true` or `false`

### Compound Types

#### 1. Tuple

* list of values
* can have different data types
* can be destructured/indexed

```rust
let x: (i32, bool, char) = (56, false, 'z');

let (num, cond, ch) = x;
let ch = x.2;
```

#### 2. Arrays

* list of values
* values must be of same type
* has fixed length
* use `Vector` type for more flexibility

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let b: [false; 3]; // [false, false, false]

let two = a.1;
```