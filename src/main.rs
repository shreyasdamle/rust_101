mod s1_hello_world;
mod s2_vars;
mod s3_data_types;
mod s4_functions;
mod s5_control_flow;
mod s6_ownership;
mod s7_references_borrowing;

fn main() {
    //S1: Print 'Hello, world!`
    s1_hello_world::run();

    //S2: Variables
    s2_vars::run();

    //S3: Data Types
    s3_data_types::run();

    //S4: Functions
    s4_functions::run();

    //S5: Control Flows
    s5_control_flow::run();

    //S6: Ownership
    s6_ownership::run();

    //S7: References and Borrowing
    s7_references_borrowing::run();
}
