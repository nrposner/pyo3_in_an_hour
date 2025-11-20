# a series of tests for stage 1 of the workshop
# establishing the basics of modules and exports
import pytest
from dunder_mifflin import Paper #ty: ignore[unresolved-import]

# dwight wants to make a Paper
def test_paper_1():
    try:
        p = Paper(
            "bad", 
            6, 8, 
            "Michael sux"
        )
        print("\n✅ CORRECT:")
        print("Who wrote that!? - Michael")
    except Exception as e:
        print(f"\nCome on, stupid machine. Just print! - Dwight")
        pytest.fail(f"\n❌ INCORRECT: Expected ValueError, but got {type(e).__name__}.")
        
# Dwight is trying to make a paper, but wrong
def test_paper_2():
    try:
        # Attempt the action that should fail
        p = Paper("ba", 6, 8, "Michael sux")
    except ValueError:
        # 1. The code DID raise the error (Success path)
        print("\n✅ CORRECT:")
        print("What do you mean I can't misspell things? - Dwight")
        return # Test passes
    except Exception as e:
        # 2. The code raised the WRONG error
        print(f"\nWhy is the paper made of wool? How is this even possible? - Michael")
        pytest.fail()
        # pytest.fail(f"\n❌ INCORRECT: Expected ValueError, but got {type(e).__name__}.")
    
    # 3. The code raised NO error (Failure path)
    print(f"\nWhy is the paper made of wool? How is this even possible? - Michael")
    pytest.fail()
    # pytest.fail("\n❌ INCORRECT: The code accepted 'ba' as a valid size, but it should have failed.")




