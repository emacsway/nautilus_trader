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

use std::str::FromStr;

use nautilus_model::identifiers::trader_id::TraderId;
use pyo3::prelude::*;
use tracing::{debug, error, info, warn, Level};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{fmt::Layer, prelude::*, EnvFilter, Registry};

/// Guards the log collector and flushes it when dropped
///
/// This struct must be dropped when the application has completed operation
/// it ensures that the any pending log lines are flushed before the application
/// closes.
#[pyclass]
pub struct LogGuard {
    #[allow(dead_code)]
    guards: Vec<WorkerGuard>,
}

/// Sets the global log collector
///
/// stdout_level: Set the level for the stdout writer
/// stderr_level: Set the level for the stderr writer
/// file_level: Set the level, the directory and the prefix for the file writer
///
/// It also configures a top level filter based on module/component name.
/// The format for the string is component1=info,component2=debug.
/// For e.g. network=error,kernel=info
///
/// # Safety
/// Should only be called once during an applications run, ideally at the
/// beginning of the run.
#[pyfunction]
pub fn set_global_log_collector(
    stdout_level: Option<String>,
    stderr_level: Option<String>,
    file_level: Option<(String, String, String)>,
) -> LogGuard {
    let mut guards = Vec::new();
    let stdout_sub_builder = stdout_level.map(|stdout_level| {
        let stdout_level = Level::from_str(&stdout_level).unwrap();
        let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
        guards.push(guard);
        Layer::default().with_writer(non_blocking.with_max_level(stdout_level))
    });
    let stderr_sub_builder = stderr_level.map(|stderr_level| {
        let stderr_level = Level::from_str(&stderr_level).unwrap();
        let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
        guards.push(guard);
        Layer::default().with_writer(non_blocking.with_max_level(stderr_level))
    });
    let file_sub_builder = file_level.map(|(dir_path, file_prefix, file_level)| {
        let file_level = Level::from_str(&file_level).unwrap();
        let rolling_log = RollingFileAppender::new(Rotation::NEVER, dir_path, file_prefix);
        let (non_blocking, guard) = tracing_appender::non_blocking(rolling_log);
        guards.push(guard);
        Layer::default().with_writer(non_blocking.with_max_level(file_level))
    });

    Registry::default()
        .with(stderr_sub_builder)
        .with(stdout_sub_builder)
        .with(file_sub_builder)
        .with(EnvFilter::from_default_env())
        .init();

    LogGuard { guards }
}

#[repr(C)]
pub struct Logger {
    pub trader_id: TraderId,
    pub is_bypassed: bool,
}

impl Logger {
    pub fn new(trader_id: TraderId, is_bypassed: bool) -> Self {
        Logger {
            trader_id,
            is_bypassed,
        }
    }

    pub fn debug(&self, message: &str, component: &str) {
        debug!(
            message,
            component = component,
            id = self.trader_id.value.as_str()
        );
    }

    pub fn info(&self, message: &str, component: &str) {
        info!(
            message,
            component = component,
            id = self.trader_id.value.as_str()
        );
    }

    pub fn warn(&self, message: &str, component: &str) {
        warn!(
            message,
            component = component,
            id = self.trader_id.value.as_str()
        );
    }

    pub fn error(&self, message: &str, component: &str) {
        error!(
            message,
            component = component,
            id = self.trader_id.value.as_str()
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
// #[cfg(test)]
// mod tests {
//     use std::time::Duration;

//     use nautilus_core::uuid::UUID4;
//     use nautilus_model::identifiers::trader_id::TraderId;
//     use tempfile::tempdir;

//     use super::*;
//     use crate::testing::wait_until;

//     fn create_logger() -> Logger {
//         Logger::new(
//             TraderId::new("TRADER-001"),
//             String::from("user-01"),
//             UUID4::new(),
//             LogLevel::Info,
//             None,
//             None,
//             None,
//             None,
//             None,
//             false,
//         )
//     }

//     #[test]
//     fn log_message_serialization() {
//         let log_message = LogEvent {
//             timestamp: 1_000_000_000,
//             level: LogLevel::Info,
//             color: LogColor::Normal,
//             component: "Portfolio".to_string(),
//             message: "This is a log message".to_string(),
//         };

//         let serialized_json = serde_json::to_string(&log_message).unwrap();
//         let deserialized_value: Value = serde_json::from_str(&serialized_json).unwrap();

//         assert_eq!(deserialized_value["timestamp"], 1_000_000_000);
//         assert_eq!(deserialized_value["level"], "INFO");
//         assert_eq!(deserialized_value["component"], "Portfolio");
//         assert_eq!(deserialized_value["message"], "This is a log message");
//     }

//     #[test]
//     fn test_new_logger() {
//         let logger = create_logger();

//         assert_eq!(logger.trader_id, TraderId::new("TRADER-001"));
//         assert_eq!(logger.level_stdout, LogLevel::Info);
//         assert_eq!(logger.level_file, None);
//         assert!(!logger.is_bypassed);
//     }

//     #[test]
//     fn test_logger_debug() {
//         let mut logger = create_logger();

//         logger.debug(
//             1_650_000_000_000_000,
//             LogColor::Normal,
//             String::from("RiskEngine"),
//             String::from("This is a test debug message."),
//         );
//     }

//     #[test]
//     fn test_logger_info() {
//         let mut logger = create_logger();

//         logger.info(
//             1_650_000_000_000_000,
//             LogColor::Normal,
//             String::from("RiskEngine"),
//             String::from("This is a test info message."),
//         );
//     }

//     #[test]
//     fn test_logger_error() {
//         let mut logger = create_logger();

//         logger.error(
//             1_650_000_000_000_000,
//             LogColor::Normal,
//             String::from("RiskEngine"),
//             String::from("This is a test error message."),
//         );
//     }

//     #[test]
//     fn test_logger_critical() {
//         let mut logger = create_logger();

//         logger.critical(
//             1_650_000_000_000_000,
//             LogColor::Normal,
//             String::from("RiskEngine"),
//             String::from("This is a test critical message."),
//         );
//     }

//     #[test]
//     fn test_logging_to_file() {
//         let temp_dir = tempdir().expect("Failed to create temporary directory");

//         let mut logger = Logger::new(
//             TraderId::new("TRADER-001"),
//             String::from("user-01"),
//             UUID4::new(),
//             LogLevel::Info,
//             Some(LogLevel::Debug),
//             Some(temp_dir.path().to_str().unwrap().to_string()),
//             None,
//             None,
//             None,
//             false,
//         );

//         logger.info(
//             1_650_000_000_000_000,
//             LogColor::Normal,
//             String::from("RiskEngine"),
//             String::from("This is a test."),
//         );

//         let mut log_contents = String::new();

//         wait_until(
//             || {
//                 let log_file_exists = std::fs::read_dir(&temp_dir)
//                     .expect("Failed to read directory")
//                     .filter_map(Result::ok)
//                     .filter(|entry| entry.path().is_file())
//                     .next()
//                     .is_some();

//                 log_file_exists
//             },
//             Duration::from_secs(2),
//         );

//         wait_until(
//             || {
//                 let log_file_path = std::fs::read_dir(&temp_dir)
//                     .expect("Failed to read directory")
//                     .filter_map(Result::ok)
//                     .filter(|entry| entry.path().is_file())
//                     .next()
//                     .expect("No files found in directory")
//                     .path();
//                 log_contents =
//                     std::fs::read_to_string(&log_file_path).expect("Error while reading log file");
//                 !log_contents.is_empty()
//             },
//             Duration::from_secs(2),
//         );

//         assert_eq!(
//             log_contents,
//             "1970-01-20T02:20:00.000000000Z [INF] TRADER-001.RiskEngine: This is a test.\n"
//         );
//     }

//     #[test]
//     fn test_log_component_level_filtering() {
//         let temp_dir = tempdir().expect("Failed to create temporary directory");

//         let mut logger = Logger::new(
//             TraderId::new("TRADER-001"),
//             String::from("user-01"),
//             UUID4::new(),
//             LogLevel::Info,
//             Some(LogLevel::Debug),
//             Some(temp_dir.path().to_str().unwrap().to_string()),
//             None,
//             None,
//             Some(HashMap::from_iter(std::iter::once((
//                 String::from("RiskEngine"),
//                 Value::from("ERROR"), // <-- This should be filtered
//             )))),
//             false,
//         );

//         logger.info(
//             1_650_000_000_000_000,
//             LogColor::Normal,
//             String::from("RiskEngine"),
//             String::from("This is a test."),
//         );

//         wait_until(
//             || {
//                 if let Some(log_file) = std::fs::read_dir(&temp_dir)
//                     .expect("Failed to read directory")
//                     .filter_map(Result::ok)
//                     .filter(|entry| entry.path().is_file())
//                     .next()
//                 {
//                     let log_file_path = log_file.path();
//                     let log_contents = std::fs::read_to_string(&log_file_path)
//                         .expect("Error while reading log file");
//                     !log_contents.contains("RiskEngine")
//                 } else {
//                     false
//                 }
//             },
//             Duration::from_secs(3),
//         );

//         assert!(
//             std::fs::read_dir(&temp_dir)
//                 .expect("Failed to read directory")
//                 .filter_map(Result::ok)
//                 .filter(|entry| entry.path().is_file())
//                 .next()
//                 .is_some(),
//             "Log file exists"
//         );
//     }

//     #[test]
//     fn test_logging_to_file_in_json_format() {
//         let temp_dir = tempdir().expect("Failed to create temporary directory");

//         let mut logger = Logger::new(
//             TraderId::new("TRADER-001"),
//             String::from("user-01"),
//             UUID4::new(),
//             LogLevel::Info,
//             Some(LogLevel::Debug),
//             Some(temp_dir.path().to_str().unwrap().to_string()),
//             None,
//             Some("json".to_string()),
//             None,
//             false,
//         );

//         logger.info(
//             1_650_000_000_000_000,
//             LogColor::Normal,
//             String::from("RiskEngine"),
//             String::from("This is a test."),
//         );

//         let mut log_contents = String::new();

//         wait_until(
//             || {
//                 if let Some(log_file) = std::fs::read_dir(&temp_dir)
//                     .expect("Failed to read directory")
//                     .filter_map(Result::ok)
//                     .filter(|entry| entry.path().is_file())
//                     .next()
//                 {
//                     let log_file_path = log_file.path();
//                     log_contents = std::fs::read_to_string(&log_file_path)
//                         .expect("Error while reading log file");
//                     !log_contents.is_empty()
//                 } else {
//                     false
//                 }
//             },
//             Duration::from_secs(2),
//         );

//         assert_eq!(
//         log_contents,
//         "{\"timestamp\":1650000000000000,\"level\":\"INFO\",\"component\":\"RiskEngine\",\"message\":\"This is a test.\"}\n"
//     );
//     }
// }
