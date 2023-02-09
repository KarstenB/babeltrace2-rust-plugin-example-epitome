use std::mem::MaybeUninit;

use babeltrace2_plugin::bt2::binding::{
    bt_component_class_initialize_method_status, bt_component_class_sink_consume_method_status,
    bt_component_class_sink_graph_is_configured_method_status, bt_message, bt_message_iterator,
    bt_self_component_sink, bt_self_component_sink_configuration, bt_value,
};
use babeltrace2_plugin::bt2::{
    BtComponentClassInitializeMethodStatus, BtComponentClassSinkConsumeMethodStatus,
    BtComponentClassSinkGraphIsConfiguredMethodStatus, BtMessageIterator,
    BtMessageIteratorNextStatus, BtSelfComponentSink, BtMessage, BtMessageConst,
};

struct EpitomeOut {
    /* Upstream message iterator (owned by this) */
    iterator: BtMessageIterator,

    /* Current event message index */
    index: u64,
}

#[no_mangle]
pub unsafe extern "C" fn epitome_out_consume(
    self_component_sink: *mut bt_self_component_sink,
) -> bt_component_class_sink_consume_method_status {
    println!("Called epitome_out_consume");

    //bt_component_class_sink_consume_method_status status =
    //    BT_COMPONENT_CLASS_SINK_CONSUME_METHOD_STATUS_OK;

    /* Retrieve our private data from the component's user data */
    let mut sink = BtSelfComponentSink::from_ptr(self_component_sink);
    let mut epitome_out =
        Box::from_raw(sink.as_self_component_inline().get_data() as *mut EpitomeOut);

    /* Consume a batch of messages from the upstream message iterator */
    let mut messages:MaybeUninit<*mut *const bt_message>  = MaybeUninit::uninit();
    let mut count: u64 = 0;
    let next_status= epitome_out.iterator.next(messages.as_mut_ptr(), &mut count);

    match next_status {
        BtMessageIteratorNextStatus::End => {
            epitome_out.iterator.put_ref();
            Box::leak(epitome_out);
            return BtComponentClassSinkConsumeMethodStatus::End.into();
        }
        BtMessageIteratorNextStatus::Again => {
            Box::leak(epitome_out);
            return BtComponentClassSinkConsumeMethodStatus::Again.into();
        }
        BtMessageIteratorNextStatus::MemoryError => {
            Box::leak(epitome_out);
            return BtComponentClassSinkConsumeMethodStatus::MemoryError.into();
        }
        BtMessageIteratorNextStatus::Error => {
            Box::leak(epitome_out);
            return BtComponentClassSinkConsumeMethodStatus::Error.into();
        }
        _ => {}
    }
    let msg_out:*mut *const bt_message =*messages.as_mut_ptr();
    println!("Count {count} {:p}", msg_out);
    let msgs: &[*const bt_message]= std::slice::from_raw_parts(msg_out, count as usize);
    /* For each consumed message */
    for msg in msgs {
        
        let btmsg = BtMessageConst::from_ptr(*msg);
        println!("msg {:?}", btmsg.get_type());
    }

    Box::leak(epitome_out);
    BtComponentClassSinkConsumeMethodStatus::Ok.into()
}

#[no_mangle]
pub unsafe extern "C" fn epitome_out_graph_is_configured(
    self_component_sink: *mut bt_self_component_sink,
) -> bt_component_class_sink_graph_is_configured_method_status {
    println!("Called epitome_out_graph_is_configured");
    /* Retrieve our private data from the component's user data */
    let mut sink = BtSelfComponentSink::from_ptr(self_component_sink);
    let mut data = Box::from_raw(sink.as_self_component_inline().get_data() as *mut EpitomeOut);

    /* Borrow our unique port */
    let in_port = sink.borrow_input_port_by_index(0);

    /* Create the upstream message iterator */
    let mut iter: MaybeUninit<*mut bt_message_iterator> = MaybeUninit::uninit();
    BtMessageIterator::create_from_sink_component(&sink, &in_port, iter.as_mut_ptr());
    data.iterator = BtMessageIterator::from_ptr(*iter.as_mut_ptr());
    println!("Data iterator {:p} {:p}", data, &data.iterator);

    Box::leak(data);
    BtComponentClassSinkGraphIsConfiguredMethodStatus::Ok.into()
}

#[no_mangle]
pub unsafe extern "C" fn epitome_out_finalize(self_component_sink: *mut bt_self_component_sink) {
    println!("Called epitome_out_finalize");
    /* Retrieve our private data from the component's user data */
    let sink = BtSelfComponentSink::from_ptr(self_component_sink).as_self_component_inline();
    let data = Box::from_raw(sink.get_data() as *mut EpitomeOut);
    println!("epitome_out_finalize, data index {}", data.index);
    /* Free the allocated structure */
    //_data runs out of scope and is de-allocated automatically
}

#[no_mangle]
pub unsafe extern "C" fn epitome_out_initialize(
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
    sink.add_input_port("in", std::ptr::null_mut(), std::ptr::null_mut());

    /* Set the component's user data to our private data structure */
    let mut comp = sink.as_self_component_inline();
    comp.set_data(Box::into_raw(epitome_out) as *mut std::ffi::c_void);
    BtComponentClassInitializeMethodStatus::Ok.into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn ptr() {}
}
