Dropping frames in glium
------------------------

This project demonstrates dropped frames in glium even with minimal draw code:

    > cargo run
    300 frames in 5.064391 seconds = 59.237 fps (estimated 3.9 frames dropped)
    300 frames in 5.060489 seconds = 59.283 fps (estimated 3.6 frames dropped)
    300 frames in 5.076766 seconds = 59.093 fps (estimated 4.6 frames dropped)

The system has no load outside of this test case but is reliably dropping
around one frame per second. These tests were run on a _MacBook Air_ with a
_Intel HD Graphics 3000_ graphics card.

This might be related to

- glium
- another library (glutin?)
- hardware (but other applications reliably get 60 FPS)
