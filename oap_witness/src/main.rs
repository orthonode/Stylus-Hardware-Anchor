use sha2::{Sha256, Digest};
use serde_json::Value;
use serde_json_canonicalizer::to_vec;
use std::env;

/// Recursive check to ensure NO JSON Numbers exist in the tree.
/// All numeric values must be strings as per VER v1.0 spec.
fn assert_no_numbers(v: &Value) {
    match v {
        Value::Number(_) => panic!("SPEC VIOLATION: JSON Number detected. All numbers must be fixed-point strings."),
        Value::Array(arr) => for i in arr { assert_no_numbers(i); },
        Value::Object(map) => for (_, val) in map { assert_no_numbers(val); },
        _ => {}
    }
}

/// Enforce the presence of a nested field and return it.
fn require_field<'a>(v: &'a Value, path: &[&str]) -> &'a Value {
    let mut cur = v;
    for key in path {
        cur = cur.get(*key).expect(&format!("SCHEMA ERROR: Missing required field path: {:?}", path));
    }
    cur
}

fn main() {
    // 1. Capture Raw Input (from Argument or Stdin)
    let args: Vec<String> = env::args().collect();
    let raw_ver = if args.len() > 1 {
        args[1].clone()
    } else {
        use std::io::{self, Read};
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("FAILED: Could not read from stdin");
        buffer.trim().to_string()
    };

    if raw_ver.is_empty() {
        eprintln!("Usage: oap_witness <VER_JSON> or pipe JSON into it.");
        std::process::exit(1);
    }

    // 2. Initial Parse
    let json_value: Value = serde_json::from_str(&raw_ver)
        .expect("SYNTAX ERROR: Invalid JSON format");

    // 3. HARD AUDIT: Version Lock
    assert_eq!(
        json_value["version"].as_str().unwrap_or(""),
        "1.0",
        "SPEC ERROR: Unsupported VER version (Expected '1.0')"
    );

    // 4. HARD AUDIT: Schema Compliance
    require_field(&json_value, &["context", "engine"]);
    require_field(&json_value, &["context", "logic_hash"]);
    require_field(&json_value, &["input"]);
    require_field(&json_value, &["output"]);

    // 5. HARD AUDIT: Determinism Enforcement (No Floats)
    assert_no_numbers(&json_value);

    // 6. RFC 8785 Canonicalization
    let canonical_bytes = to_vec(&json_value)
        .expect("INTERNAL ERROR: Canonicalization failed despite validation");

    // 7. SHA-256 Hashing
    let mut hasher = Sha256::new();
    hasher.update(&canonical_bytes);
    let result = hasher.finalize();

    // 8. FINAL OUTPUT: Lowercase Hex (Immutable Receipt ID)
    // The {:x} format specifier ensures lowercase per VER spec.
    println!("{:x}", result);
}
