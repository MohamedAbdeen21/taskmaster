from tm import task, Graph
from sub_graph import validate

@task()
def ask_user_input():
    email = input("Enter email: ")
    password = input("Enter password: ")
    
    # Note that subgraphs return the output of the last task executed
    # however, the sorting of the tasks is indeterministic 
    # so make sure your subgraphs only have a single leaf task
    valid = validate({"email": email, "password":password})

    if valid:
        print("Access granted")
    else:
        print("Access denied")

login = Graph(name="Login workflow", schedule="manual")
login.add_edges([ask_user_input])

login()
