# Run a graph without the executor (i.e. manual trigger)
# This can be helpful when you want to re-use graphs inside
# other tasks (check the upcoming example 2)

from tm import task, Graph

@task()
def hello_world():
    print("Hello, world!")

# both name and schedule arguments are required
# manual graphs are rejected by the executor
graph = Graph(name="hello world", schedule="manual")

# add tasks to the graph
graph.add_edges([hello_world])

# Validate and sort the graph, usually automatically 
# called by the executor
# The graph is only rejected if it has cycles
graph.commit()

# Run the graph, accepts *args and **kwargs
graph()
