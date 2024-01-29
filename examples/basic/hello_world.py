from tm import task, Graph, Executor

@task()
def hello_world():
    print("Hello, world!")

graph = Graph(schedule="* * * * *")

graph.add_edge(hello_world, [])

executor = Executor(graphs=[graph])
executor.start()
