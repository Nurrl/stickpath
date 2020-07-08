pub fn resolve(
    head: Vec<char>,
    pipes: Vec<(usize, usize)>,
    mut tail: Vec<char>,
) -> (Vec<char>, Vec<char>) {
    /* Run trought in reverse so we just move
     * the tail pieces to match the head ones */
    for pipe in pipes.iter().rev() {
        tail.swap(pipe.0, pipe.1)
    }

    (head, tail)
}
