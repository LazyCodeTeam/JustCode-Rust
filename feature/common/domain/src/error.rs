use snafu::{Backtrace, Snafu};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(whatever, display("{message} --- {source:?}\n{backtrace}"))]
pub struct Error {
    message: String,
    #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
    backtrace: Backtrace,
}

impl<TOk, TErr> ResultLogExt<TOk> for std::result::Result<TOk, TErr>
where
    TErr: std::error::Error,
{
    fn log(&self, level: log::Level) {
        if let Err(e) = self {
            log::log!(level, "{}", e);
        }
    }
}

pub trait ResultLogExt<T>: Sized {
    fn log(&self, level: log::Level);

    fn with_log(self, level: log::Level) -> Self {
        self.log(level);
        self
    }

    fn with_error_log(self) -> Self {
        self.with_log(log::Level::Error)
    }

    fn with_warn_log(self) -> Self {
        self.with_log(log::Level::Warn)
    }

    fn with_info_log(self) -> Self {
        self.with_log(log::Level::Info)
    }

    fn with_debug_log(self) -> Self {
        self.with_log(log::Level::Debug)
    }

    fn with_trace_log(self) -> Self {
        self.with_log(log::Level::Trace)
    }

    fn log_error(&self) {
        self.log(log::Level::Error)
    }

    fn log_warn(&self) {
        self.log(log::Level::Warn)
    }

    fn log_info(&self) {
        self.log(log::Level::Info)
    }

    fn log_debug(&self) {
        self.log(log::Level::Debug)
    }

    fn log_trace(&self) {
        self.log(log::Level::Trace)
    }
}

#[cfg(test)]
mod test {
    use snafu::whatever;

    use super::*;

    fn test_error(s: &str) -> Result<()> {
        whatever!("{}", s)
    }

    #[test]
    fn no_error() {
        testing_logger::setup();
        Ok::<(), Error>(()).log(log::Level::Error);

        testing_logger::validate(|captured_logs| {
            assert!(captured_logs.is_empty());
        });
    }

    #[test]
    fn log_error() {
        testing_logger::setup();
        test_error("test log").log_error();

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, log::Level::Error);
            assert!(captured_logs[0].body.contains("test log"));
        });
    }

    #[test]
    fn log_warn() {
        testing_logger::setup();
        test_error("test log").log_warn();

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, log::Level::Warn);
            assert!(captured_logs[0].body.contains("test log"));
        });
    }

    #[test]
    fn log_info() {
        testing_logger::setup();
        test_error("test log").log_info();

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, log::Level::Info);
            assert!(captured_logs[0].body.contains("test log"));
        });
    }

    #[test]
    fn log_debug() {
        testing_logger::setup();
        test_error("test log").log_debug();

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, log::Level::Debug);
            assert!(captured_logs[0].body.contains("test log"));
        });
    }

    #[test]
    fn log_trace() {
        testing_logger::setup();
        test_error("test log").log_trace();

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, log::Level::Trace);
            assert!(captured_logs[0].body.contains("test log"));
        });
    }

    #[test]
    fn with_error_log() {
        testing_logger::setup();
        let _ = test_error("test log").with_error_log();

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, log::Level::Error);
            assert!(captured_logs[0].body.contains("test log"));
        });
    }

    #[test]
    fn with_warn_log() {
        testing_logger::setup();
        let _ = test_error("test log").with_warn_log();

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, log::Level::Warn);
            assert!(captured_logs[0].body.contains("test log"));
        });
    }

    #[test]
    fn with_info_log() {
        testing_logger::setup();
        let _ = test_error("test log").with_info_log();

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, log::Level::Info);
            assert!(captured_logs[0].body.contains("test log"));
        });
    }

    #[test]
    fn with_debug_log() {
        testing_logger::setup();
        let _ = test_error("test log").with_debug_log();

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, log::Level::Debug);
            assert!(captured_logs[0].body.contains("test log"));
        });
    }

    #[test]
    fn with_trace_log() {
        testing_logger::setup();
        let _ = test_error("test log").with_trace_log();

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].level, log::Level::Trace);
            assert!(captured_logs[0].body.contains("test log"));
        });
    }
}
