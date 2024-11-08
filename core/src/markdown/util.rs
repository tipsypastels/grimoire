use comrak::nodes::{AstNode, NodeCode, NodeValue};

pub fn collect_text<'a>(node: &'a AstNode<'a>, output: &mut String) {
    match node.data.borrow().value {
        NodeValue::Text(ref literal) | NodeValue::Code(NodeCode { ref literal, .. }) => {
            output.push_str(literal)
        }
        NodeValue::LineBreak | NodeValue::SoftBreak => output.push(' '),
        _ => {
            for n in node.children() {
                collect_text(n, output);
            }
        }
    }
}
