use crate::{
    AccountType,
    finance::domain::{
        account::Account, budgets::budget::Budget, events::domain_event_source::DomainEventSource,
        shared::currency::Currency,
    },
};

pub fn collect_event_names(source: &dyn DomainEventSource) -> Vec<String> {
    let mut names = Vec::new();

    source.visit_event_sources(&mut |event_source| {
        for event in event_source.domain_events() {
            names.push(event.event_name().to_string());
        }
    });

    names
}

pub fn clear_all_events(source: &mut dyn DomainEventSource) {
    source.visit_event_sources_mut(&mut |event_source| {
        event_source.clear_domain_events();
    });
}

pub fn new_budget_brl() -> Budget {
    Budget::new("My Wallet".to_string(), Currency::brl())
}

pub fn new_account_brl() -> Account {
    Account::new("Generic".to_string(), AccountType::Cash, Currency::brl())
}
pub fn new_account(currency: Currency) -> Account {
    Account::new("Generic".to_string(), AccountType::Cash, currency)
}
