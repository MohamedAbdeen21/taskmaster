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

# Root task can take an argument as demonstrated in the upcoming example 5

# Messages can be any python object
@task()
def root():
    return 0

# Message from parent is passed as a keyword argument
# With parent name as a the argument name
# In this example, add_2 is called with kwarg root=0
@task()
def add_2(root):
    print(f"add_2 got {root} and returned {root + 2}")
    return root + 2

# tasks can have multiple children
@task()
def add_3(root):
    print(f"add_3 got {root} and returned {root + 3}")
    return root + 3

# you can use **kwargs to ignore input or handle the arguments yourself
@task()
def do_nothing(**kwargs):
    root = kwargs["root"]
    print(f"do_nothing got kwarg root {root} and returned None")
    return None  # tasks can also return None

# tasks can have multiple parents as well
@task()
def leaf(add_2, add_3, do_nothing):
    assert add_2 == 2
    assert add_3 == 3
    assert do_nothing is None
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
