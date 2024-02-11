from tm import task, Graph, Executor

@task()
def read_config(config):
    print("Value of 'key' in config is =", config["key"])

graph = Graph(name="Config files demo", schedule="* * * * *", config="./cfg.json")

graph.add_edges([read_config])

executor = Executor(graphs=[graph])
executor.start()

