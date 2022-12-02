#lang racket
(require "advent.rkt")

; (+ 1 1)
; (define input (open-input-file "input.txt"))
; (define words (string-split (read-line input)))
; (define nums (map (lambda (s) (string->number s)) words))
; nums
;nums
;
;(define ht (make-hash))
;(hash-set! ht "apple" '("red" "round"))
;(hash-set! ht "banana" '(yellow long))
;(hash-ref ht "apple")
;ht
;(define ht2 (make-hash (list '("apple" red round))))
;ht2
;(map string-upcase (hash-ref ht "apple"))

(define (group nums)
  (define (groupRec nums groups currentGroup)
    (if (empty? nums)
      groups
      (let ([head (first nums)]
            [tail (rest nums)])
        (if (empty? head)
          (groupRec tail (append (list (reverse currentGroup)) groups) '())
          (groupRec tail groups (append head currentGroup))))))
  (reverse (groupRec nums '() '())))

(define (sum nums)
  (foldr + 0 nums))

(define sums (map sum (group nums)))

(define (max nums)
  (last (sort nums <)))

(define (max3 nums)
  (take-right (sort nums <) 3))

(max sums)
(sum (max3 sums))
