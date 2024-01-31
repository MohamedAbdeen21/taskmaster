from tm import task, Graph, Executor

calls = 0

@task(retries = 3, retry_delay = 1.5, backoff = 1)
def can_fail():
    global calls
    calls += 1
    if calls < 4: # 1 call + 3 retries
        raise(ValueError)
    return None

graph = Graph(schedule="* * * * *")
graph.add_edges([can_fail])

Executor(graphs=[graph]).start()
