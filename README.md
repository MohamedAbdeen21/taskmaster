# Taskmaster

Taskmaster is a WIP Orchestration Framework for Python, written in Rust. This project aims to be somewhere between Airflow and crontab, with two main submodules for parsing and evaluating cron expressions, and a DAG engine with message passing capabilities.

## Features

- Written in Rust, to support the "Rewrite it in Rust" movement.

- Retry-on-fail, with configurable retry delays and exponential backoff.

- Supports communication between Tasks through passing dicts as returns and arguments.

- Reloads the DAG config file each run, so you can parametrize the DAG during runtime.

- A from-scratch cron expression parser and evaluator. In case you just need a cron parser without the DAG engine.

- Graph cycle detection.

- Manual execution to support re-using graphs.

- Standalone deployment, with embedded SQLite3 and cache server.

- Simple API. Check examples folder to get started.

## Getting Started

### Installation

> [!NOTE]  
> Will soon be available on PyPi.

1. Clone the repository:

   ```bash
   git clone https://github.com/MohamedAbdeen21/taskmaster.git
   ```

2. Install 

   ```bash
   cd taskmaster
   pip install maturin && maturin build
   pip install .
   ```

### Example Usage

Check the `examples` folder for more examples.

#### Using the cron submodule
```python
from tm.cron import Expression
from datetime import datetime

# supports:

# - Ranges (inclusive)
# --- (0-15 * * * *)

# - Intervals 
# --- (*/15 * * * *) every fifteen minutes
# --- (0-30/15 * * * *) every fifteen minutes from 0 to 30

# - Lists 
# --- (0 0,12 * * *) once at midnight and midday everyday

# - Exact values
# --- (0 0 1 1 *) once at start of year
# --- (0 0 * * 0) once every Sunday (Sunday = 0, Saturday = 6)

# - month/day-of-week names (case-insensitive)
# --- (0 0 1 jan-jun *)
# --- (0 0 1 * mon-thu)

# - combinations of all the above
# --- (40,0-30/15 * * * *)
# --- (* */12 2 jan,jun thu-sat)

expr = Expression("1-2,*/15 0 1 3 2")
t = datetime(2024,1,31,20)
t = expr.next(t)
print(t)
```

#### Using the DAG

```python
from tm import Graph, Executor, task

# The executor handling the DAG
# takes a scehdule and path of json file to be passed to root
# at execution time. Args will be passed in "config" kwarg to root

graph = Graph(name="test workflow", schedule="* * * * *")

##  pass_2 --> add_3 ---------|
##    |                      V
##    -----------> print_return_none ----> leaf

@task()
def pass_2():
    return {"value": 2}

# Read parent output using keyword arguments
@task()
def add_3(pass_2):
    msg = pass_2["value"]+3
    return {"key": msg}

# Can have multiple parents
@task()
def print_return_none(pass_2, add_3):
    print(pass_2["value"] + add_3["key"]) # prints 7
    # Can also return None

# Can receive None as input
@task()
def leaf(print_return_none):
    print(print_return_none == None) # print true

# define the DAG
graph.add_edges([pass_2], [add_3, print_return_none])
graph.add_edges([add_3], [print_return_none])
graph.add_edges([print_return_none], [leaf])

executor = Executor()
executor.add(graph)
executor.start()
```

#### Using config files

- Create a json file to be passed to root as argument. Let's call it `config.json`.
```json
{
    "initial_value": 2
}
```

```python
from tm import Graph, Executor, task

# Pass the path of the file to the dag, 
# The config is read every time the dag is executed
graph = Graph(
    name="configs demonstration",
    schedule="* * * * *",
    config="./config.json"
    )

## config --> print_add
##   |
##   ---> print_sub

@task()
def print_add(config):
    print(config["initial_value"] + 2) # prints 4

@task()
def print_sub(config):
    print(config["initial_value"] - 2) # prints 0

# second arguments "children" is None in this case
graph.add_edges([print_add, print_sub])

executor = Executor()
executor.add(graph)
executor.start()
```

