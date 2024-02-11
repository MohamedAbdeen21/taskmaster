# Simple hello world program

from tm import task, Graph, Executor

# Turn a function to a Task by using the decorator
@task()
def hello_world():
    print("Hello, world!")

# Define a simple graph
graph = Graph(name="hello world", schedule="* * * * *")

# Register a task to a graph
graph.add_edges([hello_world])

# Register graph to an executor
executor = Executor(graphs=[graph])

# Start the executor
executor.start()

