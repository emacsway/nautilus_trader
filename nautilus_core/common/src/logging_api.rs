// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2023 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

use std::ffi::c_char;

use nautilus_core::string::{cstr_to_string, str_to_cstr};
use nautilus_model::identifiers::trader_id::TraderId;
use ustr::Ustr;

use crate::{enums::LogLevel, logging::Logger};

/// Creates a new logger.
///
/// # Safety
///
/// - Assumes `trader_id_ptr` is a valid C string pointer.
#[no_mangle]
pub unsafe extern "C" fn logger_new(
    trader_id_ptr: *const c_char,
    component_ptr: *const c_char,
    is_bypassed: u8,
) -> Logger {
    Logger::new(
        TraderId::new(&cstr_to_string(trader_id_ptr)),
        Ustr::from(&cstr_to_string(component_ptr)),
        is_bypassed != 0,
    )
}

#[no_mangle]
pub extern "C" fn logger_get_trader_id_cstr(logger: &Logger) -> *const c_char {
    str_to_cstr(&logger.trader_id.to_string())
}

#[no_mangle]
pub extern "C" fn logger_is_bypassed(logger: &Logger) -> u8 {
    logger.is_bypassed as u8
}

/// Create a new log event.
///
/// # Safety
///
/// - Assumes `component_ptr` is a valid C string pointer.
/// - Assumes `message_ptr` is a valid C string pointer.
#[no_mangle]
pub unsafe extern "C" fn logger_log(
    logger: &mut Logger,
    _timestamp_ns: u64,
    level: LogLevel,
    message_ptr: *const c_char,
) {
    let message = cstr_to_string(message_ptr);
    match level {
        LogLevel::Debug => logger.debug(message),
        LogLevel::Info => logger.info(message),
        LogLevel::Warning => logger.warn(message),
        LogLevel::Error => logger.error(message),
        _ => {} // TODO: remove critical level logging
    }
}
