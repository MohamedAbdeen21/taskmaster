# Taskmaster

Taskmaster is a WIP Orchestration Framework for Python, written in Rust. This project aims to be somewhere between Airflow and crontab, with two main submodules for parsing and evaluating cron expressions, and a DAG engine with message passing capabilities.

## Features

- Simple API. With decorators for Task creation and message passing using returns and arguments.

- Written in Rust, to support the "Rewrite it in Rust" movement.

- Supports communication between Tasks through passing dicts as returns and parameters. The root task can take a json file as input.

- Reloads the root task input file each run, so you can parametrize the DAG during runtime.

- A from-scratch cron expression parser and evaluator. In case you just need a cron parser without the DAG engine.

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
from tm import Graph, Executor

# The executor handling the DAG
# takes a scehdule and path of json file to be passed to root
# at execution time. Args will be passed in "config" kwarg to root

graph = Graph(schedule="* * * * *")

##  pass_2 --> add_3 ---------|
##    |                      V
##    -----------> print_return_none ----> leaf

# Use **kwargs to ignore input
def pass_2(**kwargs):
    return {"value": 2}

# Read parent output using keyword arguments
def add_3(pass_2):
    msg = pass_2["value"]+3
    return {"key": msg}

# Can have multiple parents
def print_return_none(pass_2, add_3):
    print(root_add_2["value"] + add_3["key"]) # prints 7
    # Can also return None

# Can receive None as input
def leaf(print_return_none):
    print(print_return_none == None) # print true

# define the DAG
graph.add_edge(pass2, [add_3, print_return_none])
graph.add_edge(add_3, [print_return_none])
graph.add_edge(print_return_none, [leaf])

executor = Executor()
executor.add(graph)
executor.start()
```

#### Using config files

- Create a json file to be passed to root as argument
```json
-- filename: config.json
{
    "initial_value": 2
}
```

```python
from tm import Graph, Executor

# Pass the absolute path of the file to the dag, 
# The config is read every time the dag is executed
graph = Graph(schedule="* * * * *", config="/src/config.json")

## config --> print_add
##   |
##   ---> print_sub

# Use **kwargs to ignore input
def print_add(config):
    print(config["initial_value"] + 2) # prints 4

def print_sub(config):
    print(config["initial_value"] - 2) # prints 0

graph.add_edge(print_add, [])
graph.add_edge(print_sub, [])

executor = Executor()
executor.add(graph)
executor.start()
```

