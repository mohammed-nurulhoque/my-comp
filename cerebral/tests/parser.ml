type CharClass =
    | space | alpha | digit | alnum | upper | lower

type SetElem =
    | single int | range (int, int)

type Regex = 
    | zero '
    | literal int
    | class CharClass
    | set (List SetElim)
    | negset (List Setlim)
    | concat (regex, regex)
    | alt (regex, regex)
    | star regex
	| opt regex
    | plus regex

let partitions = {
	r1 r2 l 0 k => r1.match [] { none ' => k (none '
	                             some 0 => r2.match l k,
	                             some _ => panic("!")  },
	r1 r2 l n k => r1.match l[:n] $ {
		none '  => k (none '),
		some m1 => r2.match l[m1:] {
			some m2 => k (some (m1+m2)) $ { some t  => some t,
				                            none ' => partitions r1 r2 (m1-1) k },
			none '  => partitions r1 r2 l (m1-1) k }}}

let (t1 -> Option t2).or = k v1 v2 => {some tk v1 $ { (some t) => some t,
                                               (none ') => v2 }
let or =
	| (some x) _ => some x,
	| (none ') y => y

let Regex.match =
	| (zero ') [] k           => k (some 0),
	| (literal i) [i, ..] k   => k (some 1),
	| (class C)   l@[i, ..] k
	| (set C)     l@[i, ..] k
	| (negset C)  l@[i, ..] k => C.has i $ { true => k (some 1)
	                                       | false => k (none ') }

	| (concat (r1, r2)) l k   => partitions r1 r2 l (l.size ') k,
	| (alt (r1, r2)) l k      => r1.match l {
		| (some n) => (k (some n)) $or (r2.match l k)
		| (none ') => r2.match l k }

    | (opt r) l k             => r.match l {
		| (none ') => k (some 0)
		| (some n) => (k (some n)) $or (k (some 0)) }

	| r@(star r') l k         =>  r'.match l {
		(none ') => k (some 0),
		(some n) => r.match l[n:] {
			(none ') => panic "Unexpected!",
			(some 0) => k (some n) $or (k (some 0)),
			(some m) => k (some (n+m)) $or  (k (some n)) $or (k (some 0)) }}

	| (plus r') l k           => r'.match l {
		(none ')  => k (none '),
		(some n) => (star r).match l[n:] { some m => k (some (n+m)),
		                                   none ' => panic "Unexpected!" }
	}

let Regex.genNFA =
	| zero ' => DFA.null
	| set C => DFA.singleton C
	| literal c | class c => DFA.singleton c.toSet
	| concat (r1, r2) => r1.genNFA.seq r2.genNFA
	| alt (r1, r2) => r1.genNFA.par r2.genNFA
	| star r => 

let qsort = {
	| [] | [x] => ',
	| [x, ...L] = rec partit {
		[] => -1,
		[y] => y <= x $ { true => 0,
						  false => -1 },
		[y, ..L2] => 
			partit L2 $ n =>
			y <= x $ { true  => n+1,
					   false => L.swap(0, n+1); n }
		} L $ n =>
		qsort (L[0..n]);
		qsort (L[n+1..])
}