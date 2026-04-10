mod common;

use serde_json::json;

/// Test that Kotlin files can be parsed and classes are extracted
#[test]
fn test_parse_kotlin_extracts_classes() {
    let file_path = common::fixture_path("kotlin", "Calculator.kt");
    let arguments = json!({
        "file_path": file_path.to_str().unwrap(),
        "detail": "signatures",
        "include_deps": false,
        "max_tokens": 10_000
    });

    let result = treesitter_mcp::analysis::view_code::execute(&arguments)
        .expect("parse_file should succeed for Kotlin file");

    let text = common::get_result_text(&result);
    let shape: serde_json::Value =
        serde_json::from_str(&text).expect("Result should be valid JSON");

    // Verify classes are extracted (compact `c` rows)
    let rows_str = shape.get("c").and_then(|v| v.as_str()).unwrap_or("");
    let rows = common::helpers::parse_compact_rows(rows_str);

    let class_names: Vec<&str> = rows
        .iter()
        .filter_map(|r| r.first().map(|s| s.as_str()))
        .collect();

    assert!(
        class_names.contains(&"Calculator"),
        "Should find Calculator class, found: {:?}",
        class_names
    );

    assert!(
        class_names.contains(&"MathUtils"),
        "Should find MathUtils object, found: {:?}",
        class_names
    );

    assert!(
        class_names.contains(&"CalculationResult"),
        "Should find CalculationResult data class, found: {:?}",
        class_names
    );
}

/// Test that Kotlin methods are extracted from classes
#[test]
fn test_parse_kotlin_extracts_methods() {
    let file_path = common::fixture_path("kotlin", "Calculator.kt");
    let arguments = json!({
        "file_path": file_path.to_str().unwrap(),
        "detail": "signatures",
        "include_deps": false,
        "max_tokens": 10_000
    });

    let result = treesitter_mcp::analysis::view_code::execute(&arguments)
        .expect("parse_file should succeed for Kotlin file");

    let text = common::get_result_text(&result);
    let shape: serde_json::Value =
        serde_json::from_str(&text).expect("Result should be valid JSON");

    // Verify methods are extracted (compact `cm` rows: class_name, method_name, ...)
    let rows_str = shape.get("cm").and_then(|v| v.as_str()).unwrap_or("");
    let rows = common::helpers::parse_compact_rows(rows_str);

    let expected_methods = vec!["add", "subtract", "multiply", "divide", "getLastResult"];
    for method_name in expected_methods {
        let found = rows.iter().any(|r| {
            r.get(0).map(|s| s.as_str()) == Some("Calculator")
                && r.get(1).map(|s| s.as_str()) == Some(method_name)
        });
        assert!(found, "Should find method '{method_name}' on Calculator");
    }
}

/// Test that top-level Kotlin functions are extracted
#[test]
fn test_parse_kotlin_extracts_top_level_functions() {
    let file_path = common::fixture_path("kotlin", "Calculator.kt");
    let arguments = json!({
        "file_path": file_path.to_str().unwrap(),
        "detail": "signatures",
        "include_deps": false,
        "max_tokens": 10_000
    });

    let result = treesitter_mcp::analysis::view_code::execute(&arguments)
        .expect("parse_file should succeed for Kotlin file");

    let text = common::get_result_text(&result);
    let shape: serde_json::Value =
        serde_json::from_str(&text).expect("Result should be valid JSON");

    // Verify top-level functions (compact `f` rows)
    let rows_str = shape.get("f").and_then(|v| v.as_str()).unwrap_or("");
    let rows = common::helpers::parse_compact_rows(rows_str);

    let func_names: Vec<&str> = rows
        .iter()
        .filter_map(|r| r.first().map(|s| s.as_str()))
        .collect();

    assert!(
        func_names.contains(&"formatResult"),
        "Should find top-level function 'formatResult', found: {:?}",
        func_names
    );
}

/// Test that Kotlin imports are extracted
#[test]
fn test_parse_kotlin_extracts_imports() {
    let file_path = common::fixture_path("kotlin", "Calculator.kt");
    let arguments = json!({
        "file_path": file_path.to_str().unwrap(),
        "detail": "signatures",
        "include_deps": false,
        "max_tokens": 10_000
    });

    let result = treesitter_mcp::analysis::view_code::execute(&arguments)
        .expect("parse_file should succeed for Kotlin file");

    let text = common::get_result_text(&result);
    let shape: serde_json::Value =
        serde_json::from_str(&text).expect("Result should be valid JSON");

    // Verify imports (compact `im` rows)
    let rows_str = shape.get("im").and_then(|v| v.as_str()).unwrap_or("");

    assert!(
        rows_str.contains("kotlin.math.sqrt"),
        "Should find import for kotlin.math.sqrt, found: {:?}",
        rows_str
    );
    assert!(
        rows_str.contains("kotlin.math.pow"),
        "Should find import for kotlin.math.pow, found: {:?}",
        rows_str
    );
}

/// Test that real-world Kotlin file (SnowflakeGenerator) can be parsed
#[test]
fn test_parse_kotlin_real_world_file() {
    let file_path = common::fixture_path("kotlin", "SnowflakeGenerator.kt");
    let arguments = json!({
        "file_path": file_path.to_str().unwrap(),
        "detail": "signatures",
        "include_deps": false,
        "max_tokens": 10_000
    });

    let result = treesitter_mcp::analysis::view_code::execute(&arguments)
        .expect("parse_file should succeed for real-world Kotlin file");

    let text = common::get_result_text(&result);
    let shape: serde_json::Value =
        serde_json::from_str(&text).expect("Result should be valid JSON");

    // Verify SnowflakeGenerator class is found
    let rows_str = shape.get("c").and_then(|v| v.as_str()).unwrap_or("");
    let rows = common::helpers::parse_compact_rows(rows_str);

    let class_names: Vec<&str> = rows
        .iter()
        .filter_map(|r| r.first().map(|s| s.as_str()))
        .collect();

    assert!(
        class_names.contains(&"SnowflakeGenerator"),
        "Should find SnowflakeGenerator class, found: {:?}",
        class_names
    );

    // Verify methods
    let cm_str = shape.get("cm").and_then(|v| v.as_str()).unwrap_or("");
    let cm_rows = common::helpers::parse_compact_rows(cm_str);

    let method_on_class: Vec<(&str, &str)> = cm_rows
        .iter()
        .filter_map(|r| {
            if r.len() >= 2 {
                Some((r[0].as_str(), r[1].as_str()))
            } else {
                None
            }
        })
        .collect();

    assert!(
        method_on_class.contains(&("SnowflakeGenerator", "nextId")),
        "Should find nextId method on SnowflakeGenerator, found: {:?}",
        method_on_class
    );
}

/// Test basic language detection for .kt files
#[test]
fn test_detect_kotlin_language() {
    use treesitter_mcp::parser::{detect_language, Language};

    let lang = detect_language("Test.kt").unwrap();
    assert_eq!(lang, Language::Kotlin);

    let lang = detect_language("build.gradle.kts").unwrap();
    assert_eq!(lang, Language::Kotlin);
}
