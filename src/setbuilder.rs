use std::ops::Range;

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    None
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comparator {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    None
}

#[derive(Debug, Clone)]
pub struct ElementOperation {
    pub operation: Operation,
    pub identifier: String,
    // TODO: actual equation parsing?
    pub operand: f32
}

impl ElementOperation {
    pub fn new() -> Self {
        ElementOperation { operation: Operation::None, identifier: String::new(), operand: f32::INFINITY }
    }
    pub fn evaluate(&self) {}
}

#[derive(Debug, Clone)]
pub struct ElementFilter {
    pub operation: Operation,
    pub identifier: String,
    pub operand: f32,
    pub comparison: Comparator,
    pub compare_to: f32
}

impl ElementFilter {
    pub fn new() -> Self {
        ElementFilter { operation: Operation::None, identifier: String::new(), operand: f32::INFINITY, comparison: Comparator::None, compare_to: f32::INFINITY }
    }
    pub fn evaluate(&self) {}
}

#[derive(Debug, Clone)]
pub struct SetBuilder {
    pub range: Range<i64>,
    pub element_operation: ElementOperation,
    pub element_filter: ElementFilter
}

impl SetBuilder {
    pub fn new(range: Range<i64>, element_operation: ElementOperation, element_filter: ElementFilter) -> Self {
        SetBuilder{range, element_filter, element_operation}
    }
}