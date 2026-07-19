//! Tests for error report rendering, in particular escaping.

use jaq_all::data;
use jaq_all::load::FileReportsDisp;

/// Render all reports for a filter that fails to compile.
fn render(filter: &str, escape: Option<fn(&str) -> String>) -> String {
    let reports = match data::compile(filter) {
        Ok(_) => panic!("expected filter {filter:?} to fail compilation"),
        Err(reports) => reports,
    };
    reports
        .iter()
        .map(|fr| {
            let disp = FileReportsDisp::new(fr);
            let disp = match escape {
                Some(escape) => disp.with_escape(escape),
                None => disp,
            };
            format!("{disp}")
        })
        .collect()
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// The code snippet in a report echoes the offending source verbatim.
// Without escaping, HTML in the source is reflected as-is; this is what
// the playground would previously insert into `innerHTML`, allowing XSS.
#[test]
fn report_reflects_source_by_default() {
    let out = render("<img src=x onerror=alert(1)>", None);
    assert!(out.contains("<img src=x onerror=alert(1)>"), "{out}");
}

// With an escape function, the reflected source must be escaped, so that
// a consumer rendering into HTML cannot be tricked into executing script.
#[test]
fn report_escapes_source() {
    let out = render("<img src=x onerror=alert(1)>", Some(escape_html));
    assert!(
        !out.contains("<img") && !out.contains("onerror=alert(1)>"),
        "raw HTML leaked into escaped report: {out}"
    );
    assert!(out.contains("&lt;img src=x onerror=alert(1)&gt;"), "{out}");
}

// The report message (not just the snippet) can carry user input, e.g. an
// import path that does not resolve. That branch must be escaped too.
#[test]
fn report_escapes_message() {
    let filter = r#"import "<img src=x onerror=alert(1)>" as m; ."#;
    let raw = render(filter, None);
    assert!(
        raw.contains("could not load file <img src=x onerror=alert(1)>"),
        "{raw}"
    );

    let out = render(filter, Some(escape_html));
    assert!(
        !out.contains("<img"),
        "raw HTML leaked into escaped message: {out}"
    );
    assert!(out.contains("&lt;img src=x onerror=alert(1)&gt;"), "{out}");
}

// Escaping must not touch the report's own structure (the box-drawing
// characters and any markup a paint function adds), only interpolated text.
#[test]
fn report_escape_preserves_structure() {
    let plain = render("1 +", None);
    let escaped = render("1 +", Some(escape_html));
    // this filter's source and messages contain no `& < >`,
    // so escaping is a no-op and the two renderings are identical
    assert_eq!(plain, escaped);
}
