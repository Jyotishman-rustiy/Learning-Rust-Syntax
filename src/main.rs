mod harkirat;

use  harkirat::module_01_traits_example;
use harkirat::impl_traits;
use harkirat::module_02_declarative_macros;

use harkirat::module_02_procedural_macros;



mod module_1_data_types_examples;
mod module_2_control_flow_examplex;
mod module_3_functions_closure_examples;
mod module_04_data_structures_examples;
mod module_05_ownership_example;
mod module_06_struct_examples;
mod module_07_enum_example;
mod module_08_options_examples;


fn main() {
    module_1_data_types_examples::demo();
    module_2_control_flow_examplex::demo();
    module_3_functions_closure_examples::demo();
    module_04_data_structures_examples::demo();
    module_05_ownership_example::demo();
    module_06_struct_examples::demo();
    module_07_enum_example::demo();
    module_08_options_examples::demo();

    module_01_traits_example::demo();
    impl_traits::demo();
    module_02_declarative_macros::demo();
    module_02_procedural_macros::demo();
}
