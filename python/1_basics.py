# we want something super basic for the first part, just creating the pyclass
# Paper is a bit more involved, since we need to create a FromPython setup for the paper quality values
# maybe we'll make that an optional argument later on, when we're digging further into the weeds of manual Trait implementation?

import pytest
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
        print("Test #1\n✅ CORRECT:")
        print(f"Employee #1, Michael? I didn't realize this was a startup now. - Jim")
        print(f"I always identified with 0-indexing. - Dwight")
    except TypeError as te:
        print(f"Test #1\n❌ INCORRECT: Got a TypeError: {te}. \nConsider the valid input types of Employee.")
        print(f"\nWhy does our fax machine need to know all this? - Pam")
        print(f"It's AI, Pam, it knows everything! - Michael")
        # pytest.fail(f"\n❌ INCORRECT: Got a TypeError: {te}. \nConsider the valid input types of Employee.")

    except Exception as e:
        pytest.fail(f"Test #1\n❌ INCORRECT: Got an unexpected exception type {type(e).__name__}, {e}.")


if __name__ == "__main__":
    michael_employee_1()
