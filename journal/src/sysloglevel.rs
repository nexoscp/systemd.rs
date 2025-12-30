use log::Level;


//see syslog.h
pub enum SysLogLevel {
    /* systemd is unusable */
    EMERG = 0,

    /* action must be taken immediately */
    ALERT = 1,

    /* critical conditions */
    CRIT = 2,

    /* error conditions */
    ERR = 3,

    /* warning conditions */
    WARNING = 4,

    /* normal but significant condition */
    NOTICE = 5,

    /* informational */
    INFO = 6,

    /* debug-level messages */
    DEBUG = 7,
}

impl From<Level> for SysLogLevel {
    fn from(level: Level) -> SysLogLevel {
        match level {
            Level::Error => SysLogLevel::ERR,
            Level::Warn => SysLogLevel::WARNING,
            Level::Info => SysLogLevel::INFO,
            Level::Debug => SysLogLevel::DEBUG,
            Level::Trace => SysLogLevel::DEBUG
        }
    }
}

impl Into<char> for SysLogLevel {
    fn into(self) -> char {
        match self {
            SysLogLevel::DEBUG => '7',
            SysLogLevel::INFO => '6',
            SysLogLevel::NOTICE => '5',
            SysLogLevel::WARNING => '4',
            SysLogLevel::ERR => '3',
            SysLogLevel::CRIT => '2',
            SysLogLevel::ALERT => '1',
            SysLogLevel::EMERG => '0',
        }
    }
}
