from dunder_mifflin import Paper, Employee, dwights_algo, jims_algo #ty: ignore[unresolved-import]

def dwights_algo_1():
    try:
        paper = Paper(6, 8, "PLEASE DO NOT SHRED THIS PAPER, MICHAEL")
        employees = [
            Employee(1, "Michael", "mscott@dundermifflin.com", "the best"), 
            Employee(0, "Dwight", "dschrute@dundermifflin.com", "horrible"),
            Employee(2, "Pam", "pbeesly@dundermifflin.com", "PAM!"),
            Employee(3, "Jim", "jhalpert@dundermifflin.com", "lazy"),
            Employee(4, "Stanley", "shudson@dundermifflin.com", "grouch"),
        ]
        dwights_algo(paper, employees)

        res = [paper.contents in employee.inbox() for employee in employees]

        if res == [True, False, False, False, False]:
            print("✅ Test #1 Succeeded!")
            print("Fine, jeez, whatever - Dwight")
            return
        else:
            print(f"❌ Test #1 Failed unexpectedly. Result was expected to be [True, False, False, False, False], was found to instead be {res}.")
            print(f"\nWhy did we mix up a fax machine with a shredder?! Whose idea was that?! - Michael")

    except Exception as e:
        print(f"Test #1\n❌ Received Exception {e}.")


def dwights_algo_2():
    try:
        paper = Paper(6, 8, "Hi Michael, please see the quarterly reports attached below: yours, Pam")
        employees = [
            Employee(1, "Michael", "mscott@dundermifflin.com", "the best"), 
            Employee(0, "Dwight", "dschrute@dundermifflin.com", "horrible"),
            Employee(2, "Pam", "pbeesly@dundermifflin.com", "PAM!"),
            Employee(3, "Jim", "jhalpert@dundermifflin.com", "lazy"),
            Employee(4, "Stanley", "shudson@dundermifflin.com", "grouch"),
        ]
        dwights_algo(paper, employees)

        res = [paper.contents in employee.inbox() for employee in employees]

        if res == [False, False, False, False, False]:
            print("✅ Test #2 Succeeded!")
            print("What? You didn't say not to shred it - Dwight")
            return
        else:
            print(f"❌ Test #2 Failed unexpectedly. Result was expected to be [False, False, False, False, False], was found to instead be {res}.")
            print(f"\nReally, Dwight? Really? - Michael")

    except Exception as e:
        print(f"❌ Test #2 Failed: Received Exception {e}.")

def jims_algo_3():
    try:
        paper = Paper(6, 8, "PLEASE DO NOT SHRED THIS PAPER, MICHAEL")
        employees = [
            Employee(1, "Michael", "mscott@dundermifflin.com", "the best"), 
            Employee(0, "Dwight", "dschrute@dundermifflin.com", "horrible"),
            Employee(2, "Pam", "pbeesly@dundermifflin.com", "PAM!"),
            Employee(3, "Jim", "jhalpert@dundermifflin.com", "lazy"),
            Employee(4, "Stanley", "shudson@dundermifflin.com", "grouch"),
        ]
        jims_algo(paper, employees)

        res = [paper.contents in employee.inbox() for employee in employees]

        if res == [False, False, False, False, False]:
            print("✅ Test #3 Succeeded!")
            print("Can't give us more work if the work isn't delivered - Jim")
            return
        else:
            print(f"❌Test #3 Failed unexpectedly. Result was expected to be [False, False, False, False, False], was found to instead be {res}.")
            print(f"\nWhere did this week's work assignments go? - Michael")

    except Exception as e:
        print(f"❌Test #3 Failed: Received Exception {e}.")

if __name__ == "__main__":
    dwights_algo_1()
    dwights_algo_2()
    jims_algo_3()
