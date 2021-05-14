use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Beanstalker {
    stream: TcpStream,
}

impl Beanstalker {
    pub fn connect(address: &str) -> io::Result<Beanstalker> {
        let mut stream = TcpStream::connect(address)?;
        Ok(Beanstalker { stream })
    }

    fn send(&mut self, cmd: Command) -> io::Result<()> {
        &self.stream.write(cmd.construct().as_bytes())?;
        &self.stream.flush()?;

        // Read response body
        let mut buffer = vec![0; 128];
        let n = &self.stream.read(&mut buffer)?;
        let d = String::from_utf8_lossy(&buffer[..*n as usize]);
        println!("{:?}", d);
        Ok(())
    }

    pub fn put(&mut self, priority: u32, delay: u32, ttr: u32, data: &str) -> io::Result<()> {
        let args = &[priority.to_string(), delay.to_string(), ttr.to_string()];
        let cmd = Command::new(Operation::Put, args, Some(data));
        &self.send(cmd)?;

        Ok(())
    }
}

#[derive(Debug)]
enum Operation {
    Put,
    ListTubes,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match &self {
            Operation::Put => "put".to_string(),
            Operation::ListTubes => "list-tubes".to_string(),
        }
    }
}

/// Command is an internal structure to represent a beanstalkd command
#[derive(Debug)]
struct Command<'a, 'b> {
    op: Operation,
    args: Vec<&'a str>,
    data: Option<&'b str>,
}

impl<'a, 'b> Command<'a, 'b> {
    fn new<T: AsRef<str>>(op: Operation, args: &'a [T], data: Option<&'b str>) -> Self {
        Command {
            op: op,
            args: args.iter().map(|item| item.as_ref()).collect(),
            data: data,
        }
    }

    fn construct(&self) -> String {
        let crnl = "\r\n";
        let space = " ";

        let mut command_str = String::new();
        command_str.push_str(&self.op.to_string());
        command_str.push_str(space);
        command_str.push_str(&self.args.join(space));
        if let Some(data) = &self.data {
            command_str.push_str(space);
            command_str.push_str(&data.len().to_string());
            command_str.push_str(crnl);
            command_str.push_str(data);
            command_str.push_str(crnl);
        } else {
            command_str.push_str(crnl);
        };
        command_str
    }
}
