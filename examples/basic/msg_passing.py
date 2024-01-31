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

@task()
def root():
    return {"some_key": 0}

@task()
def add_2(root):
    msg = root['some_key']
    print(f"add_2 got {msg} and returned {msg + 2}")
    return {"new": msg + 2}

@task()
def add_3(root):
    msg = root['some_key']
    print(f"add_3 got {msg} and returned {msg + 3}")
    return {"new": msg + 3}

@task()
def do_nothing(**kwargs):
    msg = kwargs["root"]["some_key"]
    print(f"do_nothing got kwarg root {msg} and returned None")
    return None

@task()
def leaf(add_2, add_3, do_nothing):
    assert add_2["new"] == 2
    assert add_3["new"] == 3
    assert do_nothing == None
    print("Leaf received all inputs correctly")

graph = Graph(schedule="* * * * *")
graph.add_edges([root], [add_2, add_3, do_nothing])
graph.add_edges([add_2, add_3, do_nothing], [leaf])

exec = Executor()
exec.add(graph)
exec.start()
