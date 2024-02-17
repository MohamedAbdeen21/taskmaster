# Using config files to parametrize graphs

from tm import task, Graph, Executor

# Define the graph and give path to the config file to be read
# By default, config files are serialized and cached.
# If the file has changed since the last run, re-read the file and update the cache
# Try to run this demo and change the config file while it's running
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

