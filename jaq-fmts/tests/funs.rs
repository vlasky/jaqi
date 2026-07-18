pub mod common;

use serde_json::json;

// 41 = 0x29
yields!(fromcbor1, "[41] | tobytes | fromcbor", -10);
yields!(fromcbor2, "[99, 230, 176, 180] | tobytes | fromcbor", "水");
yields!(tocbor, "-10 | tocbor | . == ([41] | tobytes)", true);

// 129 = 0x81 (array of length 1), 128 = 0x80 (array of length 0)
yields!(
    fromcbor_deep,
    "[limit(127; repeat(129)), 128] | tobytes | fromcbor | getpath([limit(127; repeat(0))])",
    json!([])
);
yields!(
    fromcbor_too_deep,
    "[limit(129; repeat(129))] | tobytes | try fromcbor catch endswith(\"maximal nesting depth (128) exceeded\")",
    true
);

yields!(
    fromxml_deep,
    r#""<a>" * 128 + "x" + "</a>" * 128 | fromxml | getpath([limit(256; repeat("c", 0))])"#,
    "x"
);
yields!(
    fromxml_too_deep,
    r#""<a>" * 129 | try fromxml catch (contains("maximal nesting depth (128) exceeded by a (at "))"#,
    true
);

// "---" starts a new value, "..." ends a previous value
yields!(fromyaml_doc1, r#""---\n1" | fromyaml"#, 1);
yields!(fromyaml_doc2, r#""1\n..." | fromyaml"#, 1);
yields!(fromyaml_doc3, r#""---\n1\n..." | fromyaml"#, 1);
yields!(fromyaml_nan, r#"".nan" | fromyaml | isnan"#, true);
yields!(fromyaml_inf, r#"".inf" | fromyaml == infinite"#, true);
yields!(fromyaml_ninf, r#""-.inf" | fromyaml == -infinite"#, true);
yields!(fromyaml_bytes, r#""!!binary SGkh" | fromyaml"#, "Hi!");
yields!(fromyaml_str, r#""abc" | fromyaml"#, "abc");
yields!(
    fromyaml_arr,
    r#""[0.0, abc]" | fromyaml"#,
    json!([0.0, "abc"])
);
yields!(
    fromyaml_obj,
    r#""{\"a\":1,true: 2,3: 4}" | fromyaml | keys"#,
    json!([true, 3, "a"])
);
yields!(fromyaml_none, r#""" | [fromyaml]"#, json!([]));
yields!(fromyaml_many, r#""1\n---\n2" | [fromyaml]"#, [1, 2]);

yields!(toyaml_nan, "nan | toyaml", ".nan");
yields!(toyaml_inf, "[infinite,-infinite] | toyaml", "[.inf, -.inf]");
yields!(toyaml_bytes, r#""Hi!" | tobytes | toyaml"#, "!!binary SGkh");
yields!(toyaml_str, r#""abc" | toyaml"#, r#"abc"#);
yields!(toyaml_arr, r#"[0.0, "abc"] | toyaml"#, r#"[0.0, abc]"#);
yields!(
    toyaml_obj,
    "{a: 1, (true): 2, (3): 4} | toyaml",
    "{a: 1, true: 2, 3: 4}"
);
yields!(
    toyaml_ff,
    r"255 | tobytes | tostring | toyaml | explode",
    [-255]
);
