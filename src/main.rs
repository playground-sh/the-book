#[allow(unused)]
pub mod iii_common_concepts;
pub mod iv_ownership;

// use iii_common_concepts::i_variables_and_mutability;
// use iii_common_concepts::ii_data_types;
// use iii_common_concepts::iii_functions;
// use iii_common_concepts::v_control_flow;
use iv_ownership::i_what_is_ownership;

fn main() {
    // Chapter 3.1 - Variables and Mutability
    // i_variables_and_mutability::mutability_broken();
    // i_variables_and_mutability::mutability_fixed();
    // i_variables_and_mutability::constants();
    // i_variables_and_mutability::shadowing();
    // i_variables_and_mutability::more_shadowing();

    // Chapter 3.2 - Data Types
    // ii_data_types::int_types();
    // ii_data_types::int_literals();
    // ii_data_types::floating_point_types();
    // ii_data_types::boolean_types();
    // ii_data_types::character_type();
    // ii_data_types::tuple_type();
    // ii_data_types::array_type();

    // Chapter 3.3 - Functions
    // iii_functions::with_parameter(5);   // here `5` is the function argument
    // iii_functions::multiple_parameters(5, 'h');
    // iii_functions::statements_and_expression();
    // println!("{}", iii_functions::five());
    // println!("{}", iii_functions::plus_one(5));

    // Chapter 3.4 - Control Flow
    // v_control_flow::if_expressions::basic();
    // v_control_flow::if_expressions::else_if();
    // v_control_flow::if_expressions::using_if_in_a_let_statement();
    // v_control_flow::repetition_with_loops::returning_values_from_loops();
    // v_control_flow::repetition_with_loops::loop_labels();
    // v_control_flow::repetition_with_loops::conditional_loops_with_while();
    // v_control_flow::repetition_with_loops::looping_collections_with_for();
    // v_control_flow::repetition_with_loops::looping_collection_concisely();
    // v_control_flow::repetition_with_loops::countdown_with_for();

    // Chapter 4 - Understanding Ownership
    // i_what_is_ownership::variable_scope();
    // i_what_is_ownership::the_string_type();
    // i_what_is_ownership::memory_and_allocation::variables_and_data_interacting_with_move();
    // i_what_is_ownership::memory_and_allocation::string_with_move();
    // i_what_is_ownership::memory_and_allocation::scope_and_assignment();
    // i_what_is_ownership::memory_and_allocation::variables_and_data_interacting_with_clone();
    // i_what_is_ownership::ownership_and_copy::ownership_copy();
    // i_what_is_ownership::return_values_and_scope::scope();
    i_what_is_ownership::return_values_and_scope::return_multiple_values();
}
