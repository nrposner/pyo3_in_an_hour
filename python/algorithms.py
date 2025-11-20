import dunder_mifflin as dm
from dm import Paper, Employee # ty: ignore[unresolved-import]

def dwights_ai_decision_algorithm(paper: Paper, employees: list[Employee]):
    if "DO NOT SHRED" in paper.contents:
        # we don't shred
        # we search for the names of employees in the paper
        employee_names = [employee.name for employee in employees]
        send_to = employee_names[employee_names in paper.contents]
        # and then send to all of them
        paper.fax(paper, send_to)
    else:
        # we shred 
        paper.shred()

def jims_ai_decision_algorithm(paper: Paper, employees: list[Employee]):
    if "DO NOT SHRED" in paper.contents:
        paper.shred()
    else:
        paper.shred()
