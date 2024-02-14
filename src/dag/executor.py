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
    def __init__(self, graphs = []):
        self.graphs = []
        self.active_handlers = []
        self.pid = os.getpid()
        self.caught = False
        signal.signal(signal.SIGINT, self.wait)

        for graph in graphs:
            self.add(graph)

    def add(self, graph):
        if graph.is_manual():
            raise TypeError(f"Graph {graph.name()} has a manual schedule")

        graph.commit()
        self.schedule(graph)

    def schedule(self, graph):
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
        _ = signum, frame # silence unused variable error

        # Signals and handlers are passed to children by default
        # Ignore in children
        if self.pid != os.getpid():
            return

        if self.caught:
            print("\nForcing shutdown ..")
            [handler.kill() for handler in self.active_handlers]
            exit(1)

        self.caught = True
        print("\nCaught an interrupt signal, waiting for graphs to finish")
        while any([handler.is_alive() for handler in self.active_handlers]):
            time.sleep(2)

        exit(0)


    def start(self):
        while True:
            (next, graphs) = self.graphs.pop()

            now = datetime.now(timezone.utc)
            next = next.replace(tzinfo=timezone.utc)
            delta = next - now 

            # time.sleep(max(0, delta.total_seconds()))
            time.sleep(5)

            handlers = [Process(target=graph) for graph in graphs]

            [self.schedule(graph) for graph in graphs]
            [handler.start() for handler in handlers]

            self.active_handlers += handlers
            self.active_handlers = list(filter(lambda h: h.is_alive(), self.active_handlers))

