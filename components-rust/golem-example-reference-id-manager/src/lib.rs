mod bindings;

use crate::bindings::exports::golem_example::reference_id_manager_exports::golem_example_reference_id_manager_api::Guest;
// Import for using common lib:
// use common_lib::example_common_function;
use std::cell::RefCell;

/// This is one of any number of data types that our application
/// uses. Golem will take care to persist all application state,
/// whether that state is local to a function being executed or
/// global across the entire program.
struct State {
    current_counter: u64,
}

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<State> = RefCell::new(State {
        current_counter: 0,
    });
}

struct Component;

impl Guest for Component {
    /// Updates the component's state by adding the given value to the total.
    fn new_reference_id() -> u64 {
        STATE.with_borrow_mut(|state| {
            let current_value = state.current_counter;
            state.current_counter += 1;
            current_value
        })
    }
}

bindings::export!(Component with_types_in bindings);
