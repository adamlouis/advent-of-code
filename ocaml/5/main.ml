let read_file_string f =
  let ic = open_in f in
  let n = in_channel_length ic in
  let s = Bytes.create n in
  really_input ic s 0 n;
  close_in ic;
  (Bytes.to_string s)

let explode s =
    let rec explode_rec s result =
        let len = String.length s in
        match len with
        | 0 -> result
        | _ -> explode_rec (String.sub s 1 (len - 1)) (s.[0]::result)
    in
    explode_rec s []
;;

let implode l =
    let rec implode_rec l result =
        match l with
        | [] -> result
        | h::t ->  implode_rec t ((Char.escaped h) ^ result)
    in
    implode_rec l ""
;;

(* got part 1 with pure data strucutres (slow). part 2 with pure data structures is harder. will do later. *)

(* type 'a node = { value: 'a; next: 'a node option; previous: 'a node option};;
let a = { value=1; next=None; previous=None }

let to_nodes l =
    let rec to_nodes_rec l result =
        match l with
        | [] -> result
        | h::t -> to_nodes_rec t { value=h; next=None; previous=None; }
    in
    to_nodes_rec l 
;; *)



(* 
let should_destroy c1 c2 = 
    (c1 != c2) &&
    ((Char.lowercase_ascii c1 == c2) || (Char.lowercase_ascii c2 == c1))
;;

type destruction = { result : char list; removal : int };;

type partition = { front : char list; back: char list };;

(* TODO: make efficient *)
let partition_at l idx =
    let rec partition_at_rec l remaining front =
        match remaining with
        | 0 -> { front=List.rev front; back=l }
        | _ -> 
            let h = List.hd l in
            let t = List.tl l in
            partition_at_rec t (idx - 1) (h::front)
    in
    partition_at_rec l idx []
;;


(* destroy a single pair *)
let destroy_pair l start =
    let rec destroy_pair_rec l result idx =
        match l with
            [] -> None
            | h1::h2::tail -> if (should_destroy h1 h2)
                then Some { result=((List.rev result) @ tail); removal=idx }
                else destroy_pair_rec (h2::tail) (h1::result) (idx + 1)
            | h::[] -> None
    in
    let p = partition_at l start in
    let dst = destroy_pair_rec p.back [] 0 in
    match dst with
        | None -> None
        | Some dst -> Some { result=p.front @ dst.result; removal=start + dst.removal }
;;

let rec destroy_all l start =
    match destroy_pair l start with
    | None -> l
    | Some dst -> destroy_all dst.result dst.removal
;;

Printf.printf "%s\n" (implode (destroy_all (explode "dabAcCaCBAcCcaDA") 0));; *)

(* let data = read_file_string "./ocaml/5/data.txt";;
let chars = explode data;;

let units = destroy_all chars 0;;
Printf.printf "part=5 part_1=%d\n" (List.length units);; *)

(* let run_reaction s = 
    match explode s with
        [] -> ""
        | h1::h2::t -> "2"
        | h::t -> "1"
;; *)

(* Printf.printf "result: %s\n" (run_reaction "");
Printf.printf "result: %s\n" (run_reaction "1");
Printf.printf "result: %s\n" (run_reaction "22"); *)
(* Printf.printf "result: %s\n" (destroy_pair chars); *)

(* Printf.printf "%s\n" data;; *)
(*  *)
(* 
let v = 5;;

let sq y = y * y;;

sq 5;;
let y = sq v;;


let rec fib x = if x < 1 then 0 else if x == 1 then 1 else fib(x - 1) + fib(x - 2);;

let rec simple x = if x == 0 then 0 else simple 0;;

let result = simple 2;;

let mylist = [1;2;3;4;5]

let strlist = ["a";"s";"d";"f";"g"]

let rec sum l = match l with
[] -> 0
| h::t -> h + (sum t);;

(* type ('a, 'b) thing = 'a * 'b;; *)
type ('a,  thing = 'a * 'b;;

(* type ('a,'b) pair = {first: 'a; second: 'b};; *)
(* let x = {first=2; second="hello"};; *)

let z = duo 2 "hello";;



Printf.printf "sum %d\n" (sum mylist);;

Printf.printf "%d\n" y;;
Printf.printf "%d\n" result;;

Printf.printf "%d\n" (fib 1);;
Printf.printf "%d\n" (fib 2);;
Printf.printf "%d\n" (fib 3);;
Printf.printf "%d\n" (fib 4);;
Printf.printf "%d\n" (fib 5);;
Printf.printf "%d\n" (fib 6);;
Printf.printf "%d\n" (fib 7);;
Printf.printf "%d\n" (fib 8);;
Printf.printf "%d\n" (fib 9);;
 *)
