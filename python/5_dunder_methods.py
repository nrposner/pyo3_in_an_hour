from dunder_mifflin import Paper #ty: ignore[unresolved-import]

def len_test():
    try:
        paper = Paper(6, 8, "Would I rather be feared or loved? Easy. Both. I want people to be afraid of how much they love me.")

        assert(len(paper)==21)

        print(f"✅ Test #1 Succeeded")
        print("This is not what I planned to do with my life. - Stanley")
    except Exception as e:
        print(f"❌ Test #1 Failed: Received unexpected exception {e}")

def contains_test():
    try:
        paper = Paper(6, 8, "Would I rather be feared or loved? Easy. Both. I want people to be afraid of how much they love me.")

        assert("love" in paper)

        print(f"✅ Test #2 Succeeded!")
        print("I'm going for a smoke - Pam")
    except Exception as e:
        print(f"❌ Test #2 Failed: Received unexpected exception {e}")


def getitem_test_1():
    try:
        paper = Paper(6, 8, "Would I rather be feared or loved? Easy. Both. I want people to be afraid of how much they love me.")

        assert(paper[4]=="feared")

        print(f"✅ Test #3 Succeeded")
        print("Fear will keep them in line. Fear, and donuts - Michael")
    except Exception as e:
        print(f"❌Test #3 Failed: Received unexpected exception {e}")

def getitem_test_2():
    try:
        paper = Paper(6, 8, "Would I rather be feared or loved? Easy. Both. I want people to be afraid of how much they love me.")

        assert(paper[-2]=="love")

        print(f"✅ Test #4 Succeeded")
        print("I always related to Machiavelli - Dwight")
    except Exception as e:
        print(f"❌Test #4 Failed: Received unexpected exception {e}")
        print(e)
if __name__ == "__main__":
    len_test()
    contains_test()
    getitem_test_1()
    getitem_test_2()
