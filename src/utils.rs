use roxmltree::Document;
use rusoto_core::RusotoError;

pub fn name_query(query: &Option<String>, exact_q: &Option<String>) -> Option<Vec<String>> {
    let input = query.as_ref().map(|q| split(q, false));
    let exact_input = exact_q.as_ref().map(|q| split(q, true));
    if input.is_none() && exact_input.is_none() {
        None
    } else {
        Some([input.unwrap_or_default(), exact_input.unwrap_or_default()].concat())
    }
}
fn split(q: &str, is_exact: bool) -> Vec<String> {
    let format = |s: &str| {
        if is_exact {
            s.to_string()
        } else {
            format!("*{}*", s)
        }
    };
    q.split(',').map(|s| format(s)).collect()
}

#[test]
fn test_name_query() {
    assert_eq!(
        name_query(&Some("test".to_string()), &None),
        Some(vec!["*test*".to_string()])
    );
    assert_eq!(
        name_query(&Some("api,test".to_string()), &None),
        Some(vec!["*api*".to_string(), "*test*".to_string()])
    );
    assert_eq!(
        name_query(&None, &Some("api".to_string())),
        Some(vec!["api".to_string()])
    );
}

pub fn err_handler<E>(error: RusotoError<E>) -> String {
    match error {
        RusotoError::Unknown(ref e) => {
            let doc = Document::parse(&e.body_as_str()).unwrap();
            let finder = |s: &str| {
                doc.descendants()
                    .find(|n| n.has_tag_name(s))
                    .map(|n| n.text())
                    .flatten()
                    .unwrap_or("unknown")
            };
            format!(
                "[ERROR] code:{}, message: {}",
                finder("Code"),
                finder("Message")
            )
        }
        _ => "[ERROR] unknown error".to_string(),
    }
}