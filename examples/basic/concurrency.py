from tm import task, Graph, Executor
from time import sleep

# Tasks in the same graph run on the same thread
# But each graph runs on a separate subprocess

@task()
def sleep_5_secs():
    print("Graph 1: Ran before Graph 2")
    print("Graph 1: Sleeping for 5 seconds")
    sleep(5)
    print("Graph 1: Woke up after Graph 2")

graph1 = Graph(schedule="* * * * *")
graph1.add_edge(sleep_5_secs, [])

@task()
def every_minute():
    print("Graph 2: Hello, world!")

graph2 = Graph(schedule="* * * * *")
graph2.add_edge(every_minute, [])

executor = Executor(graphs=[graph1, graph2])
executor.start()

