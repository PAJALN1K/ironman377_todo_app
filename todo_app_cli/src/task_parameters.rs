// Составляющие структуры едичной задачи Task

use std::fmt;

#[derive(Debug)]
pub enum TaskImportance {
    High,
    Medium,
    Low,
    Unmarked,
}

impl fmt::Display for TaskImportance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum TaskLifesphere {
    Study,
    Relationships,
    Health,
    Hobby,
    Leisure,
    Unmarked
}

impl fmt::Display for TaskLifesphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum TaskState {
    Completed,
    Unfinished,
    Rejected,
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
