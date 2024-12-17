use std::any::Any;
use std::fmt::Debug;
use libcoap_sys::{
    coap_new_client_session_oscore,
    coap_oscore_conf_t
};
use crate::CoapContext;
use crate::mem::CoapLendableFfiRcCell;
use crate::session::CoapClientSession;
use crate::types::{CoapAddress, CoapProtocol, OscoreConf};

pub fn new_client_session_oscore<D: Any+?Sized+Debug>(context: &mut CoapContext, mut local_if: CoapAddress, mut server: CoapAddress, proto: CoapProtocol, mut config: OscoreConf) {

    unsafe {
        coap_new_client_session_oscore(
            context.as_mut_raw_context(),
            local_if.as_mut_raw_address(),
            server.as_mut_raw_address(),
            proto.as_raw_protocol(),
            config.as_mut_raw_conf(),
        );
    };
}