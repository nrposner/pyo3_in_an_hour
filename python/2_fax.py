# a series of tests for stage 1 of the workshop
# establishing the basics of modules and exports
import pytest
from dunder_mifflin import Paper, Employee #ty: ignore[unresolved-import]

# dwight wants to make a Paper

def test_fax_1():
    import dunder_mifflin
    from dunder_mifflin import Paper, Employee #ty: ignore[unresolved-import]

    print(f"Module file: {dunder_mifflin.__file__}")
    print(f"Class is: {Employee}")

    try:
        v_important_paper = Paper(
            "supernice", 
            10, 12, 
            "When I was but a young lad, my father said to me..."
        )
        employees = [
            Employee(1, "Michael", "mscott@dundermifflin.com", "the best"), 
            Employee(0, "Dwight", "dschrute@dundermifflin.com", "horrible"),
            Employee(2, "Pam", "pbeesly@dundermifflin.com", "PAM!"),
            Employee(3, "Jim", "jhalpert@dundermifflin.com", "lazy"),
            Employee(4, "Stanley", "shudson@dundermifflin.com", "grouch"),
        ]
        emails = [
            "mscott@dundermifflin.com",
            "pbeesly@dundermifflin.com",
            "jhalpert@dundermifflin.com",
        ]

        # v_important_paper.fax(employees, emails)
        employees = v_important_paper.fax(employees, emails)

        # now check that this paper is in the inboxes of all those 
        # emailed, and no others

        # should evaluate to [True, False, True, True, False]
        res = [v_important_paper.contents in employee.inbox() for employee in employees]

        # todo: handle this issue of failing to mutate in place
        # current approach works and is acceptable, but ideally we should support
        # in-place mutation

        if res == [True, False, True, True, False]:
            print("\n✅ CORRECT:")
            print("What the heck is this? - Jim")
            return
        else:
            print(f"\nDid nobody get my very important memo? - Michael")
            pytest.fail(f"\n❌ INCORRECT: Perhaps the logic is incorrect...")

    except Exception as e:
        print(f"\nDid nobody get my very important memo? - Michael")
        pytest.fail(f"\n❌ INCORRECT: Got an exception {type(e).__name__}, {e}.")
        

# stanley tries to send something to himself... and then realizes the fax function defaults to sending to the entire office
def test_fax_2():
    try:
        v_important_paper = Paper(
            "supernice", 
            10, 12, 
            "When I was but a young lad, my father said to me..."
        )
        employees = [
            Employee(1, "Michael", "mscott@dundermifflin.com", "the best"), 
            Employee(0, "Dwight", "dschrute@dundermifflin.com", "horrible"),
            Employee(2, "Pam", "pbeesly@dundermifflin.com", "PAM!"),
            Employee(3, "Jim", "jhalpert@dundermifflin.com", "lazy"),
            Employee(4, "Stanley", "shudson@dundermifflin.com", "grouch"),
        ]
        employees = v_important_paper.fax(employees)

        res = [v_important_paper.contents in employee.inbox() for employee in employees]

        if res == [True, True, True, True, True]:
            print("\n✅ CORRECT:")
            print("That... was not supposed to go to everyone... - Stanley")
            return
        else:
            print(f"\nWhat the... STAAAAAN?! - Michael")
            pytest.fail(f"\n❌ INCORRECT: Perhaps the logic is incorrect...")

    except Exception as e:
        print(f"\nWhat the... STAAAAAN?! - Michael")
        pytest.fail(f"\n❌ INCORRECT: Got an exception {type(e).__name__}, {e}.")


if __name__ == "__main__":
    test_fax_1()
    test_fax_2()
