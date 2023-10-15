use super::node_management::BinaryTreeNode;
use super::node_types::{Color, NodeType};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::rc::{Rc, Weak};

use crate::dom::core::node_types::comment_node::CommentNode;
use crate::dom::core::node_types::document_node::DocumentNode;
use crate::dom::core::node_types::document_type_node::DocumentTypeNode;
use crate::dom::core::node_types::entity_node::EntityNode;
use crate::dom::core::node_types::entity_reference_node::EntityReferenceNode;
use crate::dom::core::node_types::notation_node::NotationNode;
use crate::dom::node_types::attribute::AttributeNode;
use crate::dom::node_types::document_fragment_node::{
    DocumentFragmentNode, DocumentFragmentNodeType,
};
use crate::dom::node_types::text_node::TextNode;

mod element_node;
mod node_types {
    pub mod c_data_node;
    pub mod processing_instruction_node;
}

use element_node::ElementNode;
use node_types::c_data_node::CDATANode;
use node_types::processing_instruction_node::ProcessingInstructionNodeType;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    Red,
    Black,
}

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
    Attr(String, String),
    CDATASection(String),
    Entity(EntityData),
    EntityReference(EntityReferenceData),
    Document(DocumentData),
    DocumentType(String),
    DocumentFragment(DocumentFragmentData),
    ProcessingInstruction(String, String),
    Notation(String),
    //... other node types
}

#[derive(Debug, PartialEq, Eq)]
struct ElementData {
    tag_name: String,
    attributes: BTreeMap<String, String>,
    namespaces: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
struct EntityData {
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
struct EntityReferenceData {
    reference: String,
}

#[derive(Debug, PartialEq, Eq)]
struct DocumentData {
    doctype: Option<String>,
    children: Vec<Rc<RefCell<BinaryTreeNode>>>,
}

#[derive(Debug, PartialEq, Eq)]
struct DocumentFragmentData {
    children: Vec<Rc<RefCell<BinaryTreeNode>>>,
}

struct Node {
    color: Color,
    parent: Option<Weak<RefCell<Node>>>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    node_type: NodeType,
    children: Option<Box<BinaryTreeNode>>,
}

struct BinaryTreeNode {
    node: Rc<RefCell<Node>>,
    left: Option<Box<BinaryTreeNode>>,
    right: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode {
    fn insert_child(&mut self, child: Rc<RefCell<Node>>) {
        // Logic for inserting child in Binary Tree...
    }

    fn find_child(&self, tag_name: String) -> Option<Rc<RefCell<Node>>> {
        // Logic for searching child in Binary Tree...
    }

    fn search(&self, node_type: &NodeType) -> Option<&BinaryTreeNode> {
        // Detailed Binary Tree search logic here.
        None
    }

    fn delete(&mut self, node_type: &NodeType) {
        // Detailed Binary Tree deletion logic here.
    }

    fn in_order_traversal<F: FnMut(&NodeType)>(&self, mut f: F) {
        // Detailed in-order traversal logic here.
    }

    fn pre_order_traversal<F: FnMut(&NodeType)>(&self, mut f: F) {
        // Detailed pre-order traversal logic here.
    }

    fn post_order_traversal<F: FnMut(&NodeType)>(&self, mut f: F) {
        // Detailed post-order traversal logic here.
    }
}

pub struct HybridDomTree {
    root: Option<Rc<RefCell<Node>>>,
    attributes: AttributeNode,
    document_type: Option<DocumentTypeNode>, // Adding a missing property.
}

impl HybridDomTree {
    pub fn new(root_node_type: NodeType) -> Self {
        let root = Node::new(root_node_type, Color::Black);
        HybridDomTree {
            root: Some(Rc::new(RefCell::new(root))),
            attributes: AttributeNode::new(),
            document_type: None, // A default value for document_type.
        }
    }
    pub fn new() -> Self {
        Self {
            root: None,
            document_type: None, // Initialize the DocumentTypeNode as None
        }
    }
    // Add methods related to NotationNode
    pub fn insert_notation_node(
        &mut self,
        name: String,
        public_id: Option<String>,
        system_id: Option<String>,
    ) -> Result<(), &'static str> {
        if let Some(root) = &self.root {
            root.borrow_mut()
                .insert_notation_node(NotationNode::new(name, public_id, system_id))
                .map_err(|_| "Failed to insert Notation Node")
        } else {
            Err("Root does not exist")
        }
    }

    // Add methods related to EntityNode
    pub fn insert_entity(&mut self, name: String) -> Result<(), &'static str> {
        // Create and insert an EntityNode into the Hybrid Dom
        let entity_node = EntityNode::new(name);
        // Implement logic to insert entity_node into the Hybrid Dom.
        Ok(())
    }

    // Add methods related to CDATA Node
    pub fn insert_cdata(
        &mut self,
        parent_tag_name: String,
        data: String,
    ) -> Result<(), &'static str> {
        // Implement your logic to insert a CDATA section.
        Ok(())
    }

    // Add methods related to Document_fragmentNode
    pub fn insert_document_fragment(&mut self) -> Result<(), &'static str> {
        // Implement your logic to insert a document fragment node.
        // You can use the DocumentFragmentNode module here.
        Ok(())
    }

    // Add methods related to DocumentTypeNode
    pub fn set_document_type(&mut self, document_type: DocumentTypeNode) {
        self.document_type = Some(document_type);
    }

    pub fn get_document_type(&self) -> Option<&DocumentTypeNode> {
        self.document_type.as_ref()
    }

    // Other methods specific to DocumentTypeNode can be added here

    // Comment node
    pub fn insert(
        &mut self,
        parent_tag_name: String,
        node_type: NodeType,
    ) -> Result<(), &'static str> {
        // Implement your logic to insert a node in Red-Black Tree and B-Tree.
        Ok(())
    }

    pub fn delete(&mut self, tag_name: String) -> Result<(), &'static str> {
        // Implement your logic to delete a node from Red-Black Tree and B-Tree.
        Ok(())
    }

    pub fn traverse<F>(&self, action: F)
    where
        F: Fn(&Node),
    {
        // Implement the logic to traverse the HybridDomTree.
    }

    // attribute node

    pub fn set_attribute(&mut self, name: String, value: String) {
        // Delegate to the attribute node
        self.attributes.set_attribute(name, value);
    }

    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        // Delegate to the attribute node
        self.attributes.get_attribute(name)
    }

    pub fn remove_attribute(&mut self, name: &str) {
        // Delegate to the attribute node
        self.attributes.remove_attribute(name);
    }

    // Element Node
    pub fn insert(
        &mut self,
        parent_tag_name: String,
        node_type: NodeType,
    ) -> Result<(), &'static str> {
        // Implement your logic to insert a node in Red-Black Tree and B-Tree.
        Ok(())
    }

    pub fn delete(&mut self, tag_name: String) -> Result<(), &'static str> {
        // Implement your logic to delete a node from Red-Black Tree and B-Tree.
        Ok(())
    }

    pub fn traverse<F>(&self, action: F)
    where
        F: Fn(&Node),
    {
        // Implement the logic to traverse the HybridDomTree.
    }

    // Example of using ElementNode methods/functions
    pub fn create_element_node(&mut self, tag_name: String) -> Rc<RefCell<Node>> {
        // Use ElementNode methods/functions here
        let element_node = ElementNode::create(tag_name);
        // Create a new Node using the ElementNode and insert it into the tree
        // Return the created Node
        element_node.get_node()
    }

    // Text node
    pub fn create_text_node(&mut self, text_content: String) -> NodeType {
        let text_node = TextNode::new(text_content);
        NodeType::TextNode(Rc::new(RefCell::new(text_node)))
    }

    // Other methods for interacting with text nodes
}

impl fmt::Debug for HybridDomTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HybridDomTree")
    }
}

impl Node {
    fn new(node_type: NodeType, color: Color) -> Self {
        Self {
            node_type,
            color,
            parent: None,
            left: None,
            right: None,
            children: None,
        }
    }
    pub fn insert_cdata(
        &mut self,
        parent_tag_name: String,
        data: String,
    ) -> Result<(), &'static str> {
        // Implement your logic to insert a CDATA section.
        Ok(())
    }

    fn insert_notation_node(&mut self, notation_node: NotationNode) -> Result<(), String> {
        let node = Rc::new(RefCell::new(Node::new(
            NodeType::Notation(notation_node),
            Color::Black,
        )));
        self.insert_to_rb_tree(node)
    }

    fn insert_to_rb_tree(&mut self, node: Rc<RefCell<Node>>) -> Result<(), String> {
        // Detailed Red-Black Tree insertion logic here.
        Ok(())
    }

    fn search_in_rb_tree(&self, node_type: &NodeType) -> Option<Rc<RefCell<Node>>> {
        // Detailed Red-Black Tree search logic here.
        None
    }

    fn delete_from_rb_tree(&mut self, node_type: &NodeType) {
        // Detailed Red-Black Tree deletion logic here.
    }

    fn balance_rb_tree(&mut self) {
        // Detailed Red-Black Tree balancing logic here.
    }

    fn bulk_insert_to_rb_tree(&mut self, nodes: Vec<Rc<RefCell<Node>>>) {
        // Detailed Red-Black Tree bulk insertion logic here.
    }

    fn traverse_in_order(&self, visit: impl Fn(&Node)) {
        if let Some(left) = &self.left {
            left.borrow().traverse_in_order(&visit);
        }
        visit(self);
        if let Some(right) = &self.right {
            right.borrow().traverse_in_order(&visit);
        }
    }

    // Other methods like insert_child, find_child, rotate_left, rotate_right, etc.
}
impl ElementData {
    fn new(tag_name: String) -> Self {
        ElementData {
            tag_name,
            attributes: BTreeMap::new(),
            namespaces: HashMap::new(),
        }
    }

    fn add_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }

    fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    fn get_classes(&self) -> Vec<&str> {
        self.attributes
            .get("class")
            .map(|s| s.split_whitespace().collect())
            .unwrap_or_else(Vec::new)
    }
}
