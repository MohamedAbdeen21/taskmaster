import os
import time
import signal
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
        self.handlers = []
        self.pid = os.getpid()
        signal.signal(signal.SIGINT, self.wait)

    def add(self, graph):
        next = graph.next()
        for (idx,(dt, graphs)) in enumerate(self.graphs):
            if dt == next:
                graphs.append(graph)
                return
            if dt < next:
                self.graphs.insert(idx, (next,[graph]))
                return

        self.graphs.append((next, [graph]))

    def wait(self, signum, frame):
        # Signals and handlers are passed to children by default
        # Ignore in children
        if self.pid != os.getpid():
            return

        while any([handler.is_alive() for handler in self.handlers]):
            time.sleep(1)

        exit()


    def start(self):
        while True:
            (next, graphs) = self.graphs.pop()

            now = datetime.now(timezone.utc)
            next = next.replace(tzinfo=timezone.utc)

            delta = next - now 

            # time.sleep(max(0, delta.total_seconds()))
            time.sleep(5)
            self.handlers = [Process(target=graph.start) for graph in graphs]

            [self.add(graph) for graph in graphs]
            [handler.start() for handler in self.handlers]
            [handler.join() for handler in self.handlers]
            self.handlers.clear()
