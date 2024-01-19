# Taskmaster

Taskmaster is a WIP Orchestration Framework for Python, written in Rust. This project aims to be somewhere between Airflow and crontab, with two main submodules for parsing and evaluating cron expressions, and a DAG engine with message passing capabilities.

## Features

- Simple API. With decorators for Task creation and message passing using returns and arguments.

- Written in Rust, to support the "Rewrite it in Rust" movement.

- Supports communication between Tasks through passing dicts as returns and parameters. The root task can also accept configs as parameters.

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
from tm import Expression
from datetime import datetime

expr = Expression("1-2,*/15 0 1 3 2")
t = datetime(2024,1,31,20)
t = expr.next(t)
print(t)
```

#### Using the DAG

```python
from tm import Task, Graph

# The executor handling the DAG
# takes a scehdule and the parameters to be passed to root 
# at execution time. Args will be passed as main=args to root
graph = Graph(schedule="* * * * *", args={"init":0})

## 0 -> root_add_2 --> add3 ---------|
##            |                      V
##            -----------> print_return_none ----> leaf

@Task
def root_add_2(main):
    return {"value":main["init"] + 2}

@Task
def add_3(root_add_2):
    msg = root_add_2["value"]+3
    return {"key": msg}

@Task
def print_return_none(root_add_2, add_3):
    print(root_add_2["value"] + add_3["key"]) # prints 7
    # Can also return None

@Task
def leaf(print_return_none):
    print(print_return_none == None) # print true


# Can accept multiple roots
graph.add_root(root_add_2)
graph.add_edge(root_add_2, [add_3, print_return_none])
graph.add_edge(add_3, [print_return_none])
graph.add_edge(print_return_none, [leaf])

graph.start()
```