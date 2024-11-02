mod message {
    use chrono::{DateTime, Local};

    use crate::{formatter::Formatter, writer::Writer};

    #[derive(Debug)]
    pub struct Message {
        pub message: String,
        pub timestamp: DateTime<Local>,
    }

    impl Message {
        pub fn new(message: String) -> Self {
            Self {
                message,
                timestamp: Local::now(),
            }
        }
    }

    pub struct MessageService<'a, F: Formatter, W: Writer> {
        formatter: &'a F,
        writer: &'a W,
    }

    impl<'a, F: Formatter, W: Writer> MessageService<'a, F, W> {
        pub fn new(formatter: &'a F, writer: &'a W) -> Self {
            Self { formatter, writer }
        }
        pub fn send(&self, message: &Message) {
            self.writer.write(&self.formatter.format(message));
        }
    }
}

mod formatter {
    use crate::message::Message;
    pub trait Formatter {
        fn format(&self, message: &Message) -> String;
    }

    pub struct JsonFormatter;
    impl Formatter for JsonFormatter {
        fn format(&self, message: &Message) -> String {
            format!(
                r#"{{"message":"{}","timestamp":"{}"}}"#,
                message.message, message.timestamp
            )
        }
    }

    pub struct TextFormatter;
    impl Formatter for TextFormatter {
        fn format(&self, message: &Message) -> String {
            format!("{}: {}", message.timestamp, message.message)
        }
    }
}

mod writer {
    use std::fs::OpenOptions;
    use std::io::Write;

    pub trait Writer {
        fn write(&self, content: &str);
    }

    pub struct ConsoleWriter;
    impl Writer for ConsoleWriter {
        fn write(&self, content: &str) {
            println!("{}", content);
        }
    }

    pub struct FileWriter;
    impl Writer for FileWriter {
        fn write(&self, content: &str) {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(concat!(env!("CARGO_MANIFEST_DIR"), "/output.txt"))
                .unwrap();
            writeln!(file, "{}", content).unwrap();
        }
    }
}

fn main() {
    use message::Message;
    let message = Message::new("Hello, world!".to_string());

    use formatter::JsonFormatter;
    use formatter::TextFormatter;
    use message::MessageService;
    use writer::ConsoleWriter;
    use writer::FileWriter;

    let formatter = JsonFormatter {};
    let writer = ConsoleWriter {};
    let service: MessageService<'_, JsonFormatter, ConsoleWriter> =
        MessageService::new(&formatter, &writer);
    service.send(&message);

    let formatter = TextFormatter {};
    let writer = FileWriter {};
    let file_service = MessageService::new(&formatter, &writer);
    file_service.send(&message);
}
