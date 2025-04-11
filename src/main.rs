use std::{ffi::OsStr, process::{Command, Output}};

use winres::WindowsResource;

struct Python {
    python: String,
}

impl Python {

    fn new(python_path: &str) -> Self { 
        Python {
            python : String::from(python_path),
        }
    }


    fn py_exec(&self) -> Command {
        return Command::new(self.python.clone());
    }


    fn args<I, S>(&self, args: I) -> String 
    where         
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output: Output = self.py_exec()
        .args(args)
        .output()
        .expect("Failed to execute Python");

        return Python::flush_io(output);
    }


    fn exec_pip_args<I, S>(&self, args: I) -> String 
    where         
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output: Output = self.py_exec()
            .args(["-m", "pip"])
            .args(args)
            .output()
            .expect("Failed to execute Python");

        return Python::flush_io(output);
    }


    fn flush_io(output: Output) -> String {
        if output.status.success() {
            unsafe { return String::from_utf8_unchecked(output.stdout) };
        } else {
            unsafe { return String::from_utf8_unchecked(output.stderr) };
        }
    }
}


fn main() {
    let python: Python = Python::new(r"winpy\python\python.exe");
    let _ = WindowsResource::new()
        .set_icon("btg.ico")
        .compile()
        .unwrap();

    print!("{}", python.args(["--version"]));
    print!("{}", python.exec_pip_args(["--version"]));
    print!("{}", python.exec_pip_args(["list"]));
}
