mod sign_logger {
    use chrono::{Datelike, Timelike, Utc};
    macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}
    pub struct OverwriteDate {
        show: bool,
        format: String
    }
    pub struct Coloring {
        fatal: String,
        error: String,
        info: String,
        debug: String,
    }
    pub struct Overwrite {
        symbols: (String, String),
        title: String,
        color: String,
        title_underline: bool,
        underline: bool,
        date: OverwriteDate
    }
    pub enum Colors {
        UNDERLINE,
        BLUE,
        BROWN,
        RED,
        BLACK,
        GREEN,
        YELLOW,
        MAGENTA,
        CYAN,
        WHITE,
        RESET
    }
    impl Colors {
        pub fn resolve(&self) -> &'static str {
            match self {
                Self::UNDERLINE => "\u{001b}[4m",
                Self::BLUE => "\u{001b}[34;1m",
                Self::BROWN => "\u{001b}[0;33m",
                Self::RED => "\u{001b}[31;1m",
                Self::BLACK => "\u{001b}[30;1m",
                Self::GREEN => "\u{001b}[32;1m",
                Self::YELLOW => "\u{001b}[33;1m",
                Self::MAGENTA => "\u{001b}[35;1m",
                Self::CYAN => "\u{001b}[36;1m",
                Self::WHITE => "\u{001b}[37;1m",
                Self::RESET => "\u{001b}[0m",
            }
        }
    }

    pub struct Logger {
        pub(crate) symbols: (String, String),
        pub(crate) colors: Coloring,
        pub(crate) date: String
    }
    impl Logger {
        pub fn init(symbols: Option<(String, String)>, colors: Option<Coloring>, date: Option<String>) -> Self{
            Self {
                symbols: symbols.unwrap_or(("[".parse().unwrap(), "]".parse().unwrap())),
                colors: colors.unwrap_or(Coloring {
                    fatal: Colors::WHITE.resolve().parse().unwrap(),
                    error: Colors::RED.resolve().parse().unwrap(),
                    info: Colors::BLUE.resolve().parse().unwrap(),
                    debug: Colors::MAGENTA.resolve().parse().unwrap()
                }),
                date: date.unwrap_or("YY-MM-DD HH:MI".parse().unwrap())
            }
        }
        pub fn info(&self, message: String, overwrite: Option<Overwrite>) {
            self.inner(message.parse().unwrap(),"info", overwrite)
        }
        pub fn fatal(&self, message: String, overwrite: Option<Overwrite>) {
            self.inner(message.parse().unwrap(),"fatal", overwrite)
        }
        pub fn error(&self, message: String, overwrite: Option<Overwrite>) {
            self.inner(message.parse().unwrap(),"error", overwrite)
        }
        pub fn debug(&self, message: String, overwrite: Option<Overwrite>) {
            self.inner(message.parse().unwrap(),"debug", overwrite)
        }
        fn date_format(&self, arg: &str) -> String {
            let now = Utc::now();
            return format!("{}{}{}{}{}", Colors::WHITE.resolve(),ternary!(arg.chars().count() >= 1, "[", ""),
                arg.to_uppercase()
                    .replace("YY", &*now.year().to_string())
                    .replace("MM", &*now.month().to_string())
                    .replace("DD", &*now.date().to_string())
                    .replace("HH", &*now.hour().to_string())
                    .replace("MI", &*now.minute().to_string())
                    .replace("SS", &*now.second().to_string())
                    .replace("MS", &*now.timestamp_millis().to_string()),
                ternary!(arg.chars().count() >= 1, "]", ""), Colors::RESET.resolve()
            )
        }
        fn inner(&self, message: &str, name: &str, overwrite: Option<Overwrite>) {
            let mut pre_color;
            match name.to_lowercase().as_str() {
                "fatal" => { pre_color = logger.colors.fatal }
                "error" => { pre_color = logger.colors.error }
                "debug" => { pre_color = logger.colors.debug }
                "info" => { pre_color = logger.colors.info }
                _ => { pre_color = Colors::WHITE.resolve().parse().unwrap() }
            }
            let options = overwrite.unwrap_or(Overwrite {
                symbols: ("[".parse().unwrap(), "]".parse().unwrap()),
                title: name.parse().unwrap(),
                color: pre_color,
                title_underline: false,
                underline: false,
                date: OverwriteDate {
                    show: true,
                    format: logger.date
                }
            });
            let mut res = logger.symbols;
            let mut title;
            let mut format;
            let color= options.color;
            if options.date.show == false {
                format = "";
            }else if options.date.format {
                format = &*options.date.format
            } else {
                format = &*logger.date;
            }
            if !options.title {
                title = name;
            }
            if options.title.chars().count() <= 0 {
                res = ("".parse().unwrap(), "".parse().unwrap());
                title = "";
            }else if options.title {
                title = &*options.title;
            }
            println!("{}  {}{}{}{}{}{}{}{}{}{}{}", self.date_format(format), color, res.0, ternary!(options.underline, Colors::UNDERLINE.resolve(), ""), ternary!(name.to_lowercase() == "fatal", format!("\u{001b}[41m{}",title)),Colors::RESET.resolve(), ternary!(options.color, options.color, color),res.1, " ".repeat(8 - title.chars().count() + title.chars().count()/2), ternary!(options.underline, Colors::UNDERLINE.resolve(), ""), message, Colors::RESET.resolve())
        }
    }
}