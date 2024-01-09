use crate::setbuilder::{SetBuilder, Operation, Comparator};

pub trait CodeGenerator {
    fn new(ast: SetBuilder) -> Self;
    fn generate(&mut self) -> &String;
}

pub struct CGenerator {
    output: String,
    ast: SetBuilder,
}

pub struct X86_64Generator {
    output: String,
    ast: SetBuilder,
}

impl CGenerator {
    fn generate_element_operation(&mut self) {
        let generated = match self.ast.element_operation.operation {
            Operation::Add => format!("{} += {:.5}f", self.ast.element_operation.identifier, self.ast.element_operation.operand),
            Operation::Subtract => format!("{} -= {:.5}f", self.ast.element_operation.identifier, self.ast.element_operation.operand),
            Operation::Multiply => format!("{} *= {:.5}f", self.ast.element_operation.identifier, self.ast.element_operation.operand),
            Operation::Divide => format!("{} /= {:.5}f", self.ast.element_operation.identifier, self.ast.element_operation.operand),
            Operation::Modulus => format!("{} = fmod({}, {:.5}f)", self.ast.element_operation.identifier, self.ast.element_operation.identifier, self.ast.element_operation.operand),
            Operation::Power => format!("{} = pow({},{:.5}f)", self.ast.element_operation.identifier, self.ast.element_operation.identifier, self.ast.element_operation.operand),
            Operation::None => String::from(""),
        };
        self.output = self.output.replace("ELEMENT_OPERATION_DEFINITION", generated.as_str());
    }

    fn generate_element_filter(&mut self) {
        let operation = match self.ast.element_filter.operation {
            Operation::Add => format!("{} + {:.5}f", self.ast.element_filter.identifier, self.ast.element_filter.operand),
            Operation::Subtract => format!("{} - {:.5}f", self.ast.element_filter.identifier, self.ast.element_filter.operand),
            Operation::Multiply => format!("{} * {:.5}f", self.ast.element_filter.identifier, self.ast.element_filter.operand),
            Operation::Divide => format!("{} / {:.5}f", self.ast.element_filter.identifier, self.ast.element_filter.operand),
            Operation::Modulus => format!("fmod({}, {:.5}f)", self.ast.element_filter.identifier, self.ast.element_filter.operand),
            Operation::Power => format!("pow({},{:.5}f)", self.ast.element_filter.identifier, self.ast.element_filter.operand),
            Operation::None => String::from("1"),
        };
        let comparison = match self.ast.element_filter.comparison {
            Comparator::Less => format!("< {:.5}f", self.ast.element_filter.compare_to),
            Comparator::LessEqual => format!("<= {:.5}f", self.ast.element_filter.compare_to),
            Comparator::Greater => format!("> {:.5}f", self.ast.element_filter.compare_to),
            Comparator::GreaterEqual => format!(">= {:.5}f", self.ast.element_filter.compare_to),
            Comparator::Equal => format!("== {:.5}f", self.ast.element_filter.compare_to),
            Comparator::NotEqual => format!("!= {:.5}f", self.ast.element_filter.compare_to),
            Comparator::None => String::from(""),
        };
        self.output = self.output.replace("ELEMENT_FILTER_CONDITION", format!("{operation} {comparison}").as_str());
    }

    fn generate_range(&mut self) {
        self.output = self.output.replace("RANGE_START", self.ast.range.start.to_string().as_str());
        self.output = self.output.replace("RANGE_END", self.ast.range.end.to_string().as_str());
    }
}

impl CodeGenerator for CGenerator {
    fn new(ast: SetBuilder) -> Self {
        CGenerator { output: String::from(include_str!("c_codegen_template.c")), ast }
    }

    fn generate(&mut self) -> &String {
        self.generate_element_filter();
        self.generate_element_operation();
        self.generate_range();
        &self.output
    }
}

impl CodeGenerator for X86_64Generator {
    fn new(ast: SetBuilder) -> Self {
        X86_64Generator { output: String::from(""), ast }
    }

    fn generate(&mut self) -> &String {
        todo!()
        // &self.output
    }
}