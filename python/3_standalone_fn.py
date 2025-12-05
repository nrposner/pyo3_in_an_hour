## in which we develop and test standalone pyfunctions

from dunder_mifflin import Paper, search_paper #ty: ignore[unresolved-import]


def michael_search_1():
    try:
        p = Paper(
            6, 8, 
            "Four score and seven years ago..."
        )

        results = search_paper(p, ["score", "Michael"])
        # print(results)

        assert(results == [True, False])

        print("✅ Test #1 Succeeded Correctly!")
        print("I think the employees look up to me as a leader - Michael")
    except AssertionError as ae:
        print(f"❌ Test #1 Failed: Received AssertionError{ae}. \nConsider checking the output of `result`")
        print(f"\nWhere... this isn't my speech... - Michael")
    except Exception as e:
        print(f"Got unexpected exception {e}")

def pam_search_2():
    try:
        p = Paper(
            6, 8, 
            "In the vaunted year of 1964, our brave co-founder Robert Dunder invented the dunder method."
        )

        results = search_paper(p, ["dunder", "Michael"], any=True)
        # print(results)

        assert(results == [True])

        print("✅Test #2 Succeeded Correctly!")
        print("Who in the world reads this stuff? - Pam")
    except AssertionError as ae:
        print(f"❌Test #2 Failed: Received AssertionError{ae}. \nConsider checking the output of `result`")
        print(f"\nCome on, work you stupid machine - Pam")
    except Exception as e:
        print(f"Got unexpected exception {e}")

if __name__ == "__main__":
    michael_search_1()
    pam_search_2()
