#lang racket

; (define (push x stack)
;   (if (null? x)
;     stack
;     (cons x stack)))
; 
; (define (pop stack)
;   (values (rest stack) (first stack)))  ; Returns the stack then the popped value
; 
; ;(pop (push (push '() "a") "b"))
; 
; (define (chunk lst size)  ; Chunk a list into groups of size - unsafe if not evenly divisible
;   (if (empty? lst)
;     '()
;     (cons (take lst size) (chunk (drop lst size) size))))
; 
; (define (parse lines)
;   (for/fold ([stacks '()]
;              [directives '()]
;              [on-directives #f]
;              #:result (values (reverse (drop stacks 1)) (reverse directives)))
;             ([line (in-list lines)])
;     (if (non-empty-string? line)
;       (if on-directives
;         (values stacks (cons line directives) on-directives)
;         (values (cons line stacks) directives on-directives))
;       ;(let ([split (string-split line " ")])
;       ;  (if (equal? (first (string->list line)) #\1)
;       ;    (values stacks '())
;       ;    (values (cons (string-split line " ") stacks) '())))
;       (values stacks directives #t))))
; 
; (define (parse-directive-strings directive-strings)
;   (for/fold ([directives '()]
;              #:result (reverse directives))
;             ([directive-string (in-list directive-strings)])
;     (cons (for/fold ([directive '()]
;                      [should-take #f]
;                      #:result (reverse directive))
;                     ([split-part (string-split directive-string " ")])
;       (if should-take
;         (values (cons (string->number split-part) directive) (not should-take))
;         (values directive (not should-take)))) directives)))
; 
; ; (define (parse-stack-strings stack-strings)
; ;   (for/fold ([stacks '()]
; ;              #:result (reverse stacks))
; ;             ([stack-string (in-list stack-strings)])
; ;     (cons (for/fold ([stack '()]
; ;                      #:result (reverse stack))
; ;                     ([i (in-range (string-length stack-string))])
; ;       (if (equal? (modulo i 4) 1)
; ;         (let ([stack-char (string-ref stack-string i)])
; ;           (if (equal? stack-char #\space)
; ;             (push null stack)
; ;             (push (string-ref stack-string i) stack)))
; ;         stack)) stacks)))
; (define (parse-stack-strings stack-strings)
;   (for/fold ([stacks '()]
;              #:result (reverse stacks))
;             ([i (in-range (string-length (first stack-strings)))])
;     (cons (for/fold ([stack '()]
;                      #:result (reverse stack))
;                     ([stack-string (in-list stack-strings)])
;       (if (equal? (modulo i 4) 1)
;         (let ([stack-char (string-ref stack-string i)])
;           (if (equal? stack-char #\space)
;             (push null stack)
;             (push (string-ref stack-string i) stack)))
;         stack)) stacks)))
; 
; 
; (define input-path "test_input.txt")
; (define lines (file->lines input-path))
; 
; (define-values (stack-strings directive-strings) (parse lines))
; (define directives (parse-directive-strings directive-strings))
; ;stack-strings
; (define stacks (filter (lambda (x) (not (empty? x))) (parse-stack-strings (reverse stack-strings))))
; ;directives
; stacks


(define input-path "test_input.txt")
(define lines (file->lines input-path))

(define stack-regexp (pregexp "(?:\\[(\\w)\\])|(    )"))
(define numbers-regexp (pregexp "(\\d+)\\s*$"))
(define empty-regexp (pregexp "^\\s*$"))
(define directive-regexp (pregexp "\\w+ (\\d+)"))
;testing
(regexp-match* stack-regexp "[Z]        " #:match-select second)
(regexp-match* stack-regexp "[Z] [M]    " #:match-select second)
(regexp-match* stack-regexp "[Z] [M] [P]" #:match-select second)
(regexp-match* stack-regexp "    [M] [P]" #:match-select second)
(regexp-match* stack-regexp "        [P]" #:match-select second)
(regexp-match* stack-regexp "[Z]     [P]" #:match-select second)
(regexp-match* numbers-regexp " 1   2   3 " #:match-select last)
(regexp-match empty-regexp "")
(regexp-match* directive-regexp "move 11 from 1 to 22" #:match-select second)
"XXXXXXX"
(for/fold ([stacks '()]
           [stack-count 0]
           [directives '()])
          ([line (in-list lines)])

  (if (non-empty-string? line)
    (let ([match (regexp-match* stack-regexp line #:match-select second)])
      (if (not (empty? match))
        (values (cons match stacks) stack-count directives)
        (let ([match (regexp-match* numbers-regexp line #:match-select last)])
          (if (not (empty? match))
            (values stacks (string->number (first match)) directives)
            (values stacks stack-count directives)))))
    (values stacks stack-count directives)))

