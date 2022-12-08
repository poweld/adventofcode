#lang racket

(define (load-data path)
  (let ([lines (file->lines path)])
    (map string-split lines)))

(define move-win (make-hash (list
  (cons "A" "Y")
  (cons "B" "Z")
  (cons "C" "X"))))

(define move-lose (make-hash (list
  (cons "A" "Z")
  (cons "B" "X")
  (cons "C" "Y"))))

(define move-draw (make-hash (list
  (cons "A" "X")
  (cons "B" "Y")
  (cons "C" "Z"))))

(define points-move (make-hash (list
  (cons "X" 1)
  (cons "Y" 2)
  (cons "Z" 3))))


(define-values
  (points-lose points-draw points-win)
  (values 0 3 6))

(define (scoreMove op-move my-move)
  (+ (hash-ref points-move my-move)
     (cond
       [(equal? my-move (hash-ref move-win op-move)) points-win]
       [(equal? my-move (hash-ref move-lose op-move)) points-lose]
       [else points-draw])))

(define input-file "input.txt")
(define data (load-data input-file))

; Part 1
(for/fold ([score 0])
          ([round (in-list data)])
  (let* ([op-move (first round)]
         [my-move (last round)])
    (+ score (scoreMove op-move my-move))))

; Part 2
(define (strat-move strat op-move)
  (case strat
    [("X") (hash-ref move-lose op-move)]
    [("Y") (hash-ref move-draw op-move)]
    [("Z") (hash-ref move-win op-move)]))

(for/fold ([score 0])
          ([round (in-list data)])
  (let* ([op-move (first round)]
         [my-strat (last round)])
    (+ score
      (scoreMove op-move (strat-move my-strat op-move)))))
