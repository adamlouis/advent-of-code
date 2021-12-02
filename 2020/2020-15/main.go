package main

import "fmt"

func next(a []int) int {
	last := a[len(a)-1]

	for i := len(a) - 2; i >= 0; i-- {
		if a[i] == last {
			return len(a) - 1 - i
		}
	}

	return 0
}

func nextfast(last, count int, lastidx map[int]int) int {
	v, ok := lastidx[last]
	if !ok {
		return 0
	}
	return count - v - 1
}

func main() {
	// part 1
	a := []int{15, 12, 0, 14, 3, 1}
	for len(a) < 2020 {
		a = append(a, next(a))
	}
	fmt.Println(a[len(a)-1])

	fmt.Println("-----")
	// part 2
	a = []int{15, 12, 0, 14, 3, 1}
	// a = []int{0, 3, 6}
	actual := []int{}

	idx := map[int]int{}
	for i, v := range a {
		if i == len(a)-1 {
			break
		}
		actual = append(actual, v)
		idx[v] = i
	}

	last := a[len(a)-1]
	actual = append(actual, last)

	for len(actual) < 30_000_000 {
		count := len(actual)
		next := nextfast(last, count, idx)
		actual = append(actual, next)
		idx[last] = count - 1
		last = next
	}

	fmt.Println(actual[len(actual)-1])

}
