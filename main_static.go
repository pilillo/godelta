package main

/*
#cgo LDFLAGS: ./lib/libhello.a -ldl
#include "./lib/hello.h"
*/
import "C"

func main() {
	//C.init_stuff()
	//C.hello(C.CString("John Smith"))
}
