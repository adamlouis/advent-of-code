#load "str.cma";;

(* require "str";; *)

(* read file as string *)
let read_file_string f =
  let ic = open_in f in
  let n = in_channel_length ic in
  let s = Bytes.create n in
  really_input ic s 0 n;
  close_in ic;
  (Bytes.to_string s)

(* string to char list *)
let explode s =
    let rec explode_rec s result =
        let len = String.length s in
        match len with
        | 0 -> result
        | _ -> explode_rec (String.sub s 1 (len - 1)) (s.[0]::result)
    in
    explode_rec s []
;;

(* char list to string *)
let implode l =
    let rec implode_rec l result =
        match l with
        | [] -> result
        | h::t ->  implode_rec t ((Char.escaped h) ^ result)
    in
    implode_rec l ""
;;

let should_destroy c1 c2 = 
    (c1 != c2) &&
    ((Char.lowercase_ascii c1 == c2) || (Char.lowercase_ascii c2 == c1))
;;

(* destroy a single pair *)
let destroy_one l =
    let rec destroy_one_rec l result =
        match l with
            [] -> None
            | h1::h2::tail -> if (should_destroy h1 h2)
                then Some ((List.rev result) @ tail)
                else destroy_one_rec (h2::tail) (h1::result)
            | h::[] -> None
    in
    destroy_one_rec l []
;;

let rec destroy_all l =
    match destroy_one l with
    | None -> l
    | Some destroyed -> destroy_all destroyed
;;

type 'a node = {mutable previous: 'a node option; value: 'a option; mutable next: 'a node option }

module type DEQUE =
  sig
    val prepend: 'a node -> 'a -> unit
    val remove: 'a node -> unit
  end

module MutableDeque : DEQUE =
  struct
    let prepend q v =
        let new_node = { previous=Some q; value=Some v; next=q.next } in
        q.next <- Some new_node;
        match new_node.next with Some nxt -> nxt.previous <- Some new_node; () | _ -> ();
        ()
    let remove q =
        (match q.next with
            Some nxt -> nxt.previous <- q.previous; ()
            | _ -> ()
        );
        (match q.previous with
            Some prv -> (prv.next <- q.next); ()
            | _ -> ()
        );
        q.next <- None;
        q.previous <- None;
        ()
  end

let build_dequeue c =
    let rec build_dequeue_rec l result =
        match l with
            | [] -> result
            | h::t ->
                MutableDeque.prepend result h;
                build_dequeue_rec t result
    in
    build_dequeue_rec c {previous=None; value=None; next=None }
;;

let build_dequeue s =
    let len = String.length s in
    let rec build_dequeue_rec idx result =
        if idx >= len
        then result
        else (
            MutableDeque.prepend result s.[idx];
            build_dequeue_rec (idx+1) result
        )
    in
    build_dequeue_rec 0 {previous=None; value=None; next=None }
;;

let printv vo = match vo with Some v -> Printf.printf "%c" v; () | _ -> () ;;

let print_deque (n: char node) =
    let rec print_deque_rec cur =
    match cur with
        | None -> Printf.printf ""; ()
        | Some cur_n ->
            printv cur_n.value;
            print_deque_rec cur_n.next;
            ()
    in
    print_deque_rec (Some n);
    Printf.printf "\n";
;;

let to_list (n: char node) =
    let rec to_list_rec cur_o result =
    match cur_o with
        | None -> result
        | Some cur ->
            match cur.value with
                | Some v -> to_list_rec cur.next (v::result)
                | None -> to_list_rec cur.next result
    in
    to_list_rec (Some n) []
;;

let should_destroy_op c1 c2 = 
    match (c1, c2) with
    | (Some v1, Some v2) -> should_destroy v1 v2
    | _ -> false
;;

let node_to_char n = match n.value with
    None -> '-'
    | Some c -> c
;;

let print_link (q: char node) = match (q.previous, q.value, q.next) with
    | (Some p, Some v, Some n) -> Printf.printf "%c -> %c -> %c\n" (node_to_char p) v (node_to_char n); ()
    | _ -> Printf.printf "unsuppported print\n"; ()
;;

let rec destroy_all_m s =
    let root = build_dequeue s in
    let rec destroy_all_m_rec current =
        match current.next with
        | None -> ()
        | Some next ->
            match should_destroy_op current.value next.value with
            | true ->
                let previous = current.previous in
                MutableDeque.remove current;
                MutableDeque.remove next;
                (match previous with
                    Some prv -> destroy_all_m_rec prv
                    | None -> ()
                )
            | false -> destroy_all_m_rec next
    in
    destroy_all_m_rec root;
    to_list root
;;

let min_item init = 
    let rec min_item_rec l min = 
        match l with
        | [] -> min
        | h::t -> (
            match min with
            | None -> min_item_rec t (Some h)
            | Some v -> min_item_rec t (Some (if v < h then v else h))
        )
    in
    min_item_rec init None
;;

(* READ INPUT -------------------------------------------------------------- *)
(* let data = "dabAcCaCBAcCcaDA";; *)
let data = read_file_string "./ocaml/5/data.txt";;


(* PURE -------------------------------------------------------------- *)
(* PART 1 *)
let part1_pure = List.length (destroy_all (explode data));;
Printf.printf "part=5 part_1=%d [ocaml][pure]\n" (part1_pure);;
Pervasives.flush_all();;

(* MUTABLE -------------------------------------------------------------- *)
(* PART 1 *)
let part1_mutable = List.length (destroy_all_m data);;
Printf.printf "part=5 part_1=%d [ocaml][mutable]\n" (part1_mutable);;
Pervasives.flush_all();;

(* PART 2 *)
let replacements = ["[Aa]"; "[Bb]"; "[Cc]"; "[Dd]"; "[Ee]"; "[Ff]"; "[Gg]"; "[Hh]"; "[Ii]"; "[Jj]"; "[Kk]"; "[Ll]"; "[Mm]"; "[Nn]"; "[Oo]"; "[Pp]"; "[Qq]"; "[Rr]"; "[Ss]"; "[Tt]"; "[Uu]"; "[Vv]"; "[Ww]"; "[Xx]"; "[Yy]"; "[Zz]"];;
let replace exp target = (Str.global_replace (Str.regexp exp) "" target);;
let candidates = List.map (fun exp -> replace exp data) replacements;;

let part2_mutable = min_item (List.map (fun c -> List.length (destroy_all_m c) ) candidates);;

let v = match part2_mutable with
| Some v -> Printf.printf "part=5 part_2=%d [ocaml][mutable]\n" v; ()
| None -> Printf.printf "part=5 part_2=no solution [ocaml][mutable]\n"; ()
;;
Pervasives.flush_all();;
