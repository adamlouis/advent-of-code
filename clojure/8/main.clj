(require '[clojure.repl :refer :all])
(require '[clojure.string :as str])

; 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
; HEADER = # CHILDREN | # METADATA | [CHILD NODES] | METADATA

; DATATYPES ----------------------------------------
(defn get-child-count [n] (get n 0))
(defn get-metadata-count [n] (get n 1))
(defrecord Node [metadata children strlen])

; BUILD TREE ----------------------------------------
(declare build-n-nodes build-first-node)

(defn build-n-nodes [v remaining result]
    (do
        (if (= remaining 0)
            result
            (do
                (def furst (build-first-node v))
                (recur (subvec v (:strlen furst)) (- remaining 1) (conj result furst))
            )
        )
    )
)

(defn build-first-node [v]
    (do
        (def meta_count (get-metadata-count v))
        (def child_count (get-child-count v))
        (def result (
            if (= child_count 0)
            (do
                (def strlen (+ 2 meta_count))
                (def metadata (subvec v 2 strlen))
                (def children [])
                (Node. metadata children strlen)
            )
            (do
                (def tail (subvec v 2))
                (def children (build-n-nodes tail child_count []))

                ; --------------------------                
                ; these get overwritten w/ mutual recusion. i don't understand why.
                (def meta_count (get-metadata-count v))
                (def child_count (get-child-count v))
                ; --------------------------

                (def lengths (mapv #(:strlen %) children))
                (def children_length (reduce + lengths))
                (def strlen (+ 2 (+ meta_count children_length)))
                
                (def metadata (subvec v (- strlen meta_count) strlen))

                (def self (subvec v 0 strlen))
                (Node. metadata children strlen)
            )
        ))
    result)
)

; PART 1 ----------------------------------------
(defn sum-all-metadata [l result]
    (do
        (
            if (= 0 (count l))
            result
            (do
                (def metadatas (flatten (mapv  #(:metadata %) l)))
                (def children (flatten (mapv  #(:children %) l)))
                (def sum (reduce + 0 metadatas))
                (recur children (+ result sum))
            )
        )
    )
)

; PART 2 ----------------------------------------

(defn sum-meta [n] (reduce + 0 (:metadata n)))
(defn sum-list [l] (reduce + 0 l))

(defn get-indexed-children [n] (do
    (defn in_bounds? [idx]
        (and
            (> idx -1)
            (< idx (count (:children n)))
        )
    )

    (def meta_idxs (mapv (fn [x] (- x 1)) (:metadata n)))
    (def valid_meta_idxs (filter in_bounds? meta_idxs))
    (def children (mapv (fn [x] (get (:children n) x)) valid_meta_idxs))

    children
))

(defn sum-all-values [l result]
    (
        if (= 0 (count l))
        result
        (do
            (defn children? [n] (> (count (:children n)) 0))
            (defn not_children? [n] (not (children? n)))

            (def with_children (filter children? l))
            (def without_children (filter not_children? l))

            (def sum_without_children (sum-list (mapv sum-meta without_children)))
            (def filtered_children (flatten (mapv get-indexed-children with_children)))
            (recur filtered_children (+ result sum_without_children))
        )
    )
)

(def data (slurp "./clojure/8/data.txt"))
(def fields (mapv #(Integer/parseInt %) (str/split data #" ")))

(def root (trampoline build-first-node fields))
(def part1 (sum-all-metadata [root] 0))
(def part2 (sum-all-values [root] 0))

(println "day=8 part_1=" part1 "[clojure]")
(println "day=8 part_2=" part2 "[clojure]")
