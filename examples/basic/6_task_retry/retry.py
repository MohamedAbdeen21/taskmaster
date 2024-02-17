# Retry when tasks fail

from tm import task, Graph, Executor

calls = 0

# Fail on the first 3 calls and succeed on the 4th try
# first run + 3 retries = 4 total calls
# first retry delay is 1.5, second is 3, fourth is 4
@task(retries = 3, retry_delay = 1.5, backoff = 1)
def can_fail():
    global calls
    calls += 1
    if calls == 4: # 1 call + 3 retries
        return None
    raise(ValueError)

# create the graph
graph = Graph(name="retry demo", schedule="* * * * *")

# add tasks to the graph
graph.add_edges([can_fail])

# register graphs to an executor and start it
Executor(graphs=[graph]).start()
