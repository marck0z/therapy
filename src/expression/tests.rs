use super::*;

#[test]
//contains a single H substitution and a K substitution and both apply
fn single_expression() {
    let ep = ExpressionProcessor::new(
        vec!["A && B && !C => H = M"],
        vec!["H = M => K = D + (D * E / 10)"]);

    let input = Input {
        a: true,
        b: true,
        c: false,
        d: 5.0,
        e: 2,
        f: 1,
    };
    let output = ep.evaluate(&input);
    assert_eq!(output.unwrap(), Output { h: 'M', k: 6.0 })
}

#[test]
//contains a single H substitution and a K substitution and none apply
fn single_expression_error() {
    let ep = ExpressionProcessor::new(
        vec!["A && B && !C => H = M"],
        vec!["H = M => K = D + (D * E / 10)"]);

    let input = Input {
        a: true,
        b: false,
        c: false,
        d: 5.0,
        e: 2,
        f: 1,
    };
    let output = ep.evaluate(&input);
    assert!(output.is_err());
}

#[test]
//this test contains the base substitutions
fn expression_base() {
    let ep = ExpressionProcessor::new(
        vec![
            "A && B && !C => H = M",
            "A && B && C => H = P",
            "!A && B && C => H = T"
        ],
        vec![
            "H = M => K = D + (D * E / 10)",
            "H = P => K = D + (D * (E - F) / 25.5)",
            "H = T => K = D - (D * F / 30)"
        ],
    );

    let input = Input {
        a: true,
        b: true,
        c: true,
        d: 76.5,
        e: 2,
        f: 1,
    };
    let output = ep.evaluate(&input);
    assert_eq!(output.unwrap(), Output { h: 'P', k: 79.5 })
}

#[test]
//this test contains the base substitutions but none applies
fn expression_base_error() {
    let ep = ExpressionProcessor::new(
        vec![
            "A && B && !C => H = M",
            "A && B && C => H = P",
            "!A && B && C => H = T"
        ],
        vec![
            "H = M => K = D + (D * E / 10)",
            "H = P => K = D + (D * (E - F) / 25.5)",
            "H = T => K = D - (D * F / 30)"
        ],
    );

    let input = Input {
        a: true,
        b: false,
        c: true,
        d: 76.5,
        e: 2,
        f: 1,
    };
    let output = ep.evaluate(&input);
    assert!(output.is_err());
}

#[test]
//this test contains the base substitutions and the custom 1
//the values used are the same as the test expression_base
fn expression_custom1_override() {
    let ep = ExpressionProcessor::new(
        vec![
            "A && B && !C => H = M",
            "A && B && C => H = P",
            "!A && B && C => H = T"
        ],
        vec![
            "H = M => K = D + (D * E / 10)",
            "H = P => K = D + (D * (E - F) / 25.5)",
            "H = T => K = D - (D * F / 30)",
            "H = P => K = 2 * D + (D * E / 100)",
        ],
    );

    let input = Input {
        a: true,
        b: true,
        c: true,
        d: 76.5,
        e: 2,
        f: 1,
    };
    let output = ep.evaluate(&input);
    assert_eq!(output.unwrap(), Output { h: 'P', k: 154.53 })
}

#[test]
//this test contains the base substitutions and custom 1 and custom 2
//the values used are the same as the test expression_base
fn expression_custom2_override() {
    let ep = ExpressionProcessor::new(
        vec![
            "A && B && !C => H = M",
            "A && B && C => H = P",
            "!A && B && C => H = T",
            "A && B && !C => H = T",
            "A && !B && C => H = M",
        ],
        vec![
            "H = M => K = D + (D * E / 10)",
            "H = P => K = D + (D * (E - F) / 25.5)",
            "H = T => K = D - (D * F / 30)",
            "H = P => K = 2 * D + (D * E / 100)",
            "H = M => K = F + D + (D * E / 100)",
        ],
    );

    let input = Input {
        a: true,
        b: true,
        c: true,
        d: 76.5,
        e: 2,
        f: 1,
    };
    let output = ep.evaluate(&input);
    assert_eq!(output.unwrap(), Output { h: 'P', k: 154.53 })
}