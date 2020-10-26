use super::*;

#[test]
fn create_h_substitution() {
    let hs = HSubstitution::new("A && B && !C => H = M");
    assert_eq!(hs.unwrap(), HSubstitution{ input: "A && B && !C".to_string(), output: 'M' })
}

#[test]
fn create_k_substitution() {
    let hs = KSubstitution::new("H = M => K = D + (D * E / 10)");
    assert_eq!(hs.unwrap(), KSubstitution{ input: 'M', output: "K = D + (D * E / 10)".to_string() })
}

#[test]
fn matching_h_substitution() {
    let hs = HSubstitution::new("A && B && !C => H = T").unwrap();
    let context: HashMapContext = context_map! {
            "A" => true,
            "B" => true,
            "C" => false,
            "D" => 1,
            "E" => 1,
            "F" => 1
        }.unwrap();
    let result = hs.evaluate(&context);
    assert_eq!(result.unwrap(), 'T');
}

#[test]
fn not_matching_h_substitution() {
    let hs = HSubstitution::new("A && B && !C => H = T").unwrap();
    let context: HashMapContext = context_map! {
            "A" => true,
            "B" => false,
            "C" => false,
            "D" => 1,
            "E" => 1,
            "F" => 1
        }.unwrap();
    let result = hs.evaluate(&context);
    assert!(result.is_none());
}

#[test]
fn matching_k_substitution() {
    let hs = KSubstitution::new("H = M => K = F + D + (D * E / 100)").unwrap();
    let mut context: HashMapContext = context_map! {
            "A" => true,
            "B" => true,
            "C" => false,
            "D" => 2.0,
            "E" => 100,
            "F" => 1
        }.unwrap();
    let result = hs.evaluate('M', &mut context);
    assert_eq!(result.unwrap(), 5.0);
}

#[test]
fn not_matching_k_substitution() {
    let hs = KSubstitution::new("H = T => K = F + D + (D * E / 100)").unwrap();
    let mut context: HashMapContext = context_map! {
            "A" => true,
            "B" => true,
            "C" => false,
            "D" => 2.0,
            "E" => 100,
            "F" => 1
        }.unwrap();
    let result = hs.evaluate('M', &mut context);
    assert!(result.is_none());
}
