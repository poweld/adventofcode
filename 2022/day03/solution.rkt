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
  (let ([split-len (/ (length rucksack) 2)])
    (split-at rucksack split-len)))

(split-rucksack '(1 2 3 4 5 6))
;(split-at '(1 2 3 4 5 6) (/ (length )

;(priority #\a)
;(priority #\z)
;(priority #\A)
;(priority #\Z)
