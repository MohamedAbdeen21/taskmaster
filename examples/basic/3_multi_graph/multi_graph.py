# Demonstrate how multiple graphs are executed

from tm import task, Graph, Executor
from time import sleep

# Tasks in the same graph run on the same thread
# But graphs run on different processes

@task()
def sleep_5_secs():
    print("Graph 1: Ran before Graph 2!")
    print("Graph 1: Sleeping for 5 seconds!")
    sleep(5)
    print("Graph 1: Finished after Graph 2!")

graph1 = Graph(name="graph 1", schedule="* * * * *")
graph1.add_edges([sleep_5_secs])

@task()
def every_minute():
    print("Graph 2: Started after graph 1!")
    print("Graph 2: and finished before graph 1!")

graph2 = Graph(name="graph 2", schedule="* * * * *")
graph2.add_edges([every_minute])

# Graphs are started in the same order given to the executor
executor = Executor(graphs=[graph1, graph2])

# Or we can also explicitly call executor.add(..) on each graph
# executor.add(graph1)
# executor.add(graph2)

executor.start()

