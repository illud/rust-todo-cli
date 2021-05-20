#[macro_use]
extern crate prettytable;
use prettytable::Table;
use read_input::prelude::*;
// use std::io;

#[derive(Debug)]
struct Todo {
    id: String,
    title: String,
    content: String,
}

fn main() {
    let mut todo_vec = vec![];

    let mut table_menu = Table::new();
    let mut data = Table::new();
    data.add_row(row!["ID", "TITLE", "CONTENT"]);

    table_menu.add_row(row!["Menu", "Action"]);
    table_menu.add_row(row!["1", "Add"]);
    table_menu.add_row(row!["2", "Show"]);
    table_menu.add_row(row!["3", "Delete"]);

    table_menu.printstd();

    loop {
        let input_main = input::<String>().msg(">>> ").get();
        if input_main == "1" {
            let id = input::<String>().msg(">>> ID: ").get();
            let title = input::<String>().msg(">>> TITLE: ").get();
            let content = input::<String>().msg(">>> CONTENT: ").get();

            data.add_row(row![id, title, content]);
            todo_vec.push(Todo {
                id: String::from(id),
                title: String::from(title),
                content: String::from(content),
            });
        }

        if input_main == "2" {
            data.printstd();
        }

        if input_main == "3" {
            let id_to_delete = input::<String>().msg(">>> ID TO DELETE: ").get();

            for (i, x) in todo_vec.iter().enumerate() {
                if id_to_delete == x.id {
                    println!("{}", i);
                    // todo_vec.remove(1);
                    data.remove_row(i + 1);
                    // todo_vec.remove_row(Todo.id);
                } else {
                    // data.add_row(row![x.id, x.title, x.content]);
                    println!("Not Found");
                }
            }
        }
    }
}
