# errors
A bunch of examples of errors I've encountered so far, to remind myself to not do them again / analyze them later

## Other annoyances

- In unsafe code, overflow checking is disabled completely. This has led to a very nasty bug, and there is no reason to not panic on overflow, at least in debug mode. Even in C, overflow is UB, so there's no real benefit of silently overflowing, since it's an error to overflow in the first place.
