from tm import task, Graph, Executor

@task()
def hello_world():
    print("Hello, world!")

graph = Graph(name="hello world", schedule="* * * * *")

graph.add_edges([hello_world])

executor = Executor(graphs=[graph])
executor.start()

