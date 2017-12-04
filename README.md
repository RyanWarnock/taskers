# Taskers
### The Tacky Task Tracker For Tiresome Tasks

This is just a project built while learning the ropes in rust.
Please, if you notice something that can be improved make an issue :)

###### Download and try it! It's just a `cargo run` command away!

#### What does it do?
Outputs the contents of a task list file to a terminal.
You can create, remove and mark tasks as complete.

Everything is stored in a .taskers folder in your home directory. When run Taskers will create a unique file for the directory you are in.
This allows different task lists to be created so you have seperation of tasks for different projects without any extra work by the user.

#### How to use it
1. When started, the program will attempt to open a task list file in a .taskers folder in your home directory. If it fails it will attempt to create it, if that fails it will try to fall back to just creating a task.list file in the current directory.
1. Each directory gets it's own unique task list.
1. Once the task list file is loaded the file contents will be parsed into a `TaskList` struct and printed out to screen if there is any.
1. A menu will appear listing the current options

It's pretty simple

#### Things still to do
* Actually handle errors properly
* Sub tasks
* Store tasks in YAML or JSON file
* Vim-like interface
* Alarms/Reminders
* Tests

#### Things completed
* Record when a task was created
* Store tasks in a csv file
* Context aware (use specific task lists for different files/folders)
