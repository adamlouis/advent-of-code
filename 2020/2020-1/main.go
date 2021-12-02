package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {

	bytes, err := os.ReadFile("./in.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.TrimSpace(string(bytes)), "\n")
	nmap := map[int]bool{}
	ns := []int{}

	for _, l := range lines {
		v, err := strconv.Atoi(l)
		if err != nil {
			panic(err)
		}
		if nmap[2020-v] {
			fmt.Println("2:", v*(2020-v))
		}
		nmap[v] = true
		ns = append(ns, v)
	}

	for i := 0; i < len(ns); i++ {
		for j := i + 1; j < len(ns); j++ {
			for k := j + 1; k < len(ns); k++ {
				if ns[i]+ns[j]+ns[k] == 2020 {
					fmt.Println("3:", ns[i]*ns[j]*ns[k])
				}
			}
		}
	}

}
