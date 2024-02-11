# Using config files to parametrize graphs

from tm import task, Graph, Executor

# Define the graph and give path to the config file to be read
graph = Graph(name="Config files demo", schedule="* * * * *", config="./cfg.json")

# Root node takes 'config' as arg
@task()
def read_config(config):
    print("Value of 'key' in config is = ", config["key"])

# Register task to graph
graph.add_edges([read_config])

# Create executor and register the graph
executor = Executor(graphs=[graph])

# start the executor
executor.start()

