(require '[clojure.repl :refer :all])
(require '[clojure.string :as str])

; 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
; HEADER = # CHILDREN | # METADATA | [CHILD NODES] | METADATA

(def data (slurp "./clojure/8/data.txt"))
(def fields (mapv #(Integer/parseInt %) (str/split data #" ")))

(println data)
(println "Hello World")
(println (str "Hello World"))
(println (str fields))

(defn get-child-count [n] (get n 0))
(defn get-metadata-count [n] (get n 1))

; (defn sum-meta [n] (do
;     (def len (count n))
;     (def meta_count (get-metadata-count n))
;     (reduce + (subvec n (- len meta_count) len) 0)
; ))

(defrecord Node [metadata children])

(declare get-first)

(defn get-n [n remaining result]
    (do
        ; (println "CALL:get-n:" remaining n)
        (if (= remaining 0)
            result
            (do
                (def furst (get-first n))
                ; (println "RESULT:get-first:" remaining furst)
                ; result
                ; (recur n (- remaining 1) result)
                (recur (subvec n (count furst)) (- remaining 1) (conj result furst))
            )
        )
    )
)

(defn get-first [n]
    (do
        (println "CALL:get-first:" n)
        (def meta_count (get-metadata-count n))
        (def child_count (get-child-count n))
        (def result (
            if (= child_count 0)
            (Node. (subvec n 2 (+2 meta_count)) [])
            (do
                (def tail (subvec n 2))
                (def children (get-n tail child_count []))
                (println "CHILDREN" children)
                ; (def lengths [4])
                ; (def children_length (reduce + lengths 0))
                (def children_length 5)
                (def self_length (+ 2 (+ meta_count children_length)))
                
                (println "FLAT" (flatten children))
                ; (println "C_FLAT" (count (get children 0)))
                ; (println "COUNT:" meta_count (count (flatten children)))

                (def self (subvec n 0 self_length))
                [self children]
            )
        ))
    result)
)

(defn unnest [v, result]
    (do
        (println v)
        (
            if (vector? (get v 0))
            (recur (get v 1) (conj result (get v 0)))
            (if v (conj result v) result)
        )
    )
)
; (defn get-node-count [n, result]
;     (do
;         (println v)
;         (
;             if (vector? (get v 0))
;             (recur (get v 1) (conj result (get v 0)))
;             (if v (conj result v) result)
;         )
;     )
; )







(println (get-child-count fields))
(println (get-metadata-count fields))
; (println (get-node-length fields))
(def root (get-first fields))
(def nodes (unnest root []))
; (def part1 (sum-meta nodes))

(println "root = " root)
(println "nodes = " nodes)
; (println "part1 = " part1)


; (defn get-child-count [start])