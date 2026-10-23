#!/usr/bin/env python3

# Python is interpreted (like my cutie Bash or God-forgive-me JavaScript), meaning unlike C, Rust and Go, 
# it doesn't need to compile to run this file, but the runtime will be obviously slower than any compiled language
# Choosing any other interpreter other than CPython (be it PyPy or IronPython etc.) ain't do much for speed
# It's one of the reasons I import NumPy library by default when dealing with arrays ("lists")

# To run, just install python and run this file as is ( ./test1.py ), shebang above will do the job

#def main():
#
#    # There's no way to assign to variable a type, Python will proceed it dynamically
#    # To make variable empty, you assign None to it
#    a = b = None
#    print("Type two numbers separated by whitespace ('XX YY'): ")
#    a, b = input().split()
#    c = a < b
#
#    print(f"c = {c}")
#
#    if c:
#        print("First number is less than second")
#    else:
#        print("First number is bigger than or equal to second")

#def main():
#    # In case of Python, boolean values must be capitalized (True and False)
#    is_locked = True
#    can_edit = not(is_locked)
#
#    # In case of variables inside strings, you do print(f"Text {variables}") (note for "f" before double quotes)
#    print(f"is_locked = {is_locked}")
#    print(f"can_edit = {can_edit}")

#def main():
#    # Some Python operators are absolutely different from other languages
#    # || is or, && is and, ! is not
#    a = -2 > 5
#    b = 0 < 7
#    c = 0 == 0
#    d = a and b or c
#
#    print(f"a = {a}")
#    print(f"b = {b}")
#    print(f"c = {c}")
#    print(f"d = {d}")

#def main():
#    a = 19
#    b = 10
#    c = a < b
#
#    print(f"c = {c}")
#
#    print("First number is less than second.") if a < b else print("First number is bigger than or equal to second.")

#def main():
#    choice = input(f"1 for hello\n2 for bye\nChoose number: ")
#
#    match choice:
#        case "1":
#            print("Hello!")
#        case "2":
#            print("Bye!")
#        case _:
#            print("Why you didn't provide 1 or 2? :(")

#def main():
#    # Python doesn't have const function
#    # Another point, Python's bitwise XOR operator (which is tilde) works different
#    # It's basically a number multiplied by -1 instead of flipping zeros and ones
#    # So appropriate solution I've found was using ^ 0xFF operator for "true" flipping
#    a = 153
#    b = a ^ 0xFF
#
#    print(f"a = {a}, binary = {format(a, '#010b')}")
#    print(f"b = {b}, binary = {format(b, '#010b')}")

#if __name__ == "__main__":
#    main()
