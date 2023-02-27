use miette::Result;
use std::path::PathBuf;

use crate::tempfile::Tempfile;

enum TaskContent {
    File(PathBuf),
    Mem(Vec<u8>),
}

enum Task {
    Map(Box<dyn FnMut(TaskContent) -> Result<TaskContent>>),
    Reduce(Box<dyn FnOnce(Vec<TaskContent>) -> Result<TaskContent>>),
    Collect(Box<dyn FnMut(TaskContent) -> Result<Vec<TaskContent>>>),
}

pub struct TaskEval(Vec<Task>);

impl TaskEval {
    fn exec_top(mut self, param: Vec<TaskContent>) -> Result<Vec<TaskContent>> {
        let task = self.0.pop();
        match task {
            None => Ok(param),
            Some(Task::Collect(mut f)) => {
                let mut result = Vec::new();
                for item in param {
                    let mut r = (*f)(item)?;
                    result.append(&mut r);
                }
                self.exec_top(result)
            }
            Some(Task::Map(mut f)) => {
                let mut result = Vec::new();
                for item in param {
                    let r = (*f)(item)?;
                    result.push(r);
                }
                self.exec_top(result)
            },
            Some(Task::Reduce(f)) => {
              let result = f(param)?;
              self.exec_top(vec![result])
            },
        }
    }

    pub fn exec(mut self, param: TaskContent) -> Result<Vec<TaskContent>> {
        self.0.reverse();
        self.exec_top(vec![param])
    }
}
