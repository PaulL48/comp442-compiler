//! Find the sizes needed to reserve adequate memory for the program to run
//! Add literal and temporary values to the symbol table

use crate::SemanticAnalysisResults;
use crate::SemanticError;
use crate::{Function, Literal, LiteralValue, SymbolTable, SymbolTableEntry, Temporary};
use ast::{Data, Node};
use log::info;
use output_manager::OutputConfig;
use std::convert::TryInto;
use crate::mangling;

const INTEGER: &str = "integer";
const FLOAT: &str = "float";
const STRING: &str = "string";

pub fn process(
    node: &mut Node,
    current_results: &mut SemanticAnalysisResults,
    output: &mut OutputConfig,
) {
    // info!("Starting type check");
    visit(
        node,
        &mut current_results.symbol_table.clone(),
        &mut State {},
        &mut current_results.symbol_table,
        output,
    )
}

pub struct State {}

// Pass the global context around as a clone
// When a node arrives that mutates a single table, it must replace the

// Pass in a global table and a local table
// when an update is made to the local table
// copy the local table back into the global table

pub fn visit(
    node: &mut Node,
    current_context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // I don't think this needs to traverse the AST again
    // just iterate over the symbol table since it contains type information

    // FOR SETTING THE SIZE OF ALREADY EXISTENT VARIABLES:
    // just iterate over the symbol table

    // For the creation of temporary values in the symbol table
    // Visitor pattern

    match node.name().as_str() {
        "prog" => prog(node, current_context, state, global_table, output),
        "classDeclList" => class_list(node, current_context, state, global_table, output),
        "funcDefList" => function_list(node, current_context, state, global_table, output),

        "funcDecl" => func_decl(node, current_context, state, global_table, output),
        "funcBody" => func_body(node, current_context, state, global_table, output),
        "funcDef" => func_def(node, current_context, state, global_table, output),
        "fCall" => f_call(node, current_context, state, global_table, output),
        "varList" => var_list(node, current_context, state, global_table, output),

        "statBlock" => stat_block(node, current_context, state, global_table, output),
        "var" => var(node, current_context, state, global_table, output),
        "dataMember" => data_member(node, current_context, state, global_table, output),
        "intfactor" => intfactor(node, current_context, state, global_table, output),
        "floatfactor" => floatfactor(node, current_context, state, global_table, output),
        "stringfactor" => stringfactor(node, current_context, state, global_table, output),
        "type" => type_node(node, current_context, state, global_table, output),
        "varDecl" => var_decl(node, current_context, state, global_table, output),
        "id" => id(node, current_context, state, global_table, output),
        "indexList" => mandatory_indexlist(node, current_context, state, global_table, output),
        "aParams" => a_params_children(node, current_context, state, global_table, output),

        "returnStat" => return_stat(node, current_context, state, global_table, output),
        "ifStat" => if_stat(node, current_context, state, global_table, output),
        "whileStat" => while_stat(node, current_context, state, global_table, output),
        "writeStat" => write_stat(node, current_context, state, global_table, output),
        "readStat" => read_stat(node, current_context, state, global_table, output),

        "assignOp" => assign_op(node, current_context, state, global_table, output),
        "addOp" => add_op(node, current_context, state, global_table, output),
        "mulOp" => mul_op(node, current_context, state, global_table, output),
        "relOp" => rel_op(node, current_context, state, global_table, output),
        _ => {}
    }
}

fn prog(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // Here we'll explicitly invoke the individual children
    if let Data::Children(children) = node.data_mut() {
        visit(&mut children[0], context, state, global_table, output);
        visit(&mut children[1], context, state, global_table, output);

        if let Some(SymbolTableEntry::Function(main)) = context.get_mut("main") {
            entry_point(
                &mut children[2],
                main.symbol_table_mut(),
                state,
                global_table,
                output,
            );

            *global_table = context.clone();
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

fn entry_point(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
    }
}

fn class_list(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn function_list(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn var_list(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn func_body(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    let mut r_type = None;
    if let Data::Children(children) = node.data_mut() {
        for child in children {
            match child.name().as_str() {
                "statBlock" => {
                    visit(child, context, state, global_table, output);
                    r_type = child.data_type();
                } // get the type of the node and assign it to this type
                _ => visit(child, context, state, global_table, output),
            }
            // visit(child, context, state, global_table, output);
        }
    }
    if let Some(r_type) = r_type {
        node.set_type(&r_type)
    } else {
        // lets say you do return (void_func());
        //panic!(); // Return statement with no type? I think the t
    }
}

fn stat_block(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    let mut r_type = None;

    if let Data::Children(children) = node.data_mut() {
        for child in children {
            match child.name().as_str() {
                "returnStat" => {
                    visit(child, context, state, global_table, output);
                    r_type = child.data_type();
                }
                _ => visit(child, context, state, global_table, output),
            }
        }
    }

    if let Some(r_type) = r_type {
        node.set_type(&r_type)
    } else {
    }
}

fn assign_op(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    let line = *node.line();
    let col = *node.column();

    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        if let Ok(d_type) = check_binary_types(&children[0], &children[1], output, line, col) {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }
    }
}

fn var_decl(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
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

fn var(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
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
        println!("{:?}", children[0].name());

        if children[0].name() == "fCall" {
            println!("Setting type of var node based on fCall");
        }
        let d_type = children[0].data_type().clone();
        let label = children[0].label().clone();

        if let Some(d_type) = d_type {
            node.set_type(&d_type);
        } else {
            node.set_type("error-type");
        }

        if let Some(label) = label {
            node.set_label(&label);
        }

        if let Some(dimension) = dim {
            node.set_dimensions(&dimension);
        }
    }
}

fn data_member(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // This is where we would have to search not just the current context but also
    // if this was a class the inherited contexts

    // This assuming that the dataMember is an assignable value and on the lhs of an equal sign
    // match context.get(id: &str)

    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        // This is to make the borrow checker happy
        let child_data_clone = children[0].data().clone();
        let index_list_clone = children[1].clone();

        if let (Some(d_type), Some(label)) = (children[0].data_type(), children[0].label()) {
            node.set_type(&d_type);
            node.set_label(&label);
        } else {
            node.set_type("error-type");
        }

        if let Data::String(id) = child_data_clone {
            match context.get(&id) {
                Some(SymbolTableEntry::Local(local)) => {
                    // Check to make sure dimensionalities agree
                    // This means the number of indexes must be the same
                    if let Some(dimensions) = index_list_clone.dimensions() {
                        if local.dimension().len() != dimensions {
                            let err = SemanticError::new_invalid_array_dimension(
                                node.line(),
                                node.column(),
                                &dimensions,
                                &local.dimension().len(),
                            );
                            output.add(&err.to_string(), err.line(), err.col());
                        }
                    }
                }
                Some(SymbolTableEntry::Param(param)) => {
                    if let Some(dimensions) = index_list_clone.dimensions() {
                        if param.dimension().len() != dimensions {
                            let err = SemanticError::new_invalid_array_dimension(
                                node.line(),
                                node.column(),
                                &dimensions,
                                &param.dimension().len(),
                            );
                            output.add(&err.to_string(), err.line(), err.col());
                        }
                    }
                }
                Some(entry) => panic!(
                    "Id \"{}\" is naming something it shouldn't \"{}\"",
                    id, entry
                ), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
                None => {
                    let err =
                        SemanticError::new_undefined_identifier(node.line(), node.column(), &id);
                    output.add(&err.to_string(), err.line(), err.col());
                    node.set_type("error-type");
                }
            }
        }

        // We must verify that the accompanying index list matches the dimensionality of the id
    }
}

fn add_op(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    let line = *node.line();
    let col = *node.column();

    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        if let Ok(d_type) = check_binary_types(&children[0], &children[2], output, line, col) {
            node.set_type(&d_type);

            let new_name = context.get_next_temporary();
            let p = context.collect_parameters();
            let mangled_func = mangling::mangle_function(context.name(), &p, None);
            let mangled_name = mangling::mangle_id(&new_name, &mangled_func, None);
            let temp = Temporary::new(&mangled_name, &d_type, line, col);
            context.add_entry(SymbolTableEntry::Temporary(temp));
            node.set_label(&mangled_name);
        } else {
            node.set_type("error-type");
        }
    }
}

fn mul_op(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    let line = *node.line();
    let col = *node.column();

    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        if let Ok(d_type) = check_binary_types(&children[0], &children[2], output, line, col) {
            node.set_type(&d_type);

            let new_name = context.get_next_temporary();
            let p = context.collect_parameters();
            let mangled_func = mangling::mangle_function(context.name(), &p, None);
            let mangled_name = mangling::mangle_id(&new_name, &mangled_func, None);
            let temp = Temporary::new(&mangled_name, &d_type, line, col);
            context.add_entry(SymbolTableEntry::Temporary(temp));
            node.set_label(&mangled_name);
        } else {
            node.set_type("error-type");
        }
    }
}

fn rel_op(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    let line = *node.line();
    let col = *node.column();

    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }

        // Here we also must enforce that it is of int or real type
        if let Ok(d_type) = check_binary_types(&children[0], &children[2], output, line, col) {
            if !(d_type == INTEGER || d_type == FLOAT) {
                let err = SemanticError::new_invalid_relop(line, col, &d_type);
                output.add(&err.to_string(), err.line(), err.col());
                node.set_type("error-type");
                return;
            }

            node.set_type(&d_type);

            let new_name = context.get_next_temporary();
            let p = context.collect_parameters();
            let mangled_func = mangling::mangle_function(context.name(), &p, None);
            let mangled_name = mangling::mangle_id(&new_name, &mangled_func, None);
            let temp = Temporary::new(&mangled_name, &d_type, line, col);
            context.add_entry(SymbolTableEntry::Temporary(temp));
            node.set_label(&mangled_name);
        } else {
            node.set_type("error-type");
        }
    }
}

fn func_decl(
    _node: &mut Node,
    _context: &mut SymbolTable,
    _state: &mut State,
    _global_table: &mut SymbolTable,
    _output: &mut OutputConfig,
) {
}

fn intfactor(
    node: &mut Node,
    context: &mut SymbolTable,
    _state: &mut State,
    _global_table: &mut SymbolTable,
    _output: &mut OutputConfig,
) {
    let new_name = context.get_next_temporary();
    let p = context.collect_parameters();
    let mangled_func = mangling::mangle_function(context.name(), &p, None);
    let mangled_name = mangling::mangle_id(&new_name, &mangled_func, None);

    let value = if let Data::Integer(integer) = node.data() {
        *integer
    } else {
        -1337i64
    };

    let lit = Literal::new(
        &mangled_name,
        &LiteralValue::Integer(value.try_into().unwrap()),
        *node.line(),
        *node.column(),
    );
    context.add_entry(SymbolTableEntry::Literal(lit));
    node.set_type(INTEGER);
    node.set_label(&mangled_name);
}

fn floatfactor(
    node: &mut Node,
    context: &mut SymbolTable,
    _state: &mut State,
    _global_table: &mut SymbolTable,
    _output: &mut OutputConfig,
) {
    let new_name = context.get_next_temporary();
    let p = context.collect_parameters();
    let mangled_func = mangling::mangle_function(context.name(), &p, None);
    let mangled_name = mangling::mangle_id(&new_name, &mangled_func, None);


    let value = if let Data::Float(float) = node.data() {
        *float as f32
    } else {
        -1337f32
    };

    let lit = Literal::new(
        &mangled_name,
        &LiteralValue::Real(value),
        *node.line(),
        *node.column(),
    );
    context.add_entry(SymbolTableEntry::Literal(lit));

    node.set_type(FLOAT);
    node.set_label(&mangled_name);
}

fn stringfactor(
    node: &mut Node,
    context: &mut SymbolTable,
    _state: &mut State,
    _global_table: &mut SymbolTable,
    _output: &mut OutputConfig,
) {
    let new_name = context.get_next_temporary();
    let p = context.collect_parameters();
    let mangled_func = mangling::mangle_function(context.name(), &p, None);
    let mangled_name = mangling::mangle_id(&new_name, &mangled_func, None);
    let value = if let Data::String(string) = node.data() {
        string
    } else {
        "1337"
    };

    let lit = Literal::new(
        &mangled_name,
        &LiteralValue::StrLit(value.to_string()),
        *node.line(),
        *node.column(),
    );
    context.add_entry(SymbolTableEntry::Literal(lit));

    node.set_type(STRING);

    // TODO: Replace with name
    node.set_label(&mangled_name);
}

fn type_node(
    node: &mut Node,
    _context: &mut SymbolTable,
    _state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::String(variable_type) = node.data() {
        match variable_type.as_str() {
            INTEGER | FLOAT | STRING => (), // OK its a primitive
            user_defined_type => {
                // signal an error if a class doesn't exist with the name
                if let None = global_table.get(user_defined_type) {
                    let err = SemanticError::new_undefined_type(
                        node.line(),
                        node.column(),
                        user_defined_type,
                    );
                    output.add(&err.to_string(), err.line(), err.col());
                }
            }
        }
    }
}

fn mandatory_dimlist(
    _node: &mut Node,
    _context: &mut SymbolTable,
    _state: &mut State,
    _global_table: &mut SymbolTable,
    _output: &mut OutputConfig,
) {
    // The list is in a mandatory context (a declaration or a datamember)
    // This means if it has any dimensions, they must be defined
}

fn mandatory_indexlist(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // The list is in a mandatory context (a declaration or a datamember)
    // This means if it has any dimensions, they must be defined

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
                    let err = SemanticError::new_invalid_array_index(
                        child.line(),
                        child.column(),
                        &data_type,
                    );
                    output.add(&err.to_string(), err.line(), err.col());
                } else {
                    // tally the dimensions used
                    dimensions += 1;
                }
            } else {
                // Missing required dimension
                let err =
                    SemanticError::new_invalid_array_index(child.line(), child.column(), "void");
                output.add(&err.to_string(), err.line(), err.col());
            }
        }

        node.set_dimensions(&dimensions)
    } else {
        node.set_dimensions(&0);
    }
}

fn id(
    node: &mut Node,
    context: &mut SymbolTable,
    _state: &mut State,
    _global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // Fetch the type from the context and set the node to the type
    // TODO: Note that this will need changing once classes are introduced

    // TODO: ID is also used for fCalls

    // Seems it would be best to break this out into multiple

    if let Data::String(id) = node.data() {
        match context.get(id) {
            Some(SymbolTableEntry::Local(local)) => {
                node.set_type(local.data_type());
                let p = context.collect_parameters();
                let mangled_func = mangling::mangle_function(context.name(), &p, None);

                node.set_label(&mangling::mangle_id(local.id(), &mangled_func, None));
                // TODO: Dimensions
            }
            Some(SymbolTableEntry::Param(parameter)) => {
                node.set_type(parameter.data_type());
                let p = context.collect_parameters();
                let mangled_func = mangling::mangle_function(context.name(), &p, None);
                node.set_label(&mangling::mangle_id(parameter.id(), &mangled_func, None));

                // TODO: Dimensions
            }
            Some(entry) => panic!(
                "Id \"{}\" is colliding with something it shouldn't \"{}\"",
                id, entry
            ), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
            None => {
                let err = SemanticError::new_undefined_identifier(node.line(), node.column(), id);
                output.add(&err.to_string(), err.line(), err.col());
                node.set_type("error-type");
            }
        }
    }
}

fn function_id(
    node: &mut Node,
    _context: &mut SymbolTable,
    _state: &mut State,
    _global_table: &mut SymbolTable,
    _output: &mut OutputConfig,
    function: &Function,
) {
    if let Some(ret_type) = function.return_type() {
        node.set_type(ret_type);
    }
}

fn index_list(
    _node: &mut Node,
    _context: &mut SymbolTable,
    _state: &mut State,
    _global_table: &mut SymbolTable,
    _output: &mut OutputConfig,
) {
}

fn write_stat(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
    }
}

fn read_stat(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
    }
}

fn f_call(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    //let nc = node.clone();
    let line = *node.line();
    let col = *node.column();

    if let Data::Children(children) = node.data_mut() {
        // for child in children.iter_mut() {
        //     visit(child, context, state, global_table, output);
        // }

        let child_data_clone = children[0].data().clone();

        let function_id_str = if let Data::String(id) = child_data_clone {
            id
        } else {
            panic!();
        };

        // It may be advantageous to set the context to the actual function
        //
        a_params_children(&mut children[1], context, state, global_table, output);

        let parameters = if let Data::Children(parameters) = children[1].data() {
            parameters.clone()
        } else {
            Vec::new()
        };

        // Now that the parameters have had their types determined
        // We should select the correct overload based on them and supply that to both
        // following functions
        let f = select_free_overload(&function_id_str, &parameters, global_table);
        match f {
            Ok(matching_function) => {
                // println!("Selected overload for {:?} is {:?}", nc, matching_function);

                function_id(
                    &mut children[0],
                    context,
                    state,
                    global_table,
                    output,
                    &matching_function,
                );
                a_params_correct(
                    &mut children[1],
                    context,
                    state,
                    global_table,
                    output,
                    &matching_function,
                );

    

                if let Some(d_type) = matching_function.return_type() {
                    println!("Setting type of fCall node to: {:?}", d_type);
                    node.set_type(d_type);
                    let new_name = context.get_next_temporary();
                    let p = context.collect_parameters();
                    let mangled_func = mangling::mangle_function(context.name(), &p, None);
                    let mangled_name = mangling::mangle_id(&new_name, &mangled_func, None);
                    let temp = Temporary::new(&mangled_name, &d_type, line, col);
                    context.add_entry(SymbolTableEntry::Temporary(temp));
                    node.set_label(&mangled_name);
                        }
            }
            Err(Some(_)) => {
                let parameter_str = parameters
                    .iter()
                    .map(|n| n.data_type().unwrap())
                    .collect::<Vec<_>>()
                    .join(", ");

                let err = SemanticError::new_no_overload(
                    *node.line(),
                    *node.column(),
                    &function_id_str,
                    &parameter_str,
                );
                output.add(&err.to_string(), err.line(), err.col());
            } // cannot find overload
            Err(None) => {
                let err = SemanticError::new_undefined_identifier(
                    node.line(),
                    node.column(),
                    &function_id_str,
                );
                output.add(&err.to_string(), err.line(), err.col());
            } // Undefined identifier
        }

        // if let Some(f) = f {
        //     println!("Selected overload for {:?} is {:?}", nc, f);

        //     function_id(&mut children[0], context, state, global_table, output, &f);
        //     a_params_correct(&mut children[1], context, state, global_table, output, &f);
        //     if let Some(d_type) = f.return_type() {
        //         node.set_type(d_type);
        //     }
        // } else {

        // }

        // Because there's an ID we manually dispatch
        // we also need to supply the parameters to select the correct overload

        // let d_type = children[0].data_type().clone().unwrap();

        // Given: The parameter node, the function id

        // We could always visit the node twice
        // once to validate the children and once to validate the types of the function call

        // node.set_type(&d_type);
    }
}

fn a_params_children(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // a_params_children has the purpose of validating the substructures of the parameters
    // It treats dataMembers slightly differently in that they can be passed without indices as an array

    if let Data::Children(children) = node.data_mut() {
        // This sets up the data types of the children
        for child in children.iter_mut() {
            // Here we need to dispatch dataMembers exceptionally
            // Since this is the only context where an array can be partially or fully unspecified in their dimension
            match child.name().as_str() {
                "var" => {
                    println!("Visiting p");
                    parameter_var_exception(child, context, state, global_table, output);
                },
                _ => {
                    println!("Visiting non p");
                    visit(child, context, state, global_table, output);
                }
            }
        }
    }
}

fn parameter_var_exception(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // This is an exceptional path that need to dispatch dataMembers to a special case
    // ONLY to be used for visiting nodes of a parameter list
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            match child.name().as_str() {
                "dataMember" => {
                    parameter_data_member_exception(child, context, state, global_table, output)
                }
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

fn parameter_data_member_exception(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // This is exactly like a normal data except for:
    // Array indexing can be none to pass the array itself as a parameter

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
        if let (Some(d_type), Some(label)) = (children[0].data_type(), children[0].label()) {
            node.set_type(&d_type);
            node.set_label(&label);
        } else {
            node.set_type("error-type");
        }

        if let Data::String(id) = child_data_clone {
            match context.get(&id) {
                Some(SymbolTableEntry::Local(local)) => {
                    if let Some(dimensions) = index_list_clone.dimensions() {
                        // We have dimensions supplied
                        if dimensions == 0 {
                            node.set_dimensions(&local.dimension().len());
                        } else {
                            if local.dimension().len() != dimensions {
                                let err = SemanticError::new_invalid_array_dimension(
                                    node.line(),
                                    node.column(),
                                    &dimensions,
                                    &local.dimension().len(),
                                );
                                output.add(&err.to_string(), err.line(), err.col());
                            }
                        }

                        // else: the symbol table and the index list agree. GOOD.
                    } else {
                        // We have no dimensions supplied
                        // We must get them from the local and assign them to this node
                        node.set_dimensions(&local.dimension().len());
                    }
                }
                Some(SymbolTableEntry::Param(param)) => {
                    if let Some(dimensions) = index_list_clone.dimensions() {
                        if dimensions == 0 {
                            node.set_dimensions(&param.dimension().len());
                        } else {
                            // We have dimensions supplied
                            if param.dimension().len() != dimensions {
                                // But they didn't provide the correct number
                                let err = SemanticError::new_invalid_array_dimension(
                                    node.line(),
                                    node.column(),
                                    &dimensions,
                                    &param.dimension().len(),
                                );
                                output.add(&err.to_string(), err.line(), err.col());
                            } // else: They did provide the correct number!
                        }
                    } else {
                        node.set_dimensions(&param.dimension().len());
                    }
                }
                Some(entry) => panic!(
                    "Id \"{}\" is naming something it shouldn't \"{}\"",
                    id, entry
                ), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
                None => {
                    let err =
                        SemanticError::new_undefined_identifier(node.line(), node.column(), &id);
                    output.add(&err.to_string(), err.line(), err.col());
                    node.set_type("error-type");
                }
            }
        }
    }
}

fn a_params_correct(
    node: &mut Node,
    _context: &mut SymbolTable,
    _state: &mut State,
    _global_table: &mut SymbolTable,
    output: &mut OutputConfig,
    function: &Function,
) {
    // We must make sure
    // The number of children is equal to the number of parameters
    // The types of the children are equivalent to the parameters
    let line = *node.line();
    let column = *node.column();

    // println!("PARAM CORRECT {:?}", node);
    if let Data::Children(children) = node.data_mut() {
        // both the type and dimensionality of the parameter must be checked
        if function.parameter_types().len() != children.len() {
            let err = SemanticError::new_incorrect_number_arguments(
                line,
                column,
                children.len(),
                function.parameter_types().len(),
            );
            output.add(&err.to_string(), err.line(), err.col());
        }

        // now actually go through the children
        for (node, st_entry) in children.iter().zip(function.parameter_types().iter()) {
            // println!("!!! {:?}, {}", node, st_entry);
            if let Some(dimension) = node.dimensions() {
                if dimension != st_entry.dimension().len() {
                    let mut node_type = String::new();
                    node_type.push_str(&node.data_type().clone().unwrap());
                    for _ in 0..node.dimensions().clone().unwrap() {
                        node_type.push_str("[]");
                    }

                    let err = SemanticError::new_incorrect_type(
                        line,
                        column,
                        &node_type,
                        &st_entry.type_string(),
                    );
                    output.add(&err.to_string(), err.line(), err.col());
                }
            } else if st_entry.dimension().len() != 0 {
                let mut node_type = String::new();
                node_type.push_str(&node.data_type().clone().unwrap());
                let err = SemanticError::new_incorrect_type(
                    line,
                    column,
                    &node_type,
                    &st_entry.type_string(),
                );
                output.add(&err.to_string(), err.line(), err.col());
            } else if node.data_type().clone().unwrap() != *st_entry.data_type() {
                // At this point we know we have the correct base type but we need to check
                // whether the dimension of the node is correct
                if let Some(dimension) = node.dimensions() {
                    if dimension != st_entry.dimension().len() {
                        let mut node_type = String::new();
                        node_type.push_str(&node.data_type().clone().unwrap());
                        for _ in 0..node.dimensions().clone().unwrap() {
                            node_type.push_str("[]");
                        }

                        let err = SemanticError::new_incorrect_type(
                            line,
                            column,
                            &node_type,
                            &st_entry.type_string(),
                        );
                        output.add(&err.to_string(), err.line(), err.col());
                    }
                } else if st_entry.dimension().len() != 0 {
                    let mut node_type = String::new();
                    node_type.push_str(&node.data_type().clone().unwrap());
                    let err = SemanticError::new_incorrect_type(
                        line,
                        column,
                        &node_type,
                        &st_entry.type_string(),
                    );
                    output.add(&err.to_string(), err.line(), err.col());
                }
            }
        }
    }
}

use crate::ast_validation::{FunctionDefinition, ViewAs};

fn func_def(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // make sure to change the context and copy it back out once done
    let line = *node.line();
    let column = *node.column();

    // let parameter_list = {
    let _validated_node = FunctionDefinition::view_as(node);

    let validated_node = if let Ok(validated_node) = FunctionDefinition::view_as(node) {
        validated_node
    } else {
        // bad
        panic!();
    };

    let mut parameter_list = Vec::new();
    for parameter in validated_node.parameter_list().parameters() {
        let mut n = Node::new("", Data::Epsilon, 0, 0);
        n.set_type(parameter.data_type());
        parameter_list.push(n);
    }

    let function_id_str = validated_node.id().to_owned();
    let mut return_type = None;

    if let Data::Children(children) = node.data_mut() {
        // I think all of the checking has already been done in the symbol table assembly
        // so we just need to get the right context by supplying

        let t1 = context.clone();

        match select_free_overload_mut(&function_id_str, &parameter_list, context) {
            Ok(matching_function) => {
                // Add a return to temporary
                let p = t1.collect_parameters();
                let mangled_func = mangling::mangle_function(matching_function.symbol_table().name(), &p, None);
                let mangled_name = mangling::function_return_to(&mangled_func);



                for child in children.iter_mut() {
                    match child.name().as_str() {
                        "id" => (),
                        "funcBody" => {
                            // This will set the return type of the function
                            visit(
                                child,
                                matching_function.symbol_table_mut(),
                                state,
                                global_table,
                                output,
                            );
                            return_type = child.data_type();
                        }
                        _ => visit(
                            child,
                            matching_function.symbol_table_mut(),
                            state,
                            global_table,
                            output,
                        ),
                    }
                    // We don't actually need to visit the id of the function
                    // since all the verification was done for it already

                    
                }

                // let temp = Temporary::new(&mangled_name, "integer", line, column);
                // matching_function.symbol_table_mut().add_entry(SymbolTableEntry::Temporary(temp));

                if return_type != *matching_function.return_type() {
                    let err = SemanticError::new_incorrect_type(
                        *node.line(),
                        *node.column(),
                        &return_type.clone().unwrap_or("void".to_owned()),
                        &matching_function
                            .return_type()
                            .clone()
                            .unwrap_or("void".to_owned()),
                    );
                    output.add(&err.to_string(), err.line(), err.col());
                }

                *global_table = context.clone();
            }
            Err(Some(_)) => {
                let parameter_str = parameter_list
                    .iter()
                    .map(|n| n.data_type().unwrap())
                    .collect::<Vec<_>>()
                    .join(", ");

                let err =
                    SemanticError::new_no_overload(line, column, &function_id_str, &parameter_str);
                output.add(&err.to_string(), err.line(), err.col());
            } // cannot find overload
            Err(None) => {
                let err = SemanticError::new_undefined_identifier(&line, &column, &function_id_str);
                output.add(&err.to_string(), err.line(), err.col());
            } // Undefined identifier
        }
    }
}

fn return_stat(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    let mut r_type = None;
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
            r_type = child.data_type();
        }
    }
    if let Some(r_type) = r_type {
        node.set_type(&r_type);
    }
}

fn if_stat(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
    }
}

fn while_stat(
    node: &mut Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data_mut() {
        for child in children.iter_mut() {
            visit(child, context, state, global_table, output);
        }
    }
}

fn check_binary_types(
    lhs: &Node,
    rhs: &Node,
    output: &mut OutputConfig,
    _line: usize,
    _col: usize,
) -> Result<String, ()> {
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

        Err(())
    } else {
        Ok(lht)
    }
}

fn select_free_overload(
    function_id: &str,
    parameters: &[Node],
    global_table: &SymbolTable,
) -> Result<Function, Option<()>> {
    // println!(
    //     "Trying to find correct overload of {}{:?}",
    //     function_id, parameters
    // );

    let matches = global_table.get_all(&function_id);
    if matches.len() == 0 {
        return Err(None);
    }

    for matching_entry in matches {
        match matching_entry {
            SymbolTableEntry::Function(function) => {
                //println!("Checking candidate {:?}", function);

                // we must be sure the data type of the params are the same as for the function
                if function.parameter_types().len() != parameters.len() {
                    continue; // bad candidate, length mismatch
                }

                let mut parameter_failure = false;

                for (param_node, st_entry) in parameters.iter().zip(function.parameter_types()) {
                    //println!("Checking parameter n:{:?} st:{:?}", param_node, st_entry);

                    // The specifically ignores the array dimensionality
                    if param_node.data_type().unwrap() != *st_entry.data_type() {
                        //println!("Skipping candidate because type mismatch");
                        parameter_failure = true;
                    }
                }

                if !parameter_failure {
                    //println!("Candidate confirmed");
                    return Ok(function.clone());
                }
            }
            entry => panic!(
                "Id \"{}\" is colliding with something it shouldn't \"{}\"",
                function_id, entry
            ), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
        }
    }
    Err(Some(()))
}

fn select_free_overload_mut<'a>(
    function_id: &str,
    parameters: &[Node],
    global_table: &'a mut SymbolTable,
) -> Result<&'a mut Function, Option<()>> {
    // println!(
    //     "Trying to find correct overload of {}{:?}",
    //     function_id, parameters
    // );

    let matches = global_table.get_all_mut(&function_id);
    if matches.len() == 0 {
        return Err(None);
    }

    for matching_entry in matches {
        match matching_entry {
            SymbolTableEntry::Function(function) => {
                //println!("Checking candidate {:?}", function);

                // we must be sure the data type of the params are the same as for the function
                if function.parameter_types().len() != parameters.len() {
                    continue; // bad candidate, length mismatch
                }

                let mut parameter_failure = false;

                for (param_node, st_entry) in parameters.iter().zip(function.parameter_types()) {
                    //println!("Checking parameter n:{:?} st:{:?}", param_node, st_entry);

                    // The specifically ignores the array dimensionality
                    if param_node.data_type().unwrap() != *st_entry.data_type() {
                        //println!("Skipping candidate because type mismatch");
                        parameter_failure = true;
                    }
                }

                if !parameter_failure {
                    //println!("Candidate confirmed");
                    return Ok(function);
                }
            }
            entry => panic!(
                "Id \"{}\" is colliding with something it shouldn't \"{}\"",
                function_id, entry
            ), // Bad, but this shouldn't happen (likely culprit is collision with temporary)
        }
    }
    Err(Some(()))
}
