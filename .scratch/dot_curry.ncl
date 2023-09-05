let ident = "aaa" in
let record = {
    aaa.b=3,
    x2.b.c=2
} in
(.) record ident
# record . "a"
# record