mod bindings;

use crate::bindings::exports::golem_example::billing_exports::golem_example_billing_api::Guest;
// Import for using common lib:
// use common_lib::example_common_function;

use self::bindings::exports::golem_example::billing_exports::golem_example_billing_api::GuestBillingEntity;

struct Entity {
    id: u64
}

impl GuestBillingEntity for Entity {
    fn new(id: u64) -> Self {
        Self { id }
    }

    fn completion_promise(&self) -> bindings::exports::golem_example::billing_exports::golem_example_billing_api::PromiseId {
        let promise_id = bindings::golem::api::host::create_promise();
        bindings::golem::api::host::complete_promise(&promise_id, &vec![]);
        promise_id
    }
}

struct Component;

impl Guest for Component {
    type BillingEntity = Entity;
}

bindings::export!(Component with_types_in bindings);
