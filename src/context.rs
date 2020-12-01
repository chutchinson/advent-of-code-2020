use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

pub type Solution = fn(context: &mut Context) -> std::io::Result<()>;

pub struct Context {
    file: File
}

impl Write for Context {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

impl Context {
    pub fn load(&self, path: &str) -> std::io::Result<std::io::BufReader<File>> {
        let path = format!("inputs/{}", path);
        let file = File::open(path)?;
        Ok(std::io::BufReader::new(file))
    }

    pub fn solve(solution: usize, solver: Solution) -> std::io::Result<()> {
        let root = Path::new("outputs");

        if !root.exists() {
            std::fs::create_dir(root)?;
        }

        let path = format!("outputs/{}.txt", solution);
        let path = Path::new(&path);
        let file = File::create(path)?;
        let mut context = Context { file };

        solver(&mut context)
    }
}