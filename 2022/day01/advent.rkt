#lang racket
(require 2htdp/batch-io)

(define inputFilename "input.txt")
(define testInputFilename "test_input.txt")

(define input (open-input-file inputFilename))
(define testInput (open-input-file testInputFilename))

;(define words (string-split (read-line input)))
;(define nums (map (lambda (s) (string->number s)) words))
(define words (read-words-and-numbers/line inputFilename))
(define nums (read-words-and-numbers/line inputFilename))
(define testWords (read-words-and-numbers/line testInputFilename))
(define testNums (read-words-and-numbers/line testInputFilename))

(provide words nums testWords testNums)
