#lang racket

(define (load-data path)
  (let ([lines (file->lines path)])
    (map string-split lines)))

(define winningMove (make-hash (list
  (cons "A" "Y")
  (cons "B" "Z")
  (cons "C" "X"))))

(define losingMove (make-hash (list
  (cons "A" "Z")
  (cons "B" "X")
  (cons "C" "Y"))))

(define drawingMove (make-hash (list
  (cons "A" "X")
  (cons "B" "Y")
  (cons "C" "Z"))))

(define movePoints (make-hash (list
  (cons "X" 1)
  (cons "Y" 2)
  (cons "Z" 3))))


(define winPoints 6)
(define drawPoints 3)
(define losePoints 0)

(define (scoreMove op-move my-move)
  (+
    (hash-ref movePoints my-move)
    (if (equal? my-move (hash-ref winningMove op-move))
      winPoints
      (if (equal? my-move (hash-ref losingMove op-move))
        losePoints
        drawPoints))))

;(define (playRPS rounds)
;  (define (playRPSRec rounds score)
;    (if (empty? rounds)
;      score
;      (let* ([round (first rounds)]
;             [op-move (first round)]
;             [my-move (last round)])
;        (begin
;          (printf "Current Score: ~a\nOpponent move: ~s, My move: ~s\n" score op-move my-move)
;          (let ([newScore (+ score (scoreMove op-move my-move))])
;            (playRPSRec (rest rounds) newScore))))))
;  (playRPSRec rounds 0))
(define (playRPS rounds)
  (for/fold ([score 0])
            ([round (in-list rounds)])
    (let* ([op-move (first round)]
           [my-move (last round)])
      (+ score (scoreMove op-move my-move)))))

(define input-file "test_input.txt")
(define data (load-data input-file))

; Part 1
(for/fold ([score 0])
          ([round (in-list data)])
  (let* ([op-move (first round)]
         [my-move (last round)])
    (+ score (scoreMove op-move my-move))))

; Part 2
(for/fold ([score 0])
          ([round (in-list data)])
  (let* ([op-move (first round)]
         [my-strat (last round)])
    (+ score (scoreMove op-move (cond
                                  [(equal? "X" my-strat) (hash-ref losingMove op-move)]
                                  [(equal? "Y" my-strat)(hash-ref drawingMove op-move)]
                                  [else (hash-ref winningMove op-move)])))))
