// Implementation in Go of test1.c originally written in C
// I am still biased to Rust lol
// But both seems good as modern languages
// Still, C wins when it comes to maturity regarding concepts IMO

// To compile, install Go and run "go build ./test1.go"

// Similar to #include <stdio.h>
package main
import "fmt"

// One thing to note: semicolons ";" aren't required if text ends in newline
//func main() {
//    // still can be dynamically typed, but I'll keep int here
//    var a, b int
//    fmt.Print("Type two numbers separated by whitespace ('XX YY'): ")
//    fmt.Scanln(&a, &b)
//    var c bool = a < b
//
//    fmt.Printf("c = %v\n", c)
//
//    if c {
//	  fmt.Println("First number is less than second")
//    } else {
//        fmt.Println("First number is bigger than or equal to second")
//    }
//}

//func main() {
//    var is_locked = true
//    var can_edit = !is_locked
//
//    // Unless the print function includes ln at the end, \n is necessary
//    fmt.Printf("is_locked = %v\n", is_locked)
//    fmt.Printf("can_edit = %v\n", can_edit)
//}

//func main() {
//
//    a := -2 > 5
//    b := 0 < 7
//    c := 0 == 0
//    d := a && b || c
//
//    // There's no casting between bool and int, so it'll print true/false
//    // Shall you want to make it print 1/0, you have to write special function outside of main()
//    fmt.Printf("a = %v\n", a)
//    fmt.Printf("b = %v\n", b)
//    fmt.Printf("c = %v\n", c)
//    fmt.Printf("d = %v\n", d)
//}

//func main() {
//    a := 19
//    b := 10
//    c := a < b
//
//    fmt.Printf("c = %v\n", c)
//
//    // As in Rust, there's no ternary operations akin to Python or C
//    // But one line if else still possbile
//    // Not advised, it makes code uglier and unreadable
//    if a < b { fmt.Println("First number is less than second") } else { fmt.Println("First number is bigger than or equal to second") }
//}

//func main() {
//    var choice int
//    fmt.Println("1 for hello\n2 for bye\nChoose number: ")
//    fmt.Scanf("%d", &choice)
//
//    switch choice {
//        case 1:
//	    fmt.Println("Hello!")
//	case 2:
//	    fmt.Println("Bye!")
//	default:
//	    fmt.Println("Why you didn't provide 1 or 2? :(")
//    }
//}

//func main() {
//    // As for C tilde "~" equivalent in Go, it'll be ^
//    const a uint8 = 153
//    const b uint8 = ^a
//
//    // You use %b to print binary
//    // Or even %#b to print binary with prefix 0b
//    fmt.Printf("a = %v, binary = %#08b\n", a, a)
//    fmt.Printf("b = %v, binary = %#08b\n", b, b)
//}
