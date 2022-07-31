# What Was I Doing Again?
Simple CLI to-do list app.

```txt
wwida 1.0.0

USAGE:
    wwida <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add       Add a new task to the to-do list
    clear     Deletes all tasks
    help      Print this message or the help of the given subcommand(s)
    print     Print misc. summaries
    start     Set the status of a task
    update    Update a task
```

## Tasks
Each task has the following properties:

* Short description (<= 50 characters)
* Long description (optional)
* Category
    * task (default)
    * feature
    * bug
    * maintenance
    * documentation
    * message
    * other
* Status
* Deadline (optional)

All tasks are referred to by an incremental ID.

## Features
Add a new task
```txt
wwida-add 
Add a new task to the to-do list

USAGE:
    wwida add [OPTIONS] <SHORT>

ARGS:
    <SHORT>    Short description of the task, must be <= 50 chars long

OPTIONS:
    -c, --category <CATEGORY>    Task category [possible values: task, feature, bug, maintenance,
                                 documentation, other, message]
    -d, --deadline <DEADLINE>    Task deadline, e.g. today, tomorrow, this/tuesday, next/friday or
                                 01/08/2022
    -h, --help                   Print help information
    -l, --long <LONG>            Optional long description of the task
```

Update an existing tasks's parameters
```txt
wwida-update 
Update a task

USAGE:
    wwida update <ID> <SUBCOMMAND>

ARGS:
    <ID>    Task ID

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    category            Updates the tasks's category
    deadline            Updates the tasks's deadline
    discard-deadline    Removes the task's long description
    discard-long        Removes the task's long description
    help                Print this message or the help of the given subcommand(s)
    long                Updates the tasks's long description
    short               Updates the tasks's short description
    status              Updates the tasks's status
```

Print summaries of completed/pending tasks
```txt
wwida-print 
Print misc. summaries

USAGE:
    wwida print [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -f, --format <FORMAT>    [default: long] [possible values: short, long]
    -h, --help               Print help information

SUBCOMMANDS:
    completed    Show all items completed in the past duration
    help         Print this message or the help of the given subcommand(s)
    pending      Show all items still pending
```