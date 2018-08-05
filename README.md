# errors
A bunch of examples of errors I've encountered so far, to remind myself to not do them again / analyze them later

## Other annoyances

- Casts happen without any warning so `(2_isize - 3_isize) as usize` has the same effect as "overflowing", although technically not overflowing, since it's perfectly safe.
