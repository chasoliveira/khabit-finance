use crate::finance::domain::events::domain_event_source::DomainEventSource;

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
