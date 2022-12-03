#lang racket

;(char->integer #\A)
;(define priority-ht (make-hash (list

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

;(load-data input-path)
(for/fold ([acc 0])
          ([rucksack (in-list rucksacks)])
  (let-values ([(comp1 comp2) (split-rucksack rucksack)])
    (let* ([comp1-set (list->set comp1)]
           [comp2-set (list->set comp2)]
           [comp-intersect (first (set->list (set-intersect comp1-set comp2-set)))])
      (begin
        (printf "Rucksack: ~s\n" rucksacks)
        (printf "Compartment 1: ~a\n" comp1)
        (printf "Compartment 1 Set: ~a\n" comp1-set)
        (printf "Compartment 2: ~a\n" comp2)
        (printf "Compartment 2 Set: ~a\n" comp2-set)
        (printf "Misplaced item type: ~a, priority: ~a\n" comp-intersect (priority comp-intersect))
        (+ acc (priority comp-intersect))))))


;(split-rucksack '(1 2 3 4 5 6))
;(split-at '(1 2 3 4 5 6) (/ (length )

;(priority #\a)
;(priority #\z)
;(priority #\A)
;(priority #\Z)
