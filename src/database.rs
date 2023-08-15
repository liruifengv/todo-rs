use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, Write};
// 数据记录
pub struct Record {
    pub id: i32,
    pub content: String,
}

// 解析记录行
pub fn parse_record_line(line: &str) -> Record {
    let fields: Vec<&str> = line.split(',').collect();
    // 处理空行的情况
    if fields.len() == 1 {
        return Record {
            id: 0,
            content: "".to_string(),
        };
    }
    let content = fields[1..].join(",");
    Record {
        id: fields[0].parse::<i32>().unwrap(),
        content,
    }
}

// 数据库
pub struct Database {
    pub file: File,
}

impl Database {
    // 打开数据库文件
    pub fn open(filename: &str) -> Database {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(filename)
            .unwrap();
        Database { file }
    }

    // 添加记录
    pub fn add_record(&mut self, record: &Record) -> Result<(), std::io::Error> {
        let line = format!("{},{}\n", record.id, record.content);
        writeln!(self.file, "{}", line)
    }

    // 删除记录
    pub fn remove_record(&mut self, id: i32) -> Result<(), std::io::Error> {
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();
        let line = lines.find(|(_, line)| {
            let record = parse_record_line(line.as_ref().unwrap());
            record.id == id
        });
        match line {
            Some((i, _)) => {
                let contents = fs::read_to_string(".rodorc").unwrap();
                let new_contents = contents
                    .lines()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, line)| line)
                    .collect::<Vec<_>>()
                    .join("\n");
                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_contents.as_bytes()).unwrap();
                self.file.set_len(new_contents.len() as u64).unwrap();
                Ok(())
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("No such record: {}", id),
            )),
        }
    }

    // 读取记录
    pub fn read_records(&mut self) -> Vec<Record> {
        let reader = BufReader::new(&self.file);
        reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| parse_record_line(&line))
            .collect()
    }
}
