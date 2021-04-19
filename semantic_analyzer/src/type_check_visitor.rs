
//! Find the sizes needed to reserve adequate memory for the program to run
//! Add literal and temporary values to the symbol table

use crate::SemanticAnalysisResults;
use crate::{SymbolTable, SymbolTableEntry, Temporary, Literal, LiteralValue};
use ast::{Data, Node};
use output_manager::OutputConfig;
use crate::SemanticError;
use log::info;
use std::convert::TryInto;

const INTEGER: &str = "integer";
const FLOAT: &str = "float";
const STRING: &str = "string";

pub fn process(
    node: &mut Node,
    current_results: &mut SemanticAnalysisResults,
    output: &mut OutputConfig,
) {
    info!("Starting type check");
    visit(node, &mut current_results.symbol_table.clone(), &mut State {}, &mut current_results.symbol_table, output)
}

pub struct State {}

// Pass the global context around as a clone
// When a node arrives that mutates a single table, it must replace the

// Pass in a global table and a local table
// when an update is made to the local table
// copy the local table back into the global table

pub fn visit(node: &mut Node, current_context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // I don't think this needs to traverse the AST again
    // just iterate over the symbol table since it contains type information

    // FOR SETTING THE SIZE OF ALREADY EXISTENT VARIABLES:
    // just iterate over the symbol table

    // For the creation of temporary values in the symbol table
    // Visitor pattern

    match node.name().as_str() {
        "prog" => prog(node, current_context, state, global_table, output),
        "funcDecl" => func_decl(node, current_context, state, global_table, output),
        "classDeclList" => class_list(node, current_context, state, global_table, output),
        "funcDefList" => function_list(node, current_context, state, global_table, output),
        "funcBody" => func_body(node, current_context, state, global_table, output),
        "statBlock" => stat_block(node, current_context, state, global_table, output),
        "assignOp" => assign_op(node, current_context, state, global_table, output),
        "varList" => var_list(node, current_context, state, global_table, output),
        "var" => var(node, current_context, state, global_table, output),
        "dataMember" => data_member(node, current_context, state, global_table, output),
        "addOp" => add_op(node, current_context, state, global_table, output),
        "mulOp" => mul_op(node, current_context, state, global_table, output),
        "intfactor" => intfactor(node, current_context, state, global_table, output),
        "floatfactor" => floatfactor(node, current_context, state, global_table, output),
        "stringfactor" => stringfactor(node, current_context, state, global_table, output),
        "type" => type_node(node, current_context, state, global_table, output),
        "varDecl" => var_decl(node, current_context, state, global_table, output),
        "id" => id(node, current_context, state, global_table, output),
        "indexList" => mandatory_indexlist(node, current_context, state, global_table, output),
        "writeStat" => write_stat(node, current_context, state, global_table, output),
        "readStat" => read_stat(node, current_context, state, global_table, output),
        "fCall" => f_call(node, current_context, state, global_table, output),
        "aParams" => a_params_children(node, current_context, state, global_table, output),
        _ => {}
    }
}

fn prog(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("prog");
    
    // Here we'll explicitly invoke the individual children
    if let Data::Children(children) = node.data_mut() {
        visit(&mut children[0], context, state, global_table, output);
        visit(&mut children[0], context, state, global_table, output);

        if let Some(SymbolTableEntry::Function(main)) = context.get_mut("main") {
            entry_point(&mut children[2], main.symbol_table_mut(), state, global_table, output);

            *global_table = context.clone();
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

fn entry_point(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("entry_point");

    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
    }
}

fn class_list(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("class_list");

    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn function_list(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("function_list");

    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn var_list(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("var_list");

    
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn func_body(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("func_body");

    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn stat_block(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("stat_block");

    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn assign_op(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("assign_op");
    let line = *node.line();
    let col = *node.column();

    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        info!("Calling check bin from assign op");

        if let Ok(d_type) = check_binary_types(&children[0], &children[1], output, line, col) {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }
    }
}

fn var_decl(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("var_decl");

    if let Data::Children(children) = node.data_mut() {
        // for child in children {
        //     visit(child, context, state, global_table, output);
        // }

        // Manual processing of children to handle the correct context of dimlist
        visit(&mut children[0], context, state, global_table, output);
        visit(&mut children[1], context, state, global_table, output);
        mandatory_dimlist(&mut children[2], context, state, global_table, output);
        // We're gonna need to reach into the context symbol table


    }

    // TODO: More validation may have to be done here to verify dimension list
}

fn var(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    
    info!("var");

    // This is where a lot of complexity accumulates
    // each child represents one element in a sequence of
    // id.id.id...

    // We need to assure
    // The node the precedes a . is a class type
    // the node that follows a . is a member of the class
    
    // That means we should process them as a windows(2)

    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        // TODO: This is very good! Come back to it in a bit
        // for pair in children.windows(2) {


        //     // left one needs to name a class
        //     if let (Some(ld_type), Some(rd_type)) = (pair[0].data_type(), pair[1].data_type()) {
        //         match global_table.get(&ld_type) {
        //             Some(SymbolTableEntry::Class(class)) => {
        //                 // Now we must be sure that the id of the next node is valid in the scope of the class

        //             }, // Ok!
        //             Some(entry) => (), // trying to use a dot operator on a non composite type
        //             None => (), // Undefined identifer? Shouldn't happen since the dataMember or fCall needs to look up the id to be valid and the id must be a local
        //         }
        //     } else {
        //         // There's a problem
        //     }
            
        // }
        
        // if children.len() > 1 {
        //     panic!("My assumption was wrong");
        // }
        let dim = children[0].dimensions();

        if let Some(d_type) = children[0].data_type() {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }

        if let Some(dimension) = dim {
            node.set_dimensions(&dimension);
        }
    }
}

fn data_member(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // This is where we would have to search not just the current context but also
    // if this was a class the inherited contexts

    // This assuming that the dataMember is an assignable value and on the lhs of an equal sign
    // match context.get(id: &str)
    
    info!("data_member");

    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        // This is to make the borrow checker happy
        let child_data_clone = children[0].data().clone();
        let index_list_clone = children[1].clone();
        
        if let Some(d_type) = children[0].data_type() {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }

        if let Data::String(id) = child_data_clone {
            info!("Checking dimensionality of {}", id);
            match context.get(&id) {
                Some(SymbolTableEntry::Local(local)) => {
                    // Check to make sure dimensionalities agree
                    // This means the number of indexes must be the same
                    if let Some(dimensions) = index_list_clone.dimensions() {
                        info!("symb {}, ast {}", local.dimension().len(), dimensions);
                        if local.dimension().len() != dimensions {
                            let err = SemanticError::new_invalid_array_dimension(node.line(), node.column(), &dimensions, &local.dimension().len());
                            output.add(&err.to_string(), err.line(), err.col());
                        }
                    }
                    
                },
                Some(SymbolTableEntry::Param(param)) => {
                    if let Some(dimensions) = index_list_clone.dimensions() {
                        if param.dimension().len() != dimensions {
                            let err = SemanticError::new_invalid_array_dimension(node.line(), node.column(), &dimensions, &param.dimension().len());
                            output.add(&err.to_string(), err.line(), err.col());
                        }
                    }
                },
                Some(entry) => panic!("Id \"{}\" is naming something it shouldn't \"{}\"", id, entry), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
                None => {
                    let err = SemanticError::new_undefined_identifier(node.line(), node.column(), &id);
                    output.add(&err.to_string(), err.line(), err.col());
                    node.set_type("error-type");
                }
            }
        }

        // We must verify that the accompanying index list matches the dimensionality of the id
        
    }
}

fn add_op(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("add_op");

    let line = *node.line();
    let col = *node.column();
    
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        info!("Calling check bin from add_op");

        if let Ok(d_type) = check_binary_types(&children[0], &children[2], output, line, col) {
            node.set_type(&d_type);

            let new_name = context.get_next_temporary();
            let temp = Temporary::new(&new_name, &d_type, line, col);
            context.add_entry(SymbolTableEntry::Temporary(temp));
        } else {
            node.set_type("error-type");
        }


    }
}

fn mul_op(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("mul_op");

    
    let line = *node.line();
    let col = *node.column();
    
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        info!("Calling check bin from mul_op");
        if let Ok(d_type) = check_binary_types(&children[0], &children[2], output, line, col) {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }
    }
}


fn func_decl(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("func_decl");

}

fn intfactor(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("intfactor");
    let name = context.get_next_temporary();
    let value = if let Data::Integer(integer) = node.data() {
        *integer
    } else {
        -1337i64
    };

    let lit = Literal::new(&name, &LiteralValue::Integer(value.try_into().unwrap()), *node.line(), *node.column());
    context.add_entry(SymbolTableEntry::Literal(lit));
    node.set_type(INTEGER);
}

fn floatfactor(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("floatfactor");
    let name = context.get_next_temporary();
    let value = if let Data::Float(float) = node.data() {
        *float as f32
    } else {
        -1337f32
    };

    let lit = Literal::new(&name, &LiteralValue::Real(value), *node.line(), *node.column());
    context.add_entry(SymbolTableEntry::Literal(lit));

    node.set_type(FLOAT);
}

fn stringfactor(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("stringfactor");
    let name = context.get_next_temporary();
    let value = if let Data::String(string) = node.data() {
        string
    } else {
        "1337"
    };

    let lit = Literal::new(&name, &LiteralValue::StrLit(value.to_string()), *node.line(), *node.column());
    context.add_entry(SymbolTableEntry::Literal(lit));

    node.set_type(STRING);
}

fn type_node(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("type_node");

    
    if let Data::String(variable_type) = node.data() {
        match variable_type.as_str() {
            INTEGER | FLOAT | STRING => (), // OK its a primitive
            user_defined_type => {
                // signal an error if a class doesn't exist with the name
                if let None = global_table.get(user_defined_type) {
                    let err = SemanticError::new_undefined_type(node.line(), node.column(), user_defined_type);
                    output.add(&err.to_string(), err.line(), err.col());
                }
            } 
        }
    }
}

fn mandatory_dimlist(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // The list is in a mandatory context (a declaration or a datamember)
    // This means if it has any dimensions, they must be defined
    info!("mandatory_dimlist");


}

fn mandatory_indexlist(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // The list is in a mandatory context (a declaration or a datamember)
    // This means if it has any dimensions, they must be defined
    info!("mandatory_indexlist");

    // it should be a list of intfactors as children
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
        
        let mut dimensions = 0;
        // iterate again?
        for child in children.iter() {
            if let Some(data_type) = child.data_type() {
                if data_type != INTEGER {
                    let err = SemanticError::new_invalid_array_index(child.line(), child.column(), &data_type);
                    output.add(&err.to_string(), err.line(), err.col());
                } else {
                    // tally the dimensions used
                    dimensions += 1;
                }
            } else {
                // Missing required dimension
                let err = SemanticError::new_invalid_array_index(child.line(), child.column(), "void");
                output.add(&err.to_string(), err.line(), err.col());
            }
        }

        node.set_dimensions(&dimensions)
    } else {
        node.set_dimensions(&0);
    }   
}


fn id(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // Fetch the type from the context and set the node to the type
    // TODO: Note that this will need changing once classes are introduced


    // TODO: ID is also used for fCalls

    // Seems it would be best to break this out into multiple 
    info!("id");

    if let Data::String(id) = node.data() {
        match context.get(id) {
            Some(SymbolTableEntry::Local(local)) => {
                node.set_type(local.data_type());
                // TODO: Dimensions
            },
            Some(SymbolTableEntry::Param(parameter)) => {
                node.set_type(parameter.data_type());
                // TODO: Dimensions
            },
            Some(entry) => panic!("Id \"{}\" is colliding with something it shouldn't \"{}\"", id, entry), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
            None => {
                let err = SemanticError::new_undefined_identifier(node.line(), node.column(), id);
                output.add(&err.to_string(), err.line(), err.col());
                node.set_type("error-type");
            }
        }


    }
}

fn function_id(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    info!("function_id");

    // We must account for class members here as well

    if let Data::String(id) = node.data() {
        match global_table.get(id) {
            Some(SymbolTableEntry::Function(function)) => {
                // Set the current node to the return type of the function
                if let Some(ret_type) = function.return_type() {
                    node.set_type(ret_type);
                } else {
                    node.set_type("void"); // Should I just not set it?
                }
            },
            Some(entry) => panic!("Id \"{}\" is colliding with something it shouldn't \"{}\"", id, entry), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
            None => {
                let err = SemanticError::new_undefined_identifier(node.line(), node.column(), id);
                output.add(&err.to_string(), err.line(), err.col());
                node.set_type("error-type");
            }
        }
    }
}

fn index_list(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {

}

fn write_stat(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
    }
}

fn read_stat(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
    }
}

fn f_call(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    if let Data::Children(children) = node.data_mut() {
        // for child in children.iter_mut() {
        //     visit(child, context, state, global_table, output);
        // }

        // Because there's an ID we manually dispatch
        function_id(&mut children[0], context, state, global_table, output);

        let child_data_clone = children[0].data().clone();


        let function_id = if let Data::String(id) = child_data_clone {
            id
        } else {
            panic!();
        };
        
        // It may be advantageous to set the context to the actual function
        // 
        a_params_children(&mut children[1], context, state, global_table, output);

            // Given: The parameter node, the function id

        a_params_correct(&mut children[1], context, state, global_table, output, &function_id);
        // We could always visit the node twice
        // once to validate the children and once to validate the types of the function call
    }
}

fn a_params_children(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // a_params_children has the purpose of validating the substructures of the parameters
    // It treats dataMembers slightly differently in that they can be passed without indices as an array

    if let Data::Children(children) = node.data_mut() {
        // This sets up the data types of the children
        for child in children.iter_mut() {
            // Here we need to dispatch dataMembers exceptionally
            // Since this is the only context where an array can be partially or fully unspecified in their dimension
            match child.name().as_str() {
                "var" => parameter_var_exception(child, context, state, global_table, output),
                _ => visit(child, context, state, global_table, output),
            }

        }
    }
}


fn parameter_var_exception(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // This is an exceptional path that need to dispatch dataMembers to a special case
    // ONLY to be used for visiting nodes of a parameter list
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            match child.name().as_str() {
                "dataMember" => parameter_data_member_exception(child, context, state, global_table, output),
                _ => visit(child, context, state, global_table, output),
            }
        }

        if children.len() != 1 {
            panic!("Don't forget dot expressions");
            // in which case this node takes the type of the last determined child
        }

        let dim = children[0].dimensions();

        if let Some(d_type) = children[0].data_type() {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }

        if let Some(dimension) = dim {
            node.set_dimensions(&dimension);
        }
    }
}

fn parameter_data_member_exception(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig) {
    // This is exactly like a normal data except for:
    // Array indexing can be none to pass the array itself as a parameter
    info!("data_member");


    if let Data::Children(children) = node.data_mut() {
        // Do we actually want to perform these checks?
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        // println!("BADABING {:?}", children[0]);
        // println!("{:?}", children[1]);

        let child_data_clone = children[0].data().clone();
        let index_list_clone = children[1].clone();
        
        // No matter what we need to get the type of the ID node
        // if they're passing a normal number, no further action
        // If they're passing an un-indexed array, find the dimensionality from the symbol table
        // If they're passing an indexed array, 
        if let Some(d_type) = children[0].data_type() {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }

        if let Data::String(id) = child_data_clone {
            info!("Checking dimensionality of {}", id);
            match context.get(&id) {
                Some(SymbolTableEntry::Local(local)) => {
                    if let Some(dimensions) = index_list_clone.dimensions() {
                        // We have dimensions supplied
                        if dimensions == 0 {
                            node.set_dimensions(&local.dimension().len());
                        } else {
                            // info!("symb {}, ast {}", local.dimension().len(), dimensions);
                            if local.dimension().len() != dimensions {
                                let err = SemanticError::new_invalid_array_dimension(node.line(), node.column(), &dimensions, &local.dimension().len());
                                output.add(&err.to_string(), err.line(), err.col());
                            }    
                        }

                        // else: the symbol table and the index list agree. GOOD.
                    } else {
                        // We have no dimensions supplied
                        // We must get them from the local and assign them to this node
                        node.set_dimensions(&local.dimension().len());

                    }
                    
                },
                Some(SymbolTableEntry::Param(param)) => {
                    if let Some(dimensions) = index_list_clone.dimensions() {
                        if dimensions == 0 {
                            node.set_dimensions(&param.dimension().len());

                        } else {
                            // We have dimensions supplied 
                            if param.dimension().len() != dimensions {
                                // But they didn't provide the correct number
                                let err = SemanticError::new_invalid_array_dimension(node.line(), node.column(), &dimensions, &param.dimension().len());
                                output.add(&err.to_string(), err.line(), err.col());
                            } // else: They did provide the correct number!
                        }
                    } else {
                        node.set_dimensions(&param.dimension().len());

                    }
                },
                Some(entry) => panic!("Id \"{}\" is naming something it shouldn't \"{}\"", id, entry), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
                None => {
                    let err = SemanticError::new_undefined_identifier(node.line(), node.column(), &id);
                    output.add(&err.to_string(), err.line(), err.col());
                    node.set_type("error-type");
                }
            }
        }

        println!("GIVES: {:?}", node);

        // Here we have to check:
        // If the id identifies an array (which is known )

        // This is to make the borrow checker happy
        // let child_data_clone = children[0].data().clone();
        // let child_dimensions_clone = children[0].dimensions().clone();
        // let index_list_clone = children[1].clone();

        // let id = if let Data::String(id) = child_data_clone {
        //     id
        // } else {
        //     return;
        // };

        // if let Some(d_type) = children[0].data_type() {
        //     node.set_type(&d_type);
        // } else {
        //     node.set_type("error-type");
        // }

        // match context.get(&id) {
        //     Some(SymbolTableEntry::Local(local)) => {
        //         // Check to make sure dimensionalities agree
        //         // This means the number of indexes must be the same
        //         if let Some(dimensions) = index_list_clone.dimensions() {
        //             info!("symb {}, ast {}", local.dimension().len(), dimensions);
        //             if local.dimension().len() != dimensions {
        //                 // Dimensions are specified but they don't match

        //                 let err = SemanticError::new_invalid_array_dimension(node.line(), node.column(), &dimensions, &local.dimension().len());
        //                 output.add(&err.to_string(), err.line(), err.col());
        //                 node.set_type("error-type");
        //             }
        //         } else {
        //             // Dimensions are not specified
        //             // So they must be included in the dataMember type
        //             if let Some(dimensions) = child_dimensions_clone {
        //                 node.set_dimensions(&dimensions);
        //             } else {
        //                 node.set_type("error-type");
        //             }
        //         }
                
        //     },
        //     Some(SymbolTableEntry::Param(param)) => {
        //         if let Some(dimensions) = index_list_clone.dimensions() {
        //             if param.dimension().len() != dimensions {
        //                 let err = SemanticError::new_invalid_array_dimension(node.line(), node.column(), &dimensions, &param.dimension().len());
        //                 output.add(&err.to_string(), err.line(), err.col());
        //                 node.set_type("error-type");
        //             }
        //         } else {
        //             if let Some(dimensions) = child_dimensions_clone {
        //                 node.set_dimensions(&dimensions);
        //             } else {
        //                 node.set_type("error-type");
        //             }
        //         }
        //     },
        //     Some(entry) => panic!("Id \"{}\" is naming something it shouldn't \"{}\"", id, entry), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
        //     None => {
        //         let err = SemanticError::new_undefined_identifier(node.line(), node.column(), &id);
        //         output.add(&err.to_string(), err.line(), err.col());
        //         node.set_type("error-type");
        //     }
        // }
    }
}



fn a_params_correct(node: &mut Node, context: &mut SymbolTable, state: &mut State, global_table: &mut SymbolTable, output: &mut OutputConfig, function_id: &str) {
    // We must make sure
    // The number of children is equal to the number of parameters
    // The types of the children are equivalent to the parameters
    let line = *node.line();
    let column = *node.column();


    if let Data::Children(children) = node.data_mut() {
        // This sets up the data types of the children
        // for child in children.iter_mut() {
        //     visit(child, context, state, global_table, output);
        // }
        // The children have already been set up by a_params_children

    

        if let Some(SymbolTableEntry::Function(function)) = global_table.get(function_id) {
            // both the type and dimensionality of the parameter must be checked
            if function.parameter_types().len() != children.len() {
                let err = SemanticError::new_incorrect_number_arguments(line, column, children.len(), function.parameter_types().len());
                output.add(&err.to_string(), err.line(), err.col());
            }

            // now actually go through the children
            for (node, st_entry) in children.iter().zip(function.parameter_types().iter()) {
                println!("!!! {:?}, {}", node, st_entry);
                if node.data_type().clone().unwrap() != *st_entry.data_type() {                    
                    if st_entry.dimension().len() > 0 {
                        if let Some(dimension) = node.dimensions() {
                            if dimension != st_entry.dimension().len() {
                                let mut node_type = String::new();
                                node_type.push_str(&node.data_type().clone().unwrap());
                                for _ in 0..node.dimensions().clone().unwrap() {
                                    node_type.push_str("[]");
                                }
                                
                                let err = SemanticError::new_incorrect_type(line, column, &node_type, &st_entry.type_string());
                                output.add(&err.to_string(), err.line(), err.col());
            
                            }
                        }
                    }
                }
            }
        }
    }
}

fn check_binary_types(lhs: &Node, rhs: &Node, output: &mut OutputConfig, line: usize, col: usize) -> Result<String, ()> {
    info!("check_binary");

    
    let lht = if let Some(d_type) = lhs.data_type() {
        d_type
    } else {
        panic!();
    };

    let rht = if let Some(d_type) = rhs.data_type() {
        d_type
    } else {
        panic!();
    };

    if lht != rht {
        let err = SemanticError::new_binary_type_error(rhs.line(), rhs.column(), &lht, &rht);
        output.add(&err.to_string(), err.line(), err.col());
        info!("check_binary failed {}", err.to_string());

        Err(())
    } else {
        info!("check_binary ok");

        Ok(lht)
    }
}
