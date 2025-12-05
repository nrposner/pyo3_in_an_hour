from dunder_mifflin import Employee #ty: ignore[unresolved-import]

def michael_employee_1():
    try:
        employees = [
            Employee(1, "Michael", "mscott@dundermifflin.com", "the best"), 
            Employee(0, "Dwight", "dschrute@dundermifflin.com", "horrible"),
            Employee(2, "Pam", "pbeesly@dundermifflin.com", "PAM!"),
            Employee(3, "Jim", "jhalpert@dundermifflin.com", "lazy"),
            Employee(4, "Stanley", "shudson@dundermifflin.com", "grouch"),
        ]
        print("✅ Test #1 Succeeded!")
        print(f"Employee #1, Michael? I didn't realize this was a startup now. - Jim")
        print(f"I always identified with 0-indexing. - Dwight")
    except TypeError as te:
        print(f"❌ Test #1 Failed: Got a TypeError: {te}. \nConsider the valid input types of Employee.")
        print(f"\nWhy does our fax machine need to know all this? - Pam")
        print(f"It's AI, Pam, it knows everything! - Michael")

    except Exception as e:
        print(f"❌ Test #1 Failed: Got an unexpected exception type {type(e).__name__}, {e}.")


if __name__ == "__main__":
    michael_employee_1()
