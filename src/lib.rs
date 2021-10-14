use chrono::{Datelike, Timelike, Utc};
macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}
pub struct OverwriteDate {
    show: bool,
    format: &'static str,
}
pub struct Coloring {
    fatal: &'static str,
    error: &'static str,
    info: &'static str,
    debug: &'static str,
}
pub struct Overwrite {
    title: &'static str,
    color: &'static str,
    underline: bool,
    date: OverwriteDate,
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
    RESET,
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
    pub(crate) symbols: (&'static str, &'static str),
    pub(crate) colors: Coloring,
    pub(crate) date: &'static str,
}
impl Logger {
    pub fn init(
        symbols: Option<(&'static str, &'static str)>,
        colors: Option<Coloring>,
        date: Option<&'static str>,
    ) -> Self {
        Self {
            symbols: symbols.unwrap_or(("[", "]")),
            colors: colors.unwrap_or(Coloring {
                fatal: Colors::WHITE.resolve(),
                error: Colors::RED.resolve(),
                info: Colors::BLUE.resolve(),
                debug: Colors::MAGENTA.resolve(),
            }),
            date: date.unwrap_or("YY-MM-DD HH:MI"),
        }
    }
    pub fn info(&self, message: String, overwrite: Option<Overwrite>) {
        self.inner(message.as_str(), "info", overwrite)
    }
    pub fn fatal(&self, message: String, overwrite: Option<Overwrite>) {
        self.inner(message.as_str(), "fatal", overwrite)
    }
    pub fn error(&self, message: String, overwrite: Option<Overwrite>) {
        self.inner(message.as_str(), "error", overwrite)
    }
    pub fn debug(&self, message: String, overwrite: Option<Overwrite>) {
        self.inner(message.as_str(), "debug", overwrite)
    }
    fn date_format(&self, arg: &str) -> String {
        let now = Utc::now();
        return format!(
            "{}{}{}{}{}",
            Colors::WHITE.resolve(),
            ternary!(arg.chars().count() >= 1, "[", ""),
            arg.to_uppercase()
                .replace("YY", &*now.year().to_string())
                .replace("MM", &*now.month().to_string())
                .replace("DD", &*now.date().to_string())
                .replace("HH", &*now.hour().to_string())
                .replace("MI", &*now.minute().to_string())
                .replace("SS", &*now.second().to_string())
                .replace("MS", &*now.timestamp_millis().to_string()),
            ternary!(arg.chars().count() >= 1, "]", ""),
            Colors::RESET.resolve()
        );
    }
    fn inner(&self, message: &str, name: &'static str, overwrite: Option<Overwrite>) {
        let pre_color;
        match name.to_lowercase().as_str() {
            "fatal" => pre_color = self.colors.fatal,
            "error" => pre_color = self.colors.error,
            "debug" => pre_color = self.colors.debug,
            "info" => pre_color = self.colors.info,
            _ => pre_color = Colors::WHITE.resolve(),
        }
        let options = overwrite.unwrap_or(Overwrite {
            title: name,
            color: pre_color,
            underline: false,
            date: OverwriteDate {
                show: true,
                format: self.date,
            },
        });

        let mut res = self.symbols;
        let mut title = "";
        let format;
        let color = options.color;
        if options.date.show == false {
            format = "";
        } else if !options.date.format.is_empty() {
            format = &*options.date.format
        } else {
            format = &*self.date;
        }

        if options.title.is_empty() {
            title = name;
        }
        if options.title.chars().count() <= 0 {
            res = ("", "");
            title = "";
        } else if !options.title.is_empty() {
            title = &*options.title;
        }

        println!(
            "{}  {}{}{}{}{}{}{}{}{}{}{}",
            self.date_format(format),
            color,
            res.0,
            ternary!(options.underline, Colors::UNDERLINE.resolve(), ""),
            ternary!(
                name.to_lowercase() == "fatal",
                format!("\u{001b}[41m{}", title),
                "".to_string()
            ),
            Colors::RESET.resolve(),
            ternary!(options.color == options.color, color, ""),
            res.1,
            " ".repeat(8 - title.chars().count() + title.chars().count() / 2),
            ternary!(options.underline, Colors::UNDERLINE.resolve(), ""),
            message,
            Colors::RESET.resolve()
        )
    }
}
