use crate::ast_validation::view_as::ViewAs;
use ast::{Data, Node};
use std::fmt;

pub enum ValidatorError {
    MalformedAst(String),
    MoreChildrenRequestedThanGuaranteed(String),
}

impl fmt::Display for ValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            ValidatorError::MalformedAst(message) => message,
            ValidatorError::MoreChildrenRequestedThanGuaranteed(message) => message,
        };

        write!(f, "AST Error: {}", message)
    }
}

pub struct NodeValidator<'a> {
    node: &'a Node,
    node_name: &'a str,
}

pub struct ParentNodeValidator<'a> {
    node: &'a Node,
    node_name: &'a str,
    children: &'a [Node],
    guaranteed_size: usize,
    next_child: usize,
}

impl<'a> NodeValidator<'a> {
    pub fn new(node: &'a Node, node_name: &'a str) -> Self {
        NodeValidator { node, node_name }
    }

    pub fn has_children(&self, number: usize) -> Result<ParentNodeValidator<'a>, ValidatorError> {
        // make sure it has children
        if let Data::Children(children) = self.node.data() {
            if children.len() == number {
                return Ok(ParentNodeValidator::new(
                    self.node,
                    self.node_name,
                    children,
                    number,
                ));
            } else {
                return Err(ValidatorError::MalformedAst(format!(
                    "{} node {:?} requires {} children, found {}",
                    self.node_name,
                    self.node,
                    number,
                    children.len()
                )));
            }
        } else {
            return Err(ValidatorError::MalformedAst(format!(
                "{} node {:?} requires children but found {:?}",
                self.node_name,
                self.node,
                self.node.data()
            )));
        }
    }
    /*
        pub fn then_list_of_optional_ints(&self, list: &mut Vec<Option<i64>>) -> Result<(), ValidatorError> {
            // This is only used to processes dimension lists
            // so if it should be that lists can't mix epsilon and non epsilon dimensions, then
            // change the validation here. Actually no because that's a SEMANTIC error not some internal error

            // node can be epsilon for an empty list, or it can have zero or more children
            match self.node.data() {
                Data::Epsilon => {return Ok(());},
                Data::Children(children) => {
                    for child in children {
                        match child.data() {
                            Data::Epsilon => {list.push(None)},
                            Data::Integer(int) => {list.push(Some(*int))},
                            _ => {
                                return Err(ValidatorError::MalformedAst(format!(
                                    "{} node requires all children nodes to be epsilon or integers, found {:?}",
                                    self.node_name,
                                    child
                                )))
                            }
                        }
                    }
                },
                _ => {
                    return Err(ValidatorError::MalformedAst(format!(
                        "{} node must either be an epsilon node or have children, found {:?}",
                        self.node_name,
                        self.node
                    )))
                }
            }

            Ok(())
        }
    */
    pub fn then_list_of_optional_ints(&self) -> Result<Vec<Option<i64>>, ValidatorError> {
        // This is only used to processes dimension lists
        // so if it should be that lists can't mix epsilon and non epsilon dimensions, then
        // change the validation here. Actually no because that's a SEMANTIC error not some internal error
        let mut result = Vec::new();

        // node can be epsilon for an empty list, or it can have zero or more children
        match self.node.data() {
            Data::Epsilon => {
                return Ok(result);
            }
            Data::Children(children) => {
                for child in children {
                    match child.data() {
                        Data::Epsilon => {result.push(None)},
                        Data::Integer(int) => {result.push(Some(*int))},
                        _ => {
                            return Err(ValidatorError::MalformedAst(format!(
                                "{} node requires all children nodes to be epsilon or integers, found {:?}",
                                self.node_name,
                                child
                            )))
                        }
                    }
                }
            }
            _ => {
                return Err(ValidatorError::MalformedAst(format!(
                    "{} node must either be an epsilon node or have children, found {:?}",
                    self.node_name, self.node
                )))
            }
        }

        Ok(result)
    }

    pub fn then_list_of_strings(&self) -> Result<Vec<&'a str>, ValidatorError> {
        let mut result = Vec::new();

        match self.node.data() {
            Data::Epsilon => {
                return Ok(result);
            }
            Data::Children(children) => {
                for child in children {
                    match child.data() {
                        Data::String(s) => result.push(s),
                        _ => {
                            return Err(ValidatorError::MalformedAst(format!(
                                "{} node requires all children nodes to be strings, found {:?}",
                                self.node_name, child
                            )));
                        }
                    }
                }
            }
            _ => {
                return Err(ValidatorError::MalformedAst(format!(
                    "{} node must either be an epsilon node or have children, found {:?}",
                    self.node_name, self.node
                )));
            }
        }

        Ok(result)
    }

    pub fn then_list_of<T: ViewAs<'a>>(&self) -> Result<Vec<T>, ValidatorError> {
        let mut result = Vec::new();

        match self.node.data() {
            Data::Epsilon => return Ok(result),
            Data::Children(children) => {
                for child in children {
                    result.push(ViewAs::view_as(child)?)
                }
            }
            _ => {
                return Err(ValidatorError::MalformedAst(format!(
                    "{} node must either be an epsilon node or have children, found {:?}",
                    self.node_name, self.node
                )))
            }
        }
        Ok(result)
    }
}

impl<'a> ParentNodeValidator<'a> {
    fn new(
        node: &'a Node,
        node_name: &'a str,
        children: &'a [Node],
        guaranteed_size: usize,
    ) -> Self {
        ParentNodeValidator {
            node,
            node_name,
            children,
            guaranteed_size,
            next_child: 0,
        }
    }

    fn verify_child_available(&self) -> Result<(), ValidatorError> {
        // make sure we haven't consumed too many children
        if self.next_child == self.guaranteed_size {
            return Err(ValidatorError::MoreChildrenRequestedThanGuaranteed(
                format!(
                    "Requested more children than the {} available for node {}: {:?}",
                    self.guaranteed_size, self.node_name, self.node
                ),
            ));
        }
        Ok(())
    }

    pub fn then_string(&mut self) -> Result<&'a str, ValidatorError> {
        self.verify_child_available()?;

        if let Data::String(s) = self.children[self.next_child].data() {
            self.next_child += 1;
            Ok(s)
        } else {
            return Err(ValidatorError::MalformedAst(format!(
                "{} node {:?} requires child {} to be a string node, found {:?}",
                self.node_name,
                self.node,
                self.next_child,
                self.children[self.next_child].data()
            )));
        }
    }

    pub fn then_optional_string(&mut self) -> Result<Option<&'a str>, ValidatorError> {
        self.verify_child_available()?;

        match self.children[self.next_child].data() {
            Data::String(s) => {
                self.next_child += 1;
                Ok(Some(s))
            }
            Data::Epsilon => {
                self.next_child += 1;
                Ok(None)
            }
            _ => {
                Err(ValidatorError::MalformedAst(format!(
                    "{} node {:?} requires child {} to be a string or epsilon node, found {:?}",
                    self.node_name,
                    self.node,
                    self.next_child,
                    self.children[self.next_child].data()
                )))
            }
        }
    }

    pub fn then_node(&mut self) -> Result<&'a Node, ValidatorError> {
        self.verify_child_available()?;
        let result = &self.children[self.next_child];
        self.next_child += 1;
        Ok(result)
    }

    pub fn then<T: ViewAs<'a>>(&mut self) -> Result<T, ValidatorError> {
        self.verify_child_available()?;
        let result = ViewAs::view_as(&self.children[self.next_child])?;
        self.next_child += 1;
        Ok(result)
    }

    pub fn then_optional<T: ViewAs<'a>>(&mut self) -> Result<Option<T>, ValidatorError> {
        self.verify_child_available()?;
        match self.children[self.next_child].data() {
            Data::Epsilon => {
                self.next_child += 1;
                Ok(None)
            },
            _ => {
                let result = ViewAs::view_as(&self.children[self.next_child])?;
                self.next_child += 1;
                Ok(Some(result))
            }
        }
    }
}
