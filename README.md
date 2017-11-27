# Taskers
### The Tacky Task Tracker For Tiresome Tasks

This is just a project built while learning the ropes in rust.
Please, if you notice something that can be improved make an issue :)

###### Download and try it! It's just a `cargo run` command away!

#### What does it do?
Outputs the contents of a task.list file to a terminal.
You can create, remove and mark tasks as complete.

Everything is stored in the "tasks.list" csv file. That file name is currently hardcoded in.

#### How to use it
1. When started, the program will attempt to open the task.list file in it's directory. If it fails it will attempt to create it.
1. Once the task.list file is loaded the file contents will be parsed into a `TaskList` struct and printed out to screen if there is any.
1. A menu will appear listing the current options

It's pretty simple

#### Things still to do
* Actually handle errors properly.
* Record when a task was created, record a date they should be finished by.
* Sub tasks.
