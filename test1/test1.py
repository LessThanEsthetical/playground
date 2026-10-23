#!/usr/bin/env python3

# Python is interpreted (like my cutie Bash or God-forgive-me JavaScript), meaning unlike C, Rust and Go, 
# it doesn't need to compile to run this file, but the runtime will be obviously slower than any compiled language
# Choosing any other interpreter other than CPython (be it PyPy or IronPython etc.) ain't do much for speed
# It's one of the reasons I import NumPy library by default when dealing with arrays ("lists")

# To run, just install python and run this file as is ( ./test1.py ), shebang above will do the job

def main():

    # There's no way to assign to variable a type, Python will proceed it dynamically
    # To make variable empty, you assign None to it
    a = b = None
    print("Type two numbers separated by whitespace ('XX YY'): ")
    a, b = input().split()
    c = a < b

    print(f"c = {c}")

    if c:
        print("First number is less than second")
    else:
        print("First number is bigger than or equal to second")
