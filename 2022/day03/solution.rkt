#lang racket

(define (priority item-type)
  ; Priority for the given item-type char
  ; a-z -> 1-26
  ; A-Z -> 27-52
  (let ([ord (char->integer item-type)]
        [ord-a (char->integer #\a)]
        [ord-z (char->integer #\z)]
        [ord-A (char->integer #\A)])
    (if (and (>= ord ord-a) (<= ord ord-z))
      (+ (- ord ord-a) 1)
      (+ (- ord ord-A) 27))))

(define (split-rucksack rucksack)
  ; Split the rucksack into two evenly sized compartments
  (let ([split-len (/ (length rucksack) 2)])
    (split-at rucksack split-len)))

(define (load-data path)
  (let ([lines (file->lines path)])
    (map string->list lines)))

(define input-path "input.txt")

(define rucksacks (load-data input-path))

; Part 1
(for/fold ([acc 0])
          ([rucksack (in-list rucksacks)])
  (let-values ([(comp1 comp2) (split-rucksack rucksack)])
    (let* ([comp1-set (list->set comp1)]
           [comp2-set (list->set comp2)]
           [comp-intersect (first (set->list (set-intersect comp1-set comp2-set)))])
      (+ acc (priority comp-intersect)))))

; Part 2
(define (chunk lst size)  ; Chunk a list into groups of size - unsafe if not evenly divisible
  (if (empty? lst)
    '()
    (cons (take lst size) (chunk (drop lst size) size))))

(define group-size 3)
(for/fold ([acc 0])
          ([sack-thruple (in-list (chunk rucksacks group-size))])
  (let* ([sack-set-thruple (map list->set sack-thruple)]  ; Convert the sack list of chars to char sets
         [intersection (apply set-intersect sack-set-thruple)]  ; Intersect the three sacks
         [item (first (set->list intersection))]  ; Take the only item
         [item-priority (priority item)])
    (+ acc item-priority)))
