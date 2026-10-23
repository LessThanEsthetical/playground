// Implementation in Go of test1.c originally written in C
// I am still biased to Rust lol
// But both seems good as modern languages
// Still, C wins when it comes to maturity regarding concepts IMO

// To compile, install Go and run "go build ./test1.go"

// Similar to #include <stdio.h>
package main
import "fmt"

// One thing to note: semicolons ";" aren't required if text ends in newline
func main() {
    // still can be dynamically typed, but I'll keep int here
    var a, b int
    fmt.Print("Type two numbers separated by whitespace ('XX YY'): ")
    fmt.Scanln(&a, &b)
    var c bool = a < b

    fmt.Println("c =", c)

    if c {
	fmt.Println("First number is less than second")
    } else {
        fmt.Println("First number is bigger than or equal to second")
    }
}
