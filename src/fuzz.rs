use crate::prelude::BodyNode;
use ego_tree::NodeRef;
use fuzzcheck::fuzz_test;
use scraper::{ElementRef, Html, Node};

fn test(html: &crate::prelude::Html) -> bool {
    let document = html.to_string();
    let doc = Html::parse_document(&document);
    let root = doc.tree.root().first_child().unwrap();
    if !root.value().is_doctype() {
        return false;
    }
    let html_node = root.next_sibling().unwrap();
    let head = html_node.first_child().unwrap();
    let body = head.next_sibling().unwrap();

    fn check(expected: BodyNode, actual: NodeRef<Node>) -> bool {
        // todo: remove `text` field on `P`.
        if matches!(expected, BodyNode::Text(_)) || actual.value().is_text() {
            return true;
        }
        let actual = ElementRef::wrap(actual).unwrap();
        let tag_name = match expected {
            BodyNode::H1(_) => "h1",
            BodyNode::H2(_) => "h2",
            BodyNode::H3(_) => "h3",
            BodyNode::H4(_) => "h4",
            BodyNode::H5(_) => "h5",
            BodyNode::H6(_) => "h6",
            BodyNode::P(_) => "P",
            BodyNode::Text(_) => unreachable!(),
            BodyNode::Form(_) => "form",
            BodyNode::Br(_) => "br",
            BodyNode::Div(_) => "div",
            BodyNode::A(_) => "a",
            BodyNode::Input(_) => "input",
            BodyNode::Label(_) => "label",
            BodyNode::Select(_) => "select",
            BodyNode::NoScript(_) => "noscript",
            BodyNode::Img(_) => "img",
        };
        let children = match expected {
            BodyNode::H1(_)
            | BodyNode::H2(_)
            | BodyNode::H3(_)
            | BodyNode::H4(_)
            | BodyNode::H5(_)
            | BodyNode::H6(_)
            | BodyNode::Text(_)
            | BodyNode::Br(_)
            | BodyNode::A(_)
            | BodyNode::Input(_)
            | BodyNode::Label(_)
            | BodyNode::Select(_)
            | BodyNode::NoScript(_)
            | BodyNode::Img(_) => vec![],
            BodyNode::Form(f) => f.into_pub_fields().children,
            BodyNode::Div(d) => d.into_pub_fields().children,
            BodyNode::P(p) => p.into_pub_fields().children,
        };

        actual.value().name().to_ascii_lowercase() == tag_name
            && children
                .into_iter()
                .zip(actual.children())
                .all(|(expected, actual)| check(expected, actual))
    }

    for (node, actual) in html
        .clone()
        .into_pub_fields()
        .body
        .into_pub_fields()
        .children
        .into_iter()
        .zip(body.children())
    {
        if !check(node, actual) {
            return false;
        }
    }

    true
}

#[test]
fn test_emit() {
    let res = fuzz_test(test)
        .default_mutator()
        .serde_serializer()
        .default_sensor_and_pool()
        .arguments_from_cargo_fuzzcheck()
        .launch();
    assert!(!res.found_test_failure)
}

mod regressions {
    use super::test;
    use crate::tags::{body::body, head::head, html::html};

    #[test]
    fn regression_1() {
        let html: crate::tags::html::Html = html().head(head()).body(body());
        assert!(test(&html));
    }

    #[test]
    fn regression_2() {
        let html: crate::tags::html::Html = serde_json::from_str(r#"
        {"head":{"children":[{"Title":{"text":"","attrs":{}}}]},"body":{"children":[{"Text":{"text":"","attrs":{}}},{"Input":{"attrs":{}}},{"Text":{"text":"���+","attrs":{"cf":"","l":"","0a2Fk":"","o":"","Z54":"4I","H":"W7","78":"","E":"","72":"","8":"","F":"","w":"","2":"","n6":"Zctw4zb809N","O":"","4":"","U":"","YqGEyOqdtW6B0R43":"q6","1":"","S":"","c":"","3D":"3","":""}}}],"attrs":{"":""}}}
        "#).unwrap();
        assert!(test(&html));
    }

    #[test]
    fn regression_3() {
        let html: crate::tags::html::Html = serde_json::from_str(
            r#"{"head":{"children":[]},"body":{"children":[{"Img":{"attrs":{}}}],"attrs":{}}}"#,
        )
        .unwrap();
        assert!(test(&html));
    }
}
