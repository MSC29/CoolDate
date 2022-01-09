# CoolDate

Cool Date rust lib

## What is this?

## What does it do?

#### default: future cool dates
input a date, find next anniversaries
e.g. birth day & find 4444th day, after today if past date, after date if future date
e.g. couple together date & find 666th day together, after today if past date, after date if future date

#### input a date, find past anniversaries
e.g. birth day & find 4444th day, before today
e.g. couple together date & find 666th day together, before today

#### input a date, find all anniversaries

#### input a date, find all anniversaries, filter by Unit

#### input a date, find all anniversaries, filter by Count

#### input a date, find all anniversaries, filter by Unit & Count

#### working hours
input a date, input the number of hours per day, input number of days per week

## Dev

## structure

src/lib.rs is the lib
test/test.rs is a just a quick & dirty check whether the compiled lib runs properly

### build lib

```rust
cargo build --release
```

### create testable executable

```rust
cd test && rustc test.rs --extern lib=../target/release/libfoo.rlib && cd ..
./test/test
```
