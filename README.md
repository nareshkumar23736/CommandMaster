CommandMaster is a simple command execution tool that can execute commands one after another n times.

### How to use
1)create command_executor.yaml file in the location where CommandMaster is placed
Example content:

```
commands:
  - name: test1
    command:
     - pwd
    number_of_times: 10
```

name: Any arbitrary name that suites your needs

command: under this define the commands you want to execute one after the other

exec_per_sec: number of times the command needs to be executed.
