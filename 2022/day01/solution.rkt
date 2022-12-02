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
        (values acc (cons (string->number line) current-group))
        (values (cons (reverse current-group) acc) '())))))

(define input-file "input.txt")

(define sorted-sums
  (sort (map sum (load-data input-file)) <))

(last sorted-sums)
(sum (take-right sorted-sums 3))
