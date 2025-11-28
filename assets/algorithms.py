from dunder_mifflin import Paper, Employee # ty: ignore[unresolved-import]

def dwights_ai_decision_algorithm(paper: Paper, employees: list[Employee]) -> None | list[str]:
    try: 
        if "do not shred" in paper.contents.lower():
            # we search for the names of employees in the paper
            employee_names = [employee.name().lower() for employee in employees]
            # and then send to all those mentioned

            present = [name in paper.contents.lower() for name in employee_names]

            # present_1 = employee_names in paper.contents.lower()
            # # present = employees[employee_names in paper.contents.lower()]
            # return [employee.email() for employee in present]
            return [employee.email() for indx,employee in enumerate(employees) if present[indx]]
        else:
            return []
    except Exception as e:
        print(f"Got unexpected Exception {e}")


# not the same signature

def jims_ai_decision_algorithm(paper: Paper, employees: list[Employee]) -> None | list[str]:
    try: 
        if "DO NOT SHRED" in paper.contents:
            return []
        else:
            return []
    except Exception as e:
        print(f"Got unexpected Exception {e}")
