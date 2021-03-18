// Citadel C bindings library (libcitadel)
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.

use libc::c_char;

use crate::citadel_client_t;
use crate::TryIntoString;

#[no_mangle]
pub extern "C" fn release_string(s: *mut c_char) {
    s.try_into_string();
}

#[no_mangle]
pub extern "C" fn citadel_is_ok(client: *mut citadel_client_t) -> bool {
    citadel_client_t::from_raw(client).is_ok()
}

#[no_mangle]
pub extern "C" fn citadel_has_err(client: *mut citadel_client_t) -> bool {
    citadel_client_t::from_raw(client).has_err()
}
