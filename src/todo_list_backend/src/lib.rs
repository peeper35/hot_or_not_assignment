use candid::{types::number::Nat, CandidType, Deserialize};
use ic_cdk::{query, update};
use std::{cell::RefCell, collections::BTreeMap};

#[derive(Debug, Clone, Default, CandidType, Deserialize)]
pub struct TodoTask {
    pub title: String,
    pub desc: String,
    pub completed: bool,
}

type TodoListStore = BTreeMap<Nat, TodoTask>;

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(1));
    static TODO_LIST_STORAGE: RefCell<TodoListStore> = RefCell::default();
}

#[update]
fn create_todo(title: String, desc: String) -> Nat {
    COUNTER.with(|counter| {
        let id = (*counter.borrow()).clone();

        TODO_LIST_STORAGE.with(|store| {
            store.borrow_mut().insert(
                id.clone(),
                TodoTask {
                    title,
                    desc,
                    completed: false,
                },
            )
        });

        *counter.borrow_mut() += 1;

        id
    })
}

#[query]
fn fetch_todo(id: Nat) -> Result<TodoTask, String> {
    TODO_LIST_STORAGE.with(|store| {
        let task_store = store.borrow();

        if let Some(todo_task) = task_store.get(&id) {
            Ok((*todo_task).clone())
        } else {
            Err(String::from("Todo task not found"))
        }
    })
}

#[query]
fn fetch_all_todos(limit: usize, page_no: usize) -> Vec<(Nat, TodoTask)> {
    TODO_LIST_STORAGE.with(|store| {
        let task_store = store.borrow().clone();
        let todo_task = task_store.into_iter();

        let start = (page_no - 1) * limit;
        let end = start + limit;

        todo_task.skip(start).take(end).collect::<Vec<_>>()
    })
}

#[update]
fn update_todo(id: Nat, todo_task_update: TodoTask) -> Result<String, String> {
    TODO_LIST_STORAGE.with(|store| {
        let mut todo_store = store.borrow_mut();

        if let Some(todo_task) = todo_store.get_mut(&id) {
            todo_task.title = todo_task_update.title;
            todo_task.desc = todo_task_update.desc;
            todo_task.completed = todo_task_update.completed;

            Ok(String::from("Updated todo task"))
        } else {
            Err(String::from("Todo task not found"))
        }
    })
}

#[update]
fn delete_todo(id: Nat) -> Result<String, String> {
    TODO_LIST_STORAGE.with(|store| {
        let mut todo_store = store.borrow_mut();

        if todo_store.remove(&id).is_some() {
            Ok(String::from("Deleted todo task"))
        } else {
            Err(String::from("Todo task not found"))
        }
    })
}

ic_cdk::export_candid!();
