# Creating DAGs and passing messages

from tm import task, Graph, Executor

#            root
#           /  |  \
#          0   0   0
#         /    |    \
#      add_2 add_3 do_nothing
#        \     |     /
#         2    3   None
#          \   |   /
#           \  |  /
#            leaf

# Root can take an argument as demonstrated in the upcoming example 5

# Messages must be of type Option[Dict]
@task()
def root():
    return {"some_key": 0}

# Message from parent is passed as a keyword argument
# With parent name as a the argument name
@task()
def add_2(root):
    msg = root['some_key']
    print(f"add_2 got {msg} and returned {msg + 2}")
    return {"new": msg + 2}

# tasks can have multiple children
@task()
def add_3(root):
    msg = root['some_key']
    print(f"add_3 got {msg} and returned {msg + 3}")
    return {"new": msg + 3}

# you can use **kwargs to ignore input or accept generic arguments
@task()
def do_nothing(**kwargs):
    msg = kwargs["root"]["some_key"]
    print(f"do_nothing got kwarg root {msg} and returned None")
    return None # Tasks must return Optional[Dict]

# tasks can have multiple parents as well
@task()
def leaf(add_2, add_3, do_nothing):
    assert add_2["new"] == 2
    assert add_3["new"] == 3
    assert do_nothing == None
    print("Leaf received all inputs correctly")

# create the graph
graph = Graph(name="message passing demo", schedule="* * * * *")

# Define the nodes of the graph

# Single parent, multiple children
graph.add_edges([root], [add_2, add_3, do_nothing])

# Multiple parents, single child
graph.add_edges([add_2, add_3, do_nothing], [leaf])

# Creating executors
exec = Executor()

# registering graphs
exec.add(graph)

# start executor
exec.start()
