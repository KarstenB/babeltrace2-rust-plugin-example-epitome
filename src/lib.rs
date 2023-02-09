// Copyright 2023 - 2023, Karsten Becker and the rust babeltrace2-plugin contributors
// SPDX-License-Identifier: GPL-2.0-or-later
use babeltrace2_plugin::bt2::binding::{
    bt_component_class_initialize_method_status, bt_component_class_sink_consume_method_status,
    bt_component_class_sink_graph_is_configured_method_status,
    bt_self_component_sink, bt_self_component_sink_configuration, bt_value,
};
use babeltrace2_plugin::bt2::{
    BtComponentClassInitializeMethodStatus, BtComponentClassSinkConsumeMethodStatus,
    BtComponentClassSinkGraphIsConfiguredMethodStatus, BtMessageConst,
    BtMessageIterator, BtMessageType, BtSelfComponentSink, BtMessageIteratorNextStatus,
};
use babeltrace2_plugin::{drop_data, get_iterator, get_scoped_boxed_data, iterator_to_vec, set_boxed_data};

struct EpitomeOut {
    /* Upstream message iterator (owned by this) */
    iterator: BtMessageIterator,
    /* Current event message index */
    index: u64,
}

fn print_event(message: &BtMessageConst, epitome_out: &mut EpitomeOut) {
    /* Discard if it's not an event message */
    if message.get_type() != BtMessageType::Event {
        return;
    }
    /* Borrow the event message's event and its class */
    let event = message.event_borrow_event_const();
    let event_class = event.borrow_class_const();

    /* Get the number of payload field members */
    let payload_field = event.borrow_payload_field_const();
    let member_count = payload_field
        .borrow_class_const()
        .structure_get_member_count();

    /* Write a corresponding line to the standard output */
    println!(
        "#{}: {} ({} payload member{})",
        epitome_out.index,
        event_class.get_name().to_string_lossy(),
        member_count,
        if member_count == 1 { "" } else { "s" }
    );

    /* Increment the current event message's index */
    epitome_out.index += 1;
}

#[no_mangle]
pub extern "C" fn epitome_out_consume(
    self_component_sink: *mut bt_self_component_sink,
) -> bt_component_class_sink_consume_method_status {
    println!("Called epitome_out_consume");

    /* Retrieve our private data from the component's user data */
    let mut sink = BtSelfComponentSink::from_ptr(self_component_sink);
    get_scoped_boxed_data(&mut sink, |epitome_out: &mut Box<EpitomeOut>| {
        /* Consume a batch of messages from the upstream message iterator */
        let res = iterator_to_vec(&mut epitome_out.iterator);
        match res {
            Ok(vec) => {
                for msg in vec {
                    print_event(&msg, epitome_out);
                    msg.put_ref();
                }
                BtComponentClassSinkConsumeMethodStatus::Ok.into()
            },
            Err(err) => {
                if err==BtMessageIteratorNextStatus::End {
                    epitome_out.iterator.put_ref();
                }
                return BtComponentClassSinkConsumeMethodStatus::from(err).into();
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn epitome_out_graph_is_configured(
    self_component_sink: *mut bt_self_component_sink,
) -> bt_component_class_sink_graph_is_configured_method_status {
    println!("Called epitome_out_graph_is_configured");
    /* Retrieve our private data from the component's user data */
    let mut sink = BtSelfComponentSink::from_ptr(self_component_sink);
    get_scoped_boxed_data(&mut sink, |epitome_out: &mut Box<EpitomeOut>| {
        /* Borrow our unique port */
        let mut sink = BtSelfComponentSink::from_ptr(self_component_sink);
        let in_port = sink.borrow_input_port_by_index(0);

        /* Create the upstream message iterator */
        epitome_out.iterator = get_iterator(&sink, &in_port);

        BtComponentClassSinkGraphIsConfiguredMethodStatus::Ok.into()
    })
}

#[no_mangle]
pub extern "C" fn epitome_out_finalize(self_component_sink: *mut bt_self_component_sink) {
    println!("Called epitome_out_finalize");
    /* Retrieve our private data from the component's user data */
    let mut sink = BtSelfComponentSink::from_ptr(self_component_sink);
    drop_data(&mut sink);
    /* Free the allocated structure */
    //_data runs out of scope and is de-allocated automatically
}

#[no_mangle]
pub extern "C" fn epitome_out_initialize(
    self_component_sink: *mut bt_self_component_sink,
    _configuration: *mut bt_self_component_sink_configuration,
    _params: *const bt_value,
    _initialize_method_data: *mut ::std::os::raw::c_void,
) -> bt_component_class_initialize_method_status {
    println!("Called epitome_out_initialize");
    /* Allocate a private data structure */
    let mut epitome_out = Box::new(EpitomeOut {
        iterator: BtMessageIterator::empty(),
        index: 0,
    });
    /* Initialize the first event message's index */
    epitome_out.index = 1;

    /*
     * Add an input port named `in` to the sink component.
     *
     * This is needed so that this sink component can be connected to a
     * filter or a source component. With a connected upstream
     * component, this sink component can create a message iterator
     * to consume messages.
     */
    let mut sink = BtSelfComponentSink::from_ptr(self_component_sink);
    unsafe {sink.add_input_port("in", std::ptr::null_mut(), std::ptr::null_mut())};

    /* Set the component's user data to our private data structure */
    set_boxed_data(&mut sink, epitome_out);
    BtComponentClassInitializeMethodStatus::Ok.into()
}
