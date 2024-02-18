# Simple hello world program

from tm import task, Graph, Executor

# Turn a function to a Task by using the decorator
@task()
def hello_world():
    print("Hello, world!")

# Define a simple graph that runs every minute
graph = Graph(name="Hello world", schedule="* * * * *")

# Register a task to a graph
graph.add_edges([hello_world])

# Register graph to an executor
# Executors ensure that graphs run in-parallel and on-time
executor = Executor(graphs=[graph])

# Start the executor
executor.start()

