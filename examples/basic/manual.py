# Run a graph without the executor (i.e. manual trigger)

from tm import task, Graph

@task()
def hello_world():
    print("Hello, world!")

# both name and schedule arguments are required
# manual graphs are rejected by the executor
graph = Graph(name="hello world", schedule="manual")

# add tasks to the graph
graph.add_edges([hello_world])

# Validate and sort the graph
graph.commit()

# Run the graph
graph()
