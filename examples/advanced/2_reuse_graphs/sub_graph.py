# Here we define a graph that will be
# executed by another graph

from tm import task, Graph
from email.utils import parseaddr

@task()
def validate_email(credentials):
    email = credentials.get("email")
    if email == None:
        return False

    if parseaddr(email) == ("",""):
        return False

    return True

@task()
def validate_password(credentials):
    password = credentials.get("password")
    if password == None:
        return False

    if len(password) < 8:
        return False

    return True

@task()
def collect(validate_email, validate_password):
    return validate_email and validate_password

# Notice how root tasks ask for configs,
# but we don't define it in Graph constructor
# we define it in graph call instead
validate = Graph(name="hello world", schedule="manual")

# add tasks to the graph
validate.add_edges([validate_email, validate_password], [collect])
