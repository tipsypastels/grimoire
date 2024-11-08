use id_arena::{Arena, Id};

#[derive(Debug)]
pub struct Toc {
    arena: Arena<TocNode>,
    open: [Option<Id<TocNode>>; 6],
}

impl Toc {
    pub(super) fn new() -> Self {
        Self {
            arena: Arena::new(),
            open: [None; 6],
        }
    }

    pub fn roots(&self) -> impl Iterator<Item = &'_ TocNode> {
        self.arena.iter().map(|e| e.1).filter(|node| node.is_root())
    }

    pub fn children<'a>(&'a self, node: &'a TocNode) -> impl Iterator<Item = &'a TocNode> {
        node.children.iter().filter_map(|&id| self.arena.get(id))
    }

    pub(super) fn insert(&mut self, depth: u8, slug: String, text: String) {
        let node_id = self.arena.next_id();
        let mut node = TocNode::new(depth, slug, text);

        if let Some(parent_id) = self.parent_id(depth) {
            if let Some(parent_node) = self.arena.get_mut(parent_id) {
                // Account for levels that may have been skipped.
                node.depth = parent_node.depth + 1;
                parent_node.children.push(node_id);
            }
        }

        self.open[node.depth as usize] = Some(node_id);
        self.arena.alloc(node);
    }

    fn parent_id(&self, depth: u8) -> Option<Id<TocNode>> {
        if depth == 1 {
            return None;
        }

        let parent_id = self.open[depth as usize - 1];
        if parent_id.is_some() {
            return parent_id;
        }

        // At least one level was skipped, fall back to grandparent...
        self.parent_id(depth - 1)
    }
}

#[derive(Debug)]
pub struct TocNode {
    pub depth: u8,
    pub slug: String,
    pub text: String,
    children: Vec<Id<Self>>,
}

impl TocNode {
    fn new(depth: u8, slug: String, text: String) -> Self {
        Self {
            depth,
            slug,
            text,
            children: Vec::new(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.children.is_empty()
    }
}
