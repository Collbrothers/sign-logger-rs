

# Sign-logger
*Rust edition*

 The best rust logger out there, framework logging support is on the horizon.

## Table of contents:

- [Getting started](#getting-started)

## Getting started

```rust
use sign_logger::*;


fn main() {
    let log = Logger::init(None, None, None);
    log.info("This message will be underlined", Some(Overwrite {
        title: "Info",
        color: Colors::BLUE.resolve(),
        underline: true,
        date: OverwriteDate {
            show: true,
            format: "YY-MM-DD HH:MI"
        }
    }));
    log.error("This message will have a modified date format", Some(Overwrite {
        title: "Error",
        color: Colors::RED.resolve(),
        underline: false,
        date: OverwriteDate {
            show: true,
            format: "YY-MM-DD"
        }
    }));
    log.debug("This message will not show the date at all", Some(Overwrite {
        title: "Error",
        color: Colors::MAGENTA.resolve(),
        underline: false,
        date: OverwriteDate {
            show: false,
            format: ""
        }
    }));
    log.fatal("This message is all default", None);
    log.fatal("This message has a different colour than default", Some(Overwrite {
        title: "Fatal",
        color: Colors::YELLOW.resolve(),
        underline: false,
        date: OverwriteDate {
            show: true,
            format: "YY-MM-DD HH:MI"
        }
    }));
}
```
Output:

<img src="https://cdn.discordapp.com/attachments/713410105928056856/898154155301036094/unknown.png"/>

## Impl Logger
Implements <a href="https://github.com/BlazifyOrg/sign-logger-rs/blob/main/src/lib.rs#L58">Logger Struct<a/>

### Logger::init
<pre class="highlight highlight-rust">
init(symbols: Option<(&'static str, &'static str)>, colors: Option<<a>Coloring</a>>, date: Option<&'static str>)
</pre>
Example Usage:
```rust
Logger::init(Some(("[","]")), Some(Coloring {
    fatal: Colors::YELLOW.resolve(),
    error: Colors::BROWN.resolve(),
    info: Colors::WHITE.resolve(),
    debug: Colors::BLACK.resolve()
}), Some("YY-MM-DD"));
```

### Logger.info & Logger.debug & Logger.error & Logger.fatal
<pre class="highlight highlight-rust">
info(&self, message: &str, overwrite: Option<<a href="https://github.com/BlazifyOrg/sign-logger-rs/blob/main/src/lib.rs#L21">Overwrite</a>>)
</pre>

## Others

### sign_logger::Colors
```rust
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
```

### sign_logger::Overwrite
```rust
pub struct Overwrite {
    pub title: &'static str,
    pub color: &'static str,
    pub underline: bool,
    pub date: OverwriteDate,
}
```
Example usage:
```rust
Logger.fatal("This message has a different colour than default", Some(Overwrite {
    title: "Fatal",
    color: Colors::YELLOW.resolve(),
    underline: false,
    date: OverwriteDate {
        show: true,
        format: "YY-MM-DD HH:MI"
}
}));
```

### sign_logger::Coloring
```rust
pub struct Coloring {
    pub fatal: &'static str,
    pub error: &'static str,
    pub info: &'static str,
    pub debug: &'static str,
}
```
Example usage:
```rust
Logger::init(None, Some(Coloring {
    fatal: Colors::RED.resolve(),
    error: Colors::YELLOW.resolve(),
    info: Colors::GREEN.resolve(),
    debug: Colors::CYAN.resolve()
}), None);
```

### sign_logger::OverwriteDate
```rust
pub struct OverwriteDate {
    pub show: bool,
    pub format: &'static str,
}
```
Example usage:
```rust
Logger.fatal("This message has a different colour than default", Some(Overwrite {
    title: "Fatal",
    color: Colors::YELLOW.resolve(),
    underline: false,
    date: OverwriteDate {
        show: true,
        format: "YY-MM-DD HH:MI"
}
}));
```