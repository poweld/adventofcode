#lang racket

(struct section-range (start end) #:transparent)

(define (string->section-range s)
  (apply section-range (map string->number (string-split s "-"))))
(define (string->section-range-pair s)
  (map string->section-range (string-split s ",")))

(define (load-data path)
  (let ([lines (file->lines path)])
    (map string->section-range-pair lines)))

(define input-path "input.txt")

; Part 1
(define (overlaps a b)
  (and
    (>= (section-range-start b) (section-range-start a))
    (<= (section-range-end b) (section-range-end a))))

(for/fold ([acc 0])
          ([srange-pair (in-list (load-data input-path))])
  (let ([a (first srange-pair)]
        [b (second srange-pair)])
    (if (or (overlaps a b) (overlaps b a))
      (add1 acc)  ; If a section range in a pair fully overlaps the other, increment count
      acc)))

; Part 2
(define (partially-overlaps a b)
  (or
    (and
      (>= (section-range-start b) (section-range-start a))
      (<= (section-range-start b) (section-range-end a)))
    (and
      (>= (section-range-end b) (section-range-start a))
      (<= (section-range-end b) (section-range-end a)))))

(for/fold ([acc 0])
          ([srange-pair (in-list (load-data input-path))])
  (let ([a (first srange-pair)]
        [b (second srange-pair)])
    (if (or (partially-overlaps a b) (partially-overlaps b a))
      (add1 acc)  ; If a section range in a pair partially overlaps the other, increment count
      acc)))
