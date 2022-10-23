# % python 0703_if_expr.py

def work():
    return 8

def compute_next_result():
    return 10

def compute_last_result():
    return 99

state = input("continue or not? ")
result = None
if (state == "continue"):
    stuff = work()
    result = compute_next_result() + stuff
else:
    result = compute_last_result()

print(F"result: {result}")