# Therapy Assignment

This is an implementation of an assignment to convert inputs into outputs using the corresponding expressions.

## Installation

Use cargo to run the application and create a server listening on localhost port 8080

```bash
cargo run
```

## Usage

We can send a POST request to `http://127.0.0.1:8080/eval` with the values of A, B, C, D, E, F and receive the value of H and K

```bash
curl -i -d '{"A":true, "B":true, "C":true, "D":76.5, "E":2, "F":1}' -H 'Content-Type: application/json' http://127.0.0.1:8080/eval
```

## Architecture
It was decided that hard codding the corresponding expressions for substitutions is not very flexible because in a real case scenario the code would need to be rewritten constantly, whenever an expression changes or is overridden.

The approach chosen  was to divide the expressions in two categories, those who alter **H** and those who alter **K**. Then create a small parser for such expressions, this way we can handle the case when a new expression is added or overridden, easily.

This code is written in Rust and contains Unit Tests for the ExpressionProcessor (Parser) and for the **H** and **K** substitutions, which can be run using `cargo test`. For the porpouse of this assignment it's assumed that the expressions for substitutions are valid.

A single endpoint in `/eval` is provided, this was implemented using minimal functionality provided by _actix-web_ and _serde_. The logic to evaluate an expression is provided by _evalexpr_.