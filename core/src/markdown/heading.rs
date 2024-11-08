use super::{toc::Toc, util, Hooks};
use comrak::nodes::{AstNode, NodeHtmlBlock, NodeValue};

pub fn process<'a, H: Hooks>(root: &'a AstNode<'a>, toc: &mut Toc) {
    let mut first = None;

    for node in root.children() {
        let (depth, slug, text) = {
            let data = node.data.borrow();
            let NodeValue::Heading(heading) = &data.value else {
                continue;
            };

            first = first.or(Some(node));

            let depth = heading.level;
            let mut text = String::new();
            util::collect_text(node, &mut text);
            let slug = slug::slugify(&text);

            (depth, slug, text)
        };

        let literal = format!(r#"<h{depth} id="{slug}">{text}</h{depth}>"#);
        let mut data = node.data.borrow_mut();

        data.value = NodeValue::HtmlBlock(NodeHtmlBlock {
            block_type: 7,
            literal,
        });

        // This is a raw HTML node now and thus cannot have children.
        for child in node.children() {
            child.detach();
        }

        toc.insert(depth, slug, text);
    }

    if let Some(node) = first {
        let toc_html = H::render_toc(toc);
        let mut data = node.data.borrow_mut();
        if let NodeValue::HtmlBlock(block) = &mut data.value {
            block.literal = format!("{toc_html}{}", block.literal);
        } else {
            unreachable!();
        }
    }
}
