#lang racket

(define (sum nums)
  (foldr + 0 nums))

(define (load-data path)
  (let ([lines (file->lines path)])
    (for/fold ([acc '()]
               [current-group '()]
               #:result (reverse acc))
              ([line (in-list lines)])
      (if (non-empty-string? line)
        ; Convert string to number and add to current group
        (values acc (cons (string->number line) current-group))
        ; Empty string - add current group to acc
        (values (cons (reverse current-group) acc) '())))))

(define input-file "input.txt")

(define sorted-sums
  (sort (map sum (load-data input-file)) <))

; Part 1
(last sorted-sums)
; Part 2
(sum (take-right sorted-sums 3))
