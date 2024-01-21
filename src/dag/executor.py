import time
from multiprocessing import Process
from datetime import datetime, timezone

# When PyO3 executes Python code, it has to 
# acquire the GIL. PyO3 support for sub-interpreters is
# still under development. For now, we use 
# multiprocessing to run graphs in parallel
# Threads share the same GIL and attempting to acquire
# the GIL while acquired panics the thread.
class Executor:
    def __init__(self):
        self.graphs = []
    def add(self, graph):
        next = graph.next()
        for (idx,(dt, graphs)) in enumerate(self.graphs):
            if dt == next:
                graphs.append(graph)
                return
            if dt < next:
                self.graphs.insert(idx, [graph])
                return

        self.graphs.append((next, [graph]))

    def start(self):
        while True:
            (next, graphs) = self.graphs.pop()

            now = datetime.now(timezone.utc)
            next = next.replace(tzinfo=timezone.utc)

            delta = next - now 

            time.sleep(delta.total_seconds())
            handlers = [Process(target=graph.start) for graph in graphs]

            [self.add(graph) for graph in graphs]
            [handler.start() for handler in handlers]
            [handler.join() for handler in handlers]
