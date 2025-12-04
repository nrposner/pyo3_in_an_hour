from dunder_mifflin import Paper #ty: ignore[unresolved-import]

def len_test():
    try:
        paper = Paper(6, 8, "Would I rather be feared or loved? Easy. Both. I want people to be afraid of how much they love me.")

        assert(len(paper)==21)

        print(f"\nTest #1 Succeeded\n ✅")
        print("See? I told you it would work! - Dwight")
    except Exception as e:
        print("\nTest #1 Failed\n ❌")
        print(e)

def contains_test():
    try:
        paper = Paper(6, 8, "Would I rather be feared or loved? Easy. Both. I want people to be afraid of how much they love me.")

        assert("love" in paper)

        print(f"\nTest #2 Succeeded!\n ✅")
        print("See? I told you it would work! - Dwight")
    except Exception as e:
        print("\nTest #2 Failed\n ❌")
        print(e)


def getitem_test_1():
    try:
        paper = Paper(6, 8, "Would I rather be feared or loved? Easy. Both. I want people to be afraid of how much they love me.")

        assert(paper[4]=="feared")
        assert(paper[-2]=="love")

        print(f"\nTest #3 Succeeded\n ✅")
        print("See? I told you it would work! - Dwight")
    except Exception as e:
        print("\nTest #3 Failed\n ❌")
        print(e)

def getitem_test_2():
    try:
        paper = Paper(6, 8, "Would I rather be feared or loved? Easy. Both. I want people to be afraid of how much they love me.")

        assert(paper[-2]=="love")

        print(f"\nTest #4 Succeeded\n ✅")
        print("See? I told you it would work! - Dwight")
    except Exception as e:
        print("\nTest #3 Failed\n ❌")
        print(e)
if __name__ == "__main__":
    len_test()
    contains_test()
    getitem_test_1()
    getitem_test_2()
