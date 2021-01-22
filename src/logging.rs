use log::{LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

pub fn setup() {
  let level = log::LevelFilter::Info;
  let file_path = "dessa.log";

  let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

  let logfile = FileAppender::builder()
      // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
      .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {M} {l} - {m}\n")))
      .build(file_path)
      .unwrap();

  let config = Config::builder()
      .appender(Appender::builder().build("logfile", Box::new(logfile)))
      .appender(
          Appender::builder()
              .filter(Box::new(ThresholdFilter::new(level)))
              .build("stderr", Box::new(stderr)),
      )
      .build(
          Root::builder()
              .appender("logfile")
              .appender("stderr")
              .build(LevelFilter::Info),
      )
      .unwrap();

  // Use this to change log levels at runtime.
  // This means you can change the default log level to trace
  // if you are trying to debug an issue and need more logs on then turn it off
  // once you are done.
  let _handle = log4rs::init_config(config);

  log::error!("Goes to stderr and file");
}