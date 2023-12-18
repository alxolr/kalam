# Kalam

## Install
```
cargo install kalam
```

## Usage
```
USAGE:
    kalam <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    list      
    path      
    report    
    start     
    status    
    stop
```

## Example
Track a new task in a project
```
kalam start -p "Project1" -t "Task1"

Entry {
    id: "a9e4aab7-4064-4ab9-a498-c0063eddedd9",
    title: "Task1",
    project: "Project1",
    action: Start,
    created_at: "2023-12-18T13:44:07.871786624+02:00",
    updated_at: "2023-12-18T13:44:07.871786624+02:00",
}
```

Get the status of the task
```
kalam status

a9e4aab7-4064-4ab9-a498-c0063eddedd9 Task1 [Project1] 0.02 hours
```

Stop the task
```
kalam stop
```

Get the report of the task
```
kalam report -p "Project1" -r all

Project1 [0.04 hours]
      a9e4aab7 # Task1                                    (18-12-2023 13:44) +0.04 hours

Total: 0.04 hours
```
