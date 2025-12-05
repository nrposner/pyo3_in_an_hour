from dunder_mifflin import Paper, Employee #ty: ignore[unresolved-import]

# dwight wants to make a Paper

def test_fax_1():
    try:
        v_important_paper = Paper(
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

        v_important_paper.fax(employees, emails)

        # now check that this paper is in the inboxes of all those 
        # emailed, and no others

        # should evaluate to [True, False, True, True, False]
        res = [v_important_paper.contents in employee.inbox() for employee in employees]
        if res == [True, False, True, True, False]:
            print("✅ Test #1 Succeeded!")
            print("What the heck is this? - Jim")
            return
        else:
            print("❌ Test #1 Failed: Perhaps the logic is incorrect...")
            print(f"\nDid nobody get my very important memo? - Michael")

    except Exception as e:
        print(f"❌ Test #1 Failed: Got an exception {type(e).__name__}, {e}.")
        print(f"\nDid nobody get my very important memo? - Michael")
        

# stanley tries to send something to himself... and then realizes the fax function defaults to sending to the entire office
def test_fax_2():
    try:
        stanleys_art = Paper(
            10, 12, 
            "[CENSORED]"
        )
        employees = [
            Employee(1, "Michael", "mscott@dundermifflin.com", "the best"), 
            Employee(0, "Dwight", "dschrute@dundermifflin.com", "horrible"),
            Employee(2, "Pam", "pbeesly@dundermifflin.com", "PAM!"),
            Employee(3, "Jim", "jhalpert@dundermifflin.com", "lazy"),
            Employee(4, "Stanley", "shudson@dundermifflin.com", "grouch"),
        ]

        stanleys_art.fax(employees)

        res = [stanleys_art.contents in employee.inbox() for employee in employees]

        if res == [True, True, True, True, True]:
            print("✅ Test #2 Succeeded!")
            print("That... was not supposed to go to everyone... - Stanley")
            return
        else:
            print(f"❌ Test #2 Failed: Perhaps the logic is incorrect...")
            print(f"\nWhat the... STAAAAAN?! - Michael")

    except Exception as e:
        print(f"❌ Test 2 Failed: Got an exception {type(e).__name__}, {e}.")
        print(f"\nWhat the... STAAAAAN?! - Michael")


if __name__ == "__main__":
    test_fax_1()
    test_fax_2()
