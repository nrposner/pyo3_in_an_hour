# a series of tests for stage 2 of the workshop
# establishing the basics of modules and exports
from dunder_mifflin import Paper #ty: ignore[unresolved-import]

# basics
# dwight wants to make a Paper
# to succeed on this test, it is only necessary that we build the paper without a type error occuring
def jim_paper_1():
    try:
        p = Paper(
            6, 8, 
            "Michael sux"
        )
        print("Test #1 Succeeded Correctly\n✅")
        print("Who wrote that!? - Michael")
    except ValueError as ve:
        print(f"❌ Test #1 Failed: Received ValueError{ve}. \nConsider checking the valid input types of Paper")
        print(f"\nWhy isn't this working? - Jim")
    except TypeError as te:
        print(f"❌ Test #1 Failed: Received ValueError{te}. \nConsider checking the valid input types of Paper")
        print(f"\nWhat do you mean 'Paper doesn't exist?! - Jim")
    except Exception as e:
        print(f"❌ Test #1 Failed: Got unexpected error type {type(e).__name__}. \nConsider asking for help")
        print(f"\nCome on, stupid machine. Just print! - Jim")
        
# Dwight is trying to make a paper, but wrong... or is he?
# This is a judgment call: you could accept negative dimensions as inputs and coerce them into positive dimensions... or you could leave this as an error and try passing back a more informative error message
# for the time being, we'll stick to the 'no negative widths' option, since ergonomically raising a warning while still returning an Ok() value in pyo3 requires external logging crates
def dwight_paper_2():
    try:
        # should fail
        p = Paper(-6, 8, "NEW OFFICE RULES")

        print(f"✅ Test #2 Succeeded... Correctly?")
        print("See? I told you it would work! - Dwight")
    except OverflowError as oe:
        print(f"❌ Test #2 Succeeded... Incorrectly? {oe}")
        print("It's negative width, so it should just be wide to the left! What's so hard about this? - Dwight")
    except ValueError as ve:
        print(f"✅ Test #2 Failed... Correctly? {ve}")
        print("Maybe you should listen to the computer, Dwight. - Michael")
    except Exception as e:
        print(f"❌ Test #2 Failed Incorrectly! Expected OverflowError or Warning, but got {type(e).__name__}.")
        print(f"\nDwight, why is our fax machine segfaulting? - Michael")


if __name__ == "__main__":
    jim_paper_1()
    dwight_paper_2()
