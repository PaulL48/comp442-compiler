use crate::macros as mm; // for moon-macros
use crate::moon_instructions as moon;
use crate::preamble;
use crate::register::{Register, RegisterPool, R0, R13, R14, R15};
use ast::{Data, Node};
use log::info;
use output_manager::OutputConfig;
use semantic_analyzer::SemanticAnalysisResults;
use semantic_analyzer::{Literal, LiteralValue, SymbolTable, SymbolTableEntry};
use semantic_analyzer::mangling;

const OUTPUT_BUFFER_SIZE: usize = 20;

pub struct State {
    registers: RegisterPool,
}

pub fn process(
    node: &Node,
    current_results: &mut SemanticAnalysisResults,
    output: &mut OutputConfig,
) {
    // Add the contents of the lib file
    output_manager::warn_write(
        &mut output.code_file,
        &mut output.code_path,
        preamble::PREAMBLE,
    );

    // This is taken from the slides
    // Add the buffer for output
    output.add_data(&moon::cmt_line(" Buffer space used for console output"));
    mm::res(OUTPUT_BUFFER_SIZE, "buf", output);

    reserve_space(&current_results.symbol_table, output);
    visit(
        node,
        &mut current_results.symbol_table.clone(),
        &mut State {
            registers: RegisterPool::new(),
        },
        &mut current_results.symbol_table,
        output,
    )
}

fn reserve_space(table: &SymbolTable, output: &mut OutputConfig) {
    // Iterate over the functions and create labeled reserve statements for the various functions
    for element in &table.values {
        match element {
            SymbolTableEntry::Function(function) => {
                output.add_data(&moon::cmt_line(&format!(
                    "Reserved memory for function {}",
                    function.id()
                )));
                reserve_space(function.symbol_table(), output);
            }
            SymbolTableEntry::Local(local) => {
                mm::res(*local.bytes(), &mangling::mangle_id(local.id(), table.name(), None), output)
            }
            SymbolTableEntry::Param(param) => {
                mm::res(*param.bytes(), &mangling::mangle_id(param.id(), table.name(), None), output)
            }
            SymbolTableEntry::Literal(literal) => {
                let result = match literal.value() {
                    LiteralValue::Integer(int) => int.to_string(),
                    LiteralValue::Real(f) => i32::from_ne_bytes(f.to_ne_bytes()).to_string(),
                    LiteralValue::StrLit(s) => panic!(), // don't know what to do here
                };
                let t = vec![result.as_str()];
                output.add_data(&moon::labeled_line(
                    &mangling::mangle_id(table.name(), literal.id(), None),
                    &moon::mem_store_w(t.as_slice()),
                ));
            }
            SymbolTableEntry::Temporary(temporary) => {
                mm::res(*temporary.bytes(), &mangling::mangle_id(temporary.id(), table.name(), None), output)
            }
            _ => (),
        }
    }
}

fn visit(
    node: &Node,
    current_context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    match node.name().as_str() {
        "prog" => prog(node, current_context, state, global_table, output),
        // "classDeclList" => class_list(node, current_context, state, global_table, output),
        // "funcDefList" => function_list(node, current_context, state, global_table, output),

        // "funcDecl" => func_decl(node, current_context, state, global_table, output),
        "funcBody" => visit_children(node, current_context, state, global_table, output),
        "funcDef" => func_def(node, current_context, state, global_table, output),
        // "fCall" => f_call(node, current_context, state, global_table, output),
        "varList" => var_list(node, current_context, state, global_table, output),

        "statBlock" => stat_block(node, current_context, state, global_table, output),
        // "var" => var(node, current_context, state, global_table, output),
        // "dataMember" => data_member(node, current_context, state, global_table, output),
        // "intfactor" => intfactor(node, current_context, state, global_table, output),
        // "floatfactor" => floatfactor(node, current_context, state, global_table, output),
        // "stringfactor" => stringfactor(node, current_context, state, global_table, output),
        // "type" => type_node(node, current_context, state, global_table, output),
        "varDecl" => var_decl(node, current_context, state, global_table, output),
        // "id" => id(node, current_context, state, global_table, output),
        // "indexList" => mandatory_indexlist(node, current_context, state, global_table, output),
        // "aParams" => a_params_children(node, current_context, state, global_table, output),

        // "returnStat" => return_stat(node, current_context, state, global_table, output),
        "ifStat" => if_stat(node, current_context, state, global_table, output),
        "whileStat" => while_stat(node, current_context, state, global_table, output),
        "writeStat" => write_stat(node, current_context, state, global_table, output),
        // "readStat" => read_stat(node, current_context, state, global_table, output),
        "assignOp" => assign_op(node, current_context, state, global_table, output),
        "addOp" => add_op(node, current_context, state, global_table, output),
        "mulOp" => mul_op(node, current_context, state, global_table, output),
        "relOp" => rel_op(node, current_context, state, global_table, output),
        _ => {}
    }
}

fn prog(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // Here we'll explicitly invoke the individual children
    if let Data::Children(children) = node.data() {
        // class_list(node, context, state, global_table, output);
        function_list(&children[1], context, state, global_table, output);
        // entry_point(&children[2], context, state, global_table, output);
        if let Some(SymbolTableEntry::Function(main)) = context.get_mut("main") {
            entry_point(
                &children[2],
                main.symbol_table_mut(),
                state,
                global_table,
                output,
            );

            *global_table = context.clone();
        }
    } else {
        panic!();
    }

    // if let Data::Children(children) = node.data() {
    //     for child in children {
    //         visit(child, current_results);
    //     }
    // }
}

fn visit_children(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn class_list(
    node: &Node,
    context: &SymbolTable,
    state: &mut State,
    global_table: &SymbolTable,
    output: &mut OutputConfig,
) {
}

fn function_list(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn func_def(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        let id = if let Data::String(s) = children[0].data() {
            s
        } else {
            panic!();
        };
        let parameters = collect_parameters(&children[2]);

        // Switch the context 
        let mut t1 = context.clone();
        let mut t2 = global_table.clone();
        if let Some(overload) = global_table.get_function(id, &parameters) {
            output.add_exec(&moon::cmt_line(&format!("Defining function {}", id)));
            let mangled_name = mangling::mangle_function(id, &parameters, None);
            output.add_exec(&moon::labeled_line(&mangling::function_return(&mangled_name, None), &moon::res("4")));
            for (i, _) in parameters.iter().enumerate() {
                output.add_exec(&moon::labeled_line(&mangling::function_parameter(&mangled_name, i, None), &moon::res("4")));
            }

            output.add_exec(&moon::labeled_line(&mangled_name, &moon::noop()));

            for child in children {
                visit(child, &mut t1, state, &mut t2, output);
            }

            // generate exit label
            output.add_exec(&moon::labeled_line(&mangling::function_exit(&mangled_name, None), &moon::noop()));

            // We're going to need some jump back to the call site
            mm::ret(output);
        } else {
            panic!();
        }
        *context = t1;
        *global_table = t2;
    }

    // The following code is assuming this a free function

    // global_table.get_function(id: &str, parameters: &[&str])

    // End assumption

    // This could be a free function definition or a class member definition
    // To know the

    // Select the correct overload
    // This may be a class metho

    // Setup memory block for return statement

    // Setup the memory block for the parameters
    // integer and floats are passed direct into the registers
    // string and classes (compound types) are passed as addresses

    // Setup memory block for the code
}

fn entry_point(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    output.add_exec(&moon::cmt_line("Begin main ==================================================================================================="));
    output.add_exec(&moon::instr_line(&moon::entry()));
    output.add_exec(&moon::instr_line(&moon::add_i(&R14, &R0, "topaddr")));

    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }

    output.add_exec(&moon::instr_line(&moon::halt()));
    mm::cmt_exec("==================================================================================================================", output);
    mm::cmt_exec("   END OF PROGRAM/BEGINNING OF DATA", output);
    mm::cmt_exec("==================================================================================================================", output);
}

fn var_list(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn var_decl(
    node: &Node,
    context: &SymbolTable,
    state: &mut State,
    global_table: &SymbolTable,
    output: &mut OutputConfig,
) {
    // Create the reserved space? That should already be done
}

fn stat_block(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }
}

fn assign_op(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }

    let r = state.registers.reserve(1);
    let local_register = state.registers.pop();
    // let dst_offset_reg = state.registers.pop();
    // let src_offset_reg = state.registers.pop();

    // Zero the offsets
    // mm::zero(&dst_offset_reg, output);
    // mm::zero(&src_offset_reg, output);

    //

    let dst = get_child_label(node, 0);
    // let dst_offset = get_index(node: &Node)
    let src = get_child_label(node, 1);
    // let src_offset

    // Here the added offset to the dst is what will affect arrays
    mm::cmt_exec(
        &format!("Processing assign op to \"{}\" from \"{}\"", src, dst),
        output,
    );

    output.add_exec(&moon::instr_line(&moon::load_w(&local_register, &src, &R0)));
    output.add_exec(&moon::instr_line(&moon::store_w(
        &dst,
        &R0,
        &local_register,
    )));

    state.registers.release(r);
}

fn mul_op(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }

    let r = state.registers.reserve(3);
    let local_register = state.registers.pop();
    let lhs = state.registers.pop();
    let rhs = state.registers.pop();

    let lhs_label = get_child_label(node, 0);
    let rhs_label = get_child_label(node, 2);
    let dst = get_label(node);
    let op = get_child_name(node, 1);

    mm::cmt_exec(
        &format!(
            "Processing mul op {} <- {} {} {} ",
            dst, lhs_label, op, rhs_label
        ),
        output,
    );

    output.add_exec(&moon::instr_line(&moon::load_w(&lhs, &lhs_label, &R0)));
    output.add_exec(&moon::instr_line(&moon::load_w(&rhs, &rhs_label, &R0)));

    if op == "*" {
        output.add_exec(&moon::instr_line(&moon::mul(&local_register, &lhs, &rhs)));
    } else if op == "/" {
        output.add_exec(&moon::instr_line(&moon::div(&local_register, &lhs, &rhs)));
    } else if op == "and" {
        panic!();
    } else {
        panic!();
    }

    output.add_exec(&moon::instr_line(&moon::store_w(
        &dst,
        &R0,
        &local_register,
    )));

    state.registers.release(r);
}

fn add_op(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }

    let r = state.registers.reserve(3);
    let local_register = state.registers.pop();
    let lhs = state.registers.pop();
    let rhs = state.registers.pop();

    let lhs_label = get_child_label(node, 0);
    let rhs_label = get_child_label(node, 2);
    let dst = get_label(node);
    let op = get_child_name(node, 1);

    mm::cmt_exec(
        &format!(
            "Processing add op {} <- {} {} {} ",
            dst, lhs_label, op, rhs_label
        ),
        output,
    );

    output.add_exec(&moon::instr_line(&moon::load_w(&lhs, &lhs_label, &R0)));
    output.add_exec(&moon::instr_line(&moon::load_w(&rhs, &rhs_label, &R0)));

    if op == "+" {
        output.add_exec(&moon::instr_line(&moon::add(&local_register, &lhs, &rhs)));
    } else if op == "-" {
        output.add_exec(&moon::instr_line(&moon::sub(&local_register, &lhs, &rhs)));
    } else if op == "or" {
        panic!();
    } else {
        panic!();
    }

    output.add_exec(&moon::instr_line(&moon::store_w(
        &dst,
        &R0,
        &local_register,
    )));

    state.registers.release(r);
}

fn rel_op(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }

    let r = state.registers.reserve(3);
    let local_register = state.registers.pop();
    let lhs = state.registers.pop();
    let rhs = state.registers.pop();

    let lhs_label = get_child_label(node, 0);
    let rhs_label = get_child_label(node, 2);
    let dst = get_label(node);
    let op = get_child_name(node, 1);

    mm::cmt_exec(
        &format!(
            "Processing rel op {} <- {} {} {}",
            dst, lhs_label, op, rhs_label
        ),
        output,
    );

    output.add_exec(&moon::instr_line(&moon::load_w(&lhs, &lhs_label, &R0)));
    output.add_exec(&moon::instr_line(&moon::load_w(&rhs, &rhs_label, &R0)));

    if op == "lt" {
        output.add_exec(&moon::instr_line(&moon::cmp_lt(
            &local_register,
            &lhs,
            &rhs,
        )));
    } else if op == "lte" {
        output.add_exec(&moon::instr_line(&moon::cmp_lte(
            &local_register,
            &lhs,
            &rhs,
        )));
    } else if op == "gt" {
        output.add_exec(&moon::instr_line(&moon::cmp_gt(
            &local_register,
            &lhs,
            &rhs,
        )));
    } else if op == "gte" {
        output.add_exec(&moon::instr_line(&moon::cmp_gte(
            &local_register,
            &lhs,
            &rhs,
        )));
    } else if op == "eq" {
        output.add_exec(&moon::instr_line(&moon::cmp_eq(
            &local_register,
            &lhs,
            &rhs,
        )));
    } else {
        panic!();
    }

    output.add_exec(&moon::instr_line(&moon::store_w(
        &dst,
        &R0,
        &local_register,
    )));

    state.registers.release(r);
}

fn if_stat(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        let (else_label, endif_label) = context.get_next_if_else_label();
        mm::cmt_exec(
            &format!("If statement ({}, {})", else_label, endif_label),
            output,
        );

        // Loads the relOp label with the value
        visit(&children[0], context, state, global_table, output);

        let rel_op = get_child_label(node, 0);

        let r = state.registers.reserve(1);
        let cmp_res = state.registers.pop();

        output.add_exec(&moon::instr_line(&moon::load_w(&cmp_res, &rel_op, &R0)));
        output.add_exec(&moon::instr_line(&moon::jmp_zero(&cmp_res, &else_label)));

        mm::cmt_exec(
            &format!("If statement ({}, {}) TRUE block", else_label, endif_label),
            output,
        );

        // True block
        visit(&children[1], context, state, global_table, output);
        output.add_exec(&moon::instr_line(&moon::jmp(&endif_label)));

        mm::cmt_exec(
            &format!("If statement ({}, {}) FALSE block", else_label, endif_label),
            output,
        );

        // False block
        output.add_exec(&moon::labeled_line(&else_label, &moon::noop()));
        visit(&children[2], context, state, global_table, output);

        // End if
        output.add_exec(&moon::labeled_line(&endif_label, &moon::noop()));

        state.registers.release(r);
    }
}

fn while_stat(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    // The labels to jump to must be unique
    // They could use some incrementing value plus mangling
    // so main__else1, main__endif1, main__else2, main__endif2
    // The most logical place to store that counter would be the symbol table
    // perform comparison
    if let Data::Children(children) = node.data() {
        let rel_op = get_child_label(node, 0);
        let (go_while, end_while) = context.get_next_while_label();
        mm::cmt_exec(
            &format!("While statement  ({}, {})", go_while, end_while),
            output,
        );

        let r = state.registers.reserve(1);
        let cmp_res = state.registers.pop();

        output.add_exec(&moon::labeled_line(&go_while, &moon::noop()));

        mm::cmt_exec(
            &format!(
                "While statement ({}, {}), Evaluation of conditional",
                go_while, end_while
            ),
            output,
        );

        visit(&children[0], context, state, global_table, output);
        output.add_exec(&moon::instr_line(&moon::load_w(&cmp_res, &rel_op, &R0)));
        output.add_exec(&moon::instr_line(&moon::jmp_zero(&cmp_res, &end_while)));

        mm::cmt_exec(
            &&format!(
                "While statement ({}, {}), Statement block",
                go_while, end_while
            ),
            output,
        );

        // True block
        visit(&children[1], context, state, global_table, output);
        output.add_exec(&moon::instr_line(&moon::jmp(&go_while)));
        output.add_exec(&moon::labeled_line(&end_while, &moon::noop()));

        state.registers.release(r);
    }
}

fn write_stat(
    node: &Node,
    context: &mut SymbolTable,
    state: &mut State,
    global_table: &mut SymbolTable,
    output: &mut OutputConfig,
) {
    if let Data::Children(children) = node.data() {
        for child in children {
            visit(child, context, state, global_table, output);
        }
    }

    let r = state.registers.reserve(1);
    let local = state.registers.pop();
    let src = get_child_label(node, 0);

    // Taken from slides
    output.add_exec(&moon::cmt_line("Processing write statement"));
    output.add_exec(&moon::instr_line(&moon::load_w(&local, &src, &R0)));
    output.add_exec(&moon::cmt_line("put value on stack"));
    output.add_exec(&moon::instr_line(&moon::store_w("-8", &R14, &local)));
    output.add_exec(&moon::cmt_line("link buffer to stack"));
    output.add_exec(&moon::instr_line(&moon::add_i(&local, &R0, "buf")));
    output.add_exec(&moon::instr_line(&moon::store_w("-12", &R14, &local)));
    output.add_exec(&moon::cmt_line("convert int to string for output"));
    output.add_exec(&moon::instr_line(&moon::jmp_lnk(&R15, "intstr")));
    output.add_exec(&moon::instr_line(&moon::store_w("-8", &R14, &R13)));
    output.add_exec(&moon::cmt_line("output to console"));
    output.add_exec(&moon::instr_line(&moon::jmp_lnk(&R15, "putstr")));

    state.registers.release(r);
}

/// Return the aggregated list of non-array parameter type
/// from the supplied fparamList node
fn collect_parameters(node: &Node) -> Vec<String> {
    let mut result = Vec::new();
    if let Data::Children(children) = node.data() {
        for child in children {
            if let Data::Children(sub_children) = child.data() {
                if let Data::String(d_type) = sub_children[0].data() {
                    result.push(d_type.clone());
                }
            }
        }
    }
    result
}

// fn entry_point(node: &Node, context: &SymbolTable, output: &mut OutputConfig) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, output);
//         }
//     }
// }

// fn var_list(node: &Node, context: &SymbolTable, output: &mut OutputConfig) {
//     if let Data::Children(children) = node.data() {
//         for child in children {
//             visit(child, context, output);
//         }
//     }
// }

// fn var_decl(_node: &Node, _context: &SymbolTable, _output: &mut OutputConfig) {
//     // one challenge here, we need to create the entry but we also
//     // need the name of the enclosing function to prefix the label
// }

fn get_child_label(node: &Node, index: usize) -> String {
    if let Data::Children(children) = node.data() {
        if let Some(label) = children[index].label() {
            return label;
        } else {
            panic!();
        }
    } else {
        panic!();
    };
}

/// Traverse the tree of nodes looking for the return statement
fn get_return_statement() {}

// This accepts a dataMember
// fn get_index(node: &Node, child: usize) -> usize {
//     if let Data::Children(children) = node.data() { // This would be the assignOp
//         if let Data::Children(children) = children[child].data() {
//             if let Data::Children(indx_list_children) = children[0].data() {
//                 if let Data::Integer(offset) = indx_list_children[0].data() {
//                     return *offset as usize;
//                 }
//             }
//         }

//     }

//     return 0;
// }

fn get_child_name(node: &Node, index: usize) -> String {
    if let Data::Children(children) = node.data() {
        return children[index].name().clone();
    } else {
        panic!();
    };
}

fn get_label(node: &Node) -> String {
    return node.label().clone().unwrap();
}
